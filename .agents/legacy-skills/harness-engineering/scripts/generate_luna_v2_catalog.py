#!/usr/bin/env python3
"""Generate a current static Codex catalog with Luna enabled for MultiAgent V2."""

import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def run(*args: str, env: dict[str, str] | None = None) -> str:
    return subprocess.run(
        args,
        check=True,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    ).stdout


def atomic_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        mode="w", dir=path.parent, prefix=f".{path.name}.", delete=False
    ) as handle:
        json.dump(value, handle, indent=2)
        handle.write("\n")
        temporary = Path(handle.name)
    os.replace(temporary, path)


def main() -> int:
    codex_home = Path(
        os.environ.get("CODEX_HOME", Path.home() / ".codex")
    ).resolve()
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output",
        type=Path,
        default=codex_home / "model-catalog-luna-v2.json",
    )
    args = parser.parse_args()
    output = args.output.resolve()

    codex = shutil.which("codex")
    if codex is None:
        raise SystemExit("codex is not on PATH")
    auth = codex_home / "auth.json"
    if not auth.is_file():
        raise SystemExit(f"missing Codex authentication file: {auth}")

    with tempfile.TemporaryDirectory(prefix="codex-live-catalog-") as temporary:
        isolated_home = Path(temporary)
        (isolated_home / "auth.json").symlink_to(auth)
        isolated_env = os.environ.copy()
        isolated_env["CODEX_HOME"] = str(isolated_home)
        source_text = run(codex, "debug", "models", env=isolated_env)
        source = json.loads(source_text)
        cache_path = isolated_home / "models_cache.json"
        cache = json.loads(cache_path.read_text()) if cache_path.is_file() else {}

    models = source.get("models")
    if not isinstance(models, list) or not models:
        raise SystemExit("live catalog did not contain a non-empty models list")
    luna = [model for model in models if model.get("slug") == "gpt-5.6-luna"]
    if len(luna) != 1:
        raise SystemExit(f"expected exactly one gpt-5.6-luna entry, found {len(luna)}")

    source_multi_agent_version = luna[0].get("multi_agent_version")
    if source_multi_agent_version not in {"v1", "v2"}:
        raise SystemExit(
            "refusing unexpected Luna multi_agent_version "
            f"{source_multi_agent_version!r}"
        )
    luna[0]["multi_agent_version"] = "v2"

    atomic_json(output, source)
    validation = json.loads(
        run(
            codex,
            "-c",
            f"model_catalog_json={json.dumps(str(output))}",
            "debug",
            "models",
        )
    )
    validated_models = validation.get("models", [])
    validated_luna = [
        model for model in validated_models if model.get("slug") == "gpt-5.6-luna"
    ]
    if len(validated_models) != len(models) or len(validated_luna) != 1:
        raise SystemExit("generated catalog did not round-trip through Codex")
    if validated_luna[0].get("multi_agent_version") != "v2":
        raise SystemExit("generated catalog did not enable Luna for MultiAgent V2")

    source_hash = hashlib.sha256(source_text.encode()).hexdigest()
    metadata = {
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
        "codex_version": run(codex, "--version").strip(),
        "source_etag": cache.get("etag"),
        "source_sha256": source_hash,
        "transformation": {
            "model": "gpt-5.6-luna",
            "field": "multi_agent_version",
            "from": source_multi_agent_version,
            "to": "v2",
        },
    }
    atomic_json(output.with_suffix(output.suffix + ".meta.json"), metadata)
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
