"""Integrity and isolation regressions, not tests of writing quality."""
import argparse
import contextlib
import io
import json
import shutil
import tempfile
import unittest
from pathlib import Path
import dataset


class DatasetTests(unittest.TestCase):
    def test_source_integrity_and_extraction(self):
        dataset.validate(reextract=True)

    def test_detects_changed_source(self):
        with tempfile.TemporaryDirectory(prefix='writing-data-test-') as directory:
            original = dataset.LOCAL
            try:
                dataset.LOCAL = Path(directory) / 'local'
                shutil.copytree(original, dataset.LOCAL)
                with (dataset.LOCAL / 'inputs/c001.pdf').open('ab') as f:
                    f.write(b'\nchanged')
                with self.assertRaisesRegex(AssertionError, 'hash mismatch'):
                    dataset.validate()
            finally:
                dataset.LOCAL = original

    def test_export_excludes_labels_and_predictions(self):
        with tempfile.TemporaryDirectory(prefix='writing-data-test-') as directory:
            out = Path(directory) / 'inputs'
            dataset.export(argparse.Namespace(output=str(out), cases='c001,c002'))
            self.assertEqual({p.name for p in out.iterdir()}, {
                'c001.pdf', 'c001.txt', 'c001.jsonl', 'c002.pdf', 'c002.txt', 'c002.jsonl', 'index.json', 'README.txt'})
            for case in json.loads((out / 'index.json').read_text()):
                self.assertEqual(set(case), {'case_id', 'context', 'pdf', 'text', 'pages'})
            for line in (out / 'c001.jsonl').read_text().splitlines():
                self.assertEqual(set(json.loads(line)), {'case_id', 'pdf_page', 'start_char', 'end_char', 'text'})
            with self.assertRaises(FileExistsError):
                dataset.export(argparse.Namespace(output=str(out), cases='c001'))

    def test_shared_lineage_cannot_be_silent_training_data(self):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            dataset.partition(argparse.Namespace(holdout='c001'))
        partition = json.loads(output.getvalue())
        self.assertEqual(partition['development'], [])
        self.assertEqual(set(partition['excluded_shared_lineage']), {'c002','c003','c004','c005','c006','c007'})


if __name__ == '__main__':
    unittest.main()
