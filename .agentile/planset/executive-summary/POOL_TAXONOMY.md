# Pool Taxonomy & Category Matrix

## Pool Family Definition

Every pool is defined by four dimensions:

| Dimension | Description | Example |
|-----------|-------------|---------|
| **Hue** | Category / service domain | Blue (electronics) |
| **Gradient Band** | Value range | Band 3 ($75–$200) |
| **Quality Tier** | Item condition | Used-Good |
| **Region** | Geographic / warehouse scope | Pacific Northwest |

## Hue Categories (v1)

| Hue | Color Code | Domain | Example Items |
|-----|------------|--------|---------------|
| Green | `#496A4C` | Tools & Home Utility | Power drills, hand tools, garden equipment |
| Blue | `#4A6FA5` | Electronics | Gaming consoles, headphones, tablets, smart speakers |
| Amber | `#C8944F` | Apparel & Footwear | Non-luxury sneakers, jackets, bags (no high-fashion) |
| Violet | `#7B5EA7` | Collectibles | Trading cards, vinyl, hobby kits, figures |
| Teal | `#4A8B8B` | Household & Small Appliances | Coffee makers, blenders, vacuums, lamps |
| Red | `#A54A4A` | Restricted / Manual Review | Internal only — not publicly claimable. Items requiring operator judgment. |

## Value Bands

| Band | Range | Typical Items |
|------|-------|---------------|
| 1 | $10–$25 | Books, basic tools, accessories, media bundles |
| 2 | $25–$75 | Small appliances, used sneakers, controllers, board games |
| 3 | $75–$200 | Power tools, headphones, small electronics, quality apparel |
| 4 | $200–$500 | Gaming consoles, tablets, premium tools, collectible sets |
| 5 | $500+ | High-end electronics, rare collectibles (requires Band 5 KYC) |

**Note**: Band 5 requires identity verification (KYC) due to higher value and regulatory considerations.

## Quality Tiers

| Tier | Code | Criteria |
|------|------|----------|
| New | `new` | Sealed, unused, original packaging |
| Refurbished | `refurbished` | Professionally restored, tested, may have non-original packaging |
| Used-Good | `used_good` | Functional, cosmetic wear acceptable, all essential parts present |
| Collectible Graded | `collectible` | Graded by recognized standard (PSA, BGS, etc.) or operator assessment |

## Regional Scoping

Pools may be scoped to regions to manage logistics costs and warehouse capacity:
- Region maps to warehouse service areas
- An item submitted in Region A creates a claim for Region A pools
- Cross-region claims are not supported in v1

## Classification Flow

```
User estimates category + band
  → Submission created with provisional pool assignment
  → Item ships to regional warehouse
  → Warehouse operator grades item:
      - Confirms or adjusts hue
      - Confirms or adjusts band
      - Assigns quality tier
      - Records evidence
  → Final pool assignment = (hue, band, tier, region)
  → Claim minted for the final pool
```

## Pool Saturation Rules

- Each pool has a configurable max capacity
- When a pool reaches capacity, it is marked `saturated`
- Saturated pools stop accepting new submissions
- Existing claims in saturated pools remain valid
- Admins can increase capacity or create new regional pools to relieve saturation

## v1 Launch Configuration

| Parameter | Value |
|-----------|-------|
| Active hues | Green, Blue, Amber, Teal (4 of 6) |
| Active bands | 1, 2, 3 (skip 4-5 until ops mature) |
| Active tiers | New, Used-Good (skip Refurbished, Collectible initially) |
| Regions | 1 pilot region |
| Pools at launch | ~8-12 (4 hues × 3 bands, not all tiers) |
