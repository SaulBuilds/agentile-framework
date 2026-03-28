# Audits Directory

> Audit reports are created in dated directories and are **never modified after creation**.
> Corrections go in new audit files. See Rule 6 in CORE_RULES.md.

## Directory Structure

```
audits/
├── README.md                          # This file
├── YYYY-MM-DD-audit-name/
│   ├── 01_EXECUTIVE_SUMMARY.md        # High-level findings
│   ├── 02_FINDINGS.md                 # Detailed findings
│   ├── 03_RECOMMENDATIONS.md         # Action items
│   └── 04_RECONCILIATION.md          # Status of prior findings (if applicable)
└── YYYY-MM-DD-follow-up/
    └── ...
```

## Rules

1. **Dated directories**: Every audit uses `YYYY-MM-DD-name` format
2. **Immutable**: Once created, audit files are never edited
3. **Corrections**: If a finding was wrong, create a new audit file with the correction
4. **Reconciliation**: Each new audit should cross-reference prior findings and their status
5. **Numbered files**: Use sequential numbering (01_, 02_, etc.) for consistent ordering

## Creating an Audit

1. Create the directory: `audits/YYYY-MM-DD-<name>/`
2. Write findings using the numbered file convention
3. Cross-reference any prior audit findings
4. Commit the audit files
5. Never edit these files again
