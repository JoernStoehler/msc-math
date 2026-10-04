#!/usr/bin/env python3

import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock


SCRIPT = Path(__file__).with_name("artifacts.py")
SPEC = importlib.util.spec_from_file_location("msc_math_artifacts", SCRIPT)
assert SPEC and SPEC.loader
artifacts = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(artifacts)


class ArtifactTests(unittest.TestCase):
    def test_default_cache_uses_xdg_cache_home(self) -> None:
        with mock.patch.dict(
            artifacts.os.environ,
            {"XDG_CACHE_HOME": "/environment/cache"},
            clear=True,
        ):
            self.assertEqual(
                artifacts.default_cache_root(),
                Path("/environment/cache/msc-math/artifacts"),
            )

    def test_default_cache_falls_back_to_home_cache(self) -> None:
        with (
            mock.patch.object(
                artifacts.Path,
                "home",
                return_value=Path("/environment/home"),
            ),
            mock.patch.dict(
                artifacts.os.environ,
                {"XDG_CACHE_HOME": "relative-is-invalid"},
                clear=True,
            ),
        ):
            self.assertEqual(
                artifacts.default_cache_root(),
                Path("/environment/home/.cache/msc-math/artifacts"),
            )

    def test_explicit_cache_root_overrides_xdg_default(self) -> None:
        with mock.patch.dict(
            artifacts.os.environ,
            {
                "MSC_MATH_CACHE_ROOT": "/explicit/cache",
                "XDG_CACHE_HOME": "/environment/cache",
            },
            clear=True,
        ):
            self.assertEqual(
                artifacts.default_cache_root(),
                Path("/explicit/cache"),
            )

    def test_snapshot_identity_is_order_independent(self) -> None:
        first = {"path": "a", "sha256": "0" * 64, "size": 1}
        second = {"path": "nested/b", "sha256": "f" * 64, "size": 2}
        self.assertEqual(
            artifacts.snapshot_id("example", [first, second]),
            artifacts.snapshot_id("example", [second, first]),
        )

    def test_relative_path_rejects_escape(self) -> None:
        for value in ("", "/absolute", "../escape", "a/../escape", "./local"):
            with self.subTest(value=value), self.assertRaises(artifacts.ArtifactError):
                artifacts.relative_path(value)

    def test_directory_records_are_sorted_and_hashed(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            (root / "nested").mkdir()
            (root / "z").write_bytes(b"last")
            (root / "nested" / "a").write_bytes(b"first")
            records = artifacts.file_records(root)
        self.assertEqual([record["path"] for record in records], ["nested/a", "z"])
        self.assertEqual(records[0]["size"], 5)
        self.assertEqual(
            records[0]["sha256"],
            "a7937b64b8caa58f03721bb6bacf5c78cb235febe0e70b1b84cd99541461a08e",
        )

    def test_manifest_snapshot_must_match_inventory(self) -> None:
        files = [{"path": "data.jsonl", "sha256": "a" * 64, "size": 12}]
        snapshot = artifacts.snapshot_id("example", files)
        raw = artifacts.canonical_json(
            {
                "artifact": "example",
                "files": files,
                "schema_version": 1,
                "snapshot": snapshot,
            }
        )
        parsed = artifacts.parse_manifest(raw, "example", snapshot)
        self.assertEqual(parsed["files"], files)
        tampered = json.loads(raw)
        tampered["files"][0]["size"] = 13
        with self.assertRaisesRegex(artifacts.ArtifactError, "digest"):
            artifacts.parse_manifest(
                artifacts.canonical_json(tampered), "example", snapshot
            )

    def test_install_links_refuses_existing_file(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            files = root / "cache"
            files.mkdir()
            (files / "source").write_text("data")
            (root / "target").write_text("user data")
            with mock.patch.object(artifacts, "REPO_ROOT", root):
                with self.assertRaisesRegex(artifacts.ArtifactError, "existing path"):
                    artifacts.install_links(
                        {"links": {"source": "target"}}, files
                    )

    def test_verify_cache_is_offline_and_detects_corruption(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            staging = root / "staging"
            staging.mkdir()
            (staging / "data.jsonl").write_bytes(b"{\"value\": 1}\n")
            records = artifacts.file_records(staging)
            snapshot = artifacts.snapshot_id("example", records)
            directory = root / "example" / snapshot
            directory.mkdir(parents=True)
            files = directory / "files"
            staging.rename(files)
            manifest = artifacts.identity("example", records) | {"snapshot": snapshot}
            manifest_path = directory / "manifest.json"
            manifest_path.write_bytes(artifacts.canonical_json(manifest))
            registry = {"artifacts": {"example": {"snapshot": snapshot, "links": {"data.jsonl": "unused"}}}}
            args = artifacts.parser().parse_args(["verify-cache", "example", "--cache-root", raw])
            before = artifacts.file_records(directory)
            with (
                mock.patch.object(artifacts, "load_registry", return_value=registry),
                mock.patch.object(artifacts, "run_rclone", side_effect=AssertionError("network access")),
                mock.patch.object(artifacts, "install_links", side_effect=AssertionError("link mutation")),
                mock.patch("sys.stdout", new_callable=io.StringIO) as output,
            ):
                args.function(args)
                result = json.loads(output.getvalue())
                self.assertEqual(result["status"], "verified-local-cache")
                self.assertFalse(result["remote_checked"])
                self.assertEqual(result["files"], 1)
                self.assertEqual(artifacts.file_records(directory), before)
                (files / "data.jsonl").write_bytes(b"{\"value\": 2}\n")
                with self.assertRaisesRegex(artifacts.ArtifactError, "size/hash"):
                    args.function(args)
                (files / "data.jsonl").write_bytes(b"{\"value\": 1}\n")
                (files / "extra").write_text("unexpected")
                with self.assertRaisesRegex(artifacts.ArtifactError, "inventory"):
                    args.function(args)
                (files / "extra").unlink()
                manifest["files"][0]["size"] += 1
                manifest_path.write_bytes(artifacts.canonical_json(manifest))
                with self.assertRaisesRegex(artifacts.ArtifactError, "digest"):
                    args.function(args)
                manifest_path.unlink()
                with self.assertRaisesRegex(artifacts.ArtifactError, "cached manifest"):
                    args.function(args)


if __name__ == "__main__":
    unittest.main()
