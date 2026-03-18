# Item Policy & Restricted Goods Matrix

## Allowed Categories (v1)

Items in these categories are accepted for submission, subject to condition and band requirements:

| Category | Hue | Notes |
|----------|-----|-------|
| Power tools | Green | Exclude tools with prohibited battery conditions |
| Hand tools | Green | Standard hand tools, no bladed weapons |
| Home goods | Green | Decor, kitchen items, storage |
| Small appliances | Teal | Coffee makers, blenders, vacuums, lamps |
| Non-luxury sneakers | Amber | No items requiring luxury authentication |
| Casual apparel | Amber | Jackets, bags, everyday clothing |
| Books / media bundles | Blue (or Violet) | Bundled sets preferred for value matching |
| Gaming consoles & accessories | Blue | Controllers, headsets, consoles |
| Consumer electronics | Blue | Headphones, tablets, smart speakers |
| Collectibles (bounded risk) | Violet | Trading cards, vinyl, hobby kits, figures |

## Excluded Categories (v1)

These items are **blocked at submission**. The submission flow will reject them with a clear reason.

| Category | Reason |
|----------|--------|
| Firearms / weapon parts | Legal, safety |
| Controlled substances | Legal |
| Perishable goods | Spoilage, liability |
| Recalled products | Consumer safety (CPSC screening) |
| Mattresses / hygiene-sensitive goods | Health regulations |
| Damaged lithium battery devices | Fire hazard without specialized handling |
| Stolen goods / serial-blacklisted items | Legal, trust |
| Medical devices requiring regulated handling | Regulatory compliance |
| High-value luxury items | Authentication ops not mature for v1 |
| Vehicles / vehicle parts | Logistics, legal complexity |
| Hazardous materials | Safety, shipping restrictions |

## Restricted Items (Manual Review Required)

Items that may fall into gray areas are flagged for operator review before intake proceeds:

| Pattern | Rule Type | Handling |
|---------|-----------|----------|
| Items matching known recall patterns | `recall` | Auto-block, log source |
| Items with serial numbers requiring verification | `manual_review` | Operator checks serial against blacklist |
| Items in ambiguous categories | `manual_review` | Operator classifies or rejects |
| Items declared at Band 5 ($500+) | `manual_review` | Requires operator valuation confirmation |
| Items with batteries (non-damaged) | `manual_review` | Operator inspects battery condition |

## Recall Screening

- Submissions are checked against a recall database (CPSC or equivalent)
- Matching items are auto-blocked
- Recall checks also run at warehouse intake (physical verification)
- Recall database is updated periodically by compliance team

## Pre-Submission Declaration

Users must declare before submitting:
- [ ] Item is not stolen or under lien
- [ ] Item is not under recall (to the best of their knowledge)
- [ ] Item does not contain hazardous materials
- [ ] Item meets the packaging standards for its category
- [ ] User has the right to contribute this item

False declarations may result in account restrictions.

## Warehouse Rejection Rights

Even if a submission passes automated screening, the warehouse operator has final authority to:
- Reject items that don't match the submission description
- Quarantine items that appear unsafe, counterfeit, or misrepresented
- Reclassify items to a different pool band based on physical inspection

## Policy Update Process

1. Compliance team identifies a new category concern
2. Admin creates or updates a restricted item rule
3. Rule specifies: category pattern, rule type (blocked/manual_review/recall), description
4. All changes are audit-logged
5. Existing submissions in the pipeline are re-screened against updated rules
