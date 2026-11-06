# Research agenda for computational symplectic geometry

**Start with the [research proposal](report.html).** It maps seven distinct scientific questions, their motivation and progress, candidate directions, and concrete exchanges among tracks. It is intended to make the choice of future research legible; it does not assign or authorize a campaign.

- [Maintainable Markdown source](report.md) and [technical source](technical-notes.md).
- [Full technical agenda](technical-notes.html): candidate investigations, mathematical deductions, literature reconnaissance and detailed source qualifications.
- [Source ledger](source-ledger.md): exact sources, read depth and limits.
- [Pro return packet](pro-return/README.md): original returned files and integration checks.
- [Production record](production.md): prior reader feedback, what changed and validation.

To rebuild both standalone HTML files, run in `render/`:

```sh
npm ci --no-audit --no-fund
node build.cjs ../report.md ../report.html
node build.cjs ../technical-notes.md ../technical-notes.html
```
