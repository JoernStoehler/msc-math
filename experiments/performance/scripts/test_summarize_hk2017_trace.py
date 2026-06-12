import unittest

import summarize_hk2017_trace as trace_summary


class Hk2017TraceSummaryTests(unittest.TestCase):
    def test_emitted_by_len_vectors_are_summed(self):
        rows = [
            {
                "target": "hk2017-cycle-enumeration",
                "event": "hk2017_directed_cycle_summary",
                "facet_count": 6,
                "emitted_by_len": [0, 0, 1, 2],
                "emitted_sigmas": 3.0,
            },
            {
                "target": "hk2017-cycle-enumeration",
                "event": "hk2017_directed_cycle_summary",
                "facet_count": 6,
                "emitted_by_len": [0, 0, 4, 1, 1],
                "emitted_sigmas": 6.0,
            },
        ]

        summary = trace_summary.summarize(rows)

        self.assertEqual(len(summary), 1)
        self.assertEqual(summary[0]["emitted_by_len_sum"], [0, 0, 5, 3, 1])
        self.assertEqual(summary[0]["emitted_sigmas_mean"], 4.5)

    def test_trace_line_parser_keeps_emitted_by_len(self):
        line = (
            'INFO performance_sample: hk2017_directed_cycle_summary '
            'facet_count=6 dfs_prefix_count=15 emitted_sigmas=2 '
            'emitted_by_len=[0, 0, 1, 0, 1] '
            'target="hk2017-cycle-enumeration" sample=0'
        )

        fields = dict(trace_summary.FIELD_RE.findall(line))
        vector_fields = dict(trace_summary.VECTOR_FIELD_RE.findall(line))

        self.assertEqual(
            trace_summary.parse_int_vector(vector_fields["emitted_by_len"]),
            [0, 0, 1, 0, 1],
        )
        self.assertEqual(float(fields["emitted_sigmas"]), 2.0)


if __name__ == "__main__":
    unittest.main()
