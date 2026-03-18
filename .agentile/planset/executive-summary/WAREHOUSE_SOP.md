# Warehouse Standard Operating Procedures — v1

## Overview

This document defines the standard operating procedures for Gradient Barter Protocol warehouse facilities. All warehouse operators must follow these procedures to maintain chain of custody, item integrity, and claim accuracy.

---

## 1. Facility Requirements

### Physical Layout
- **Receiving area**: Sealed intake zone with CCTV coverage
- **Grading station**: Well-lit workstation with camera for evidence photos
- **General storage**: Organized bins with labeled locations
- **High-value cage**: Controlled-access area for Band 4-5 items, locked, dual-key
- **Quarantine lane**: Separate area for flagged items, restricted access
- **Outbound staging**: Packing and shipping prep area

### Security
- CCTV coverage on: receiving area, grading station, high-value cage, exits
- Camera footage retained per data retention policy (minimum 90 days)
- Access control on high-value cage (authorized operators only)
- Visitor log for non-staff

---

## 2. Inbound Receiving

### Step 1: Package Arrival
1. Log package arrival (timestamp, carrier, tracking number)
2. Inspect external packaging for damage — photograph if damaged
3. Open package in receiving area under CCTV

### Step 2: QR Scan
1. Scan QR label on item
2. System loads submission record: item details, user photos, estimated band, category
3. If QR is unreadable, use human-readable fallback code
4. If no label found, set aside as unidentified — notify admin

### Step 3: Intake Verification
1. Compare physical item against submission photos and description
2. Check item against restricted goods matrix
3. Verify serial number (if applicable) against blacklist database

**Outcome**: Item proceeds to grading OR is flagged for manual review/rejection.

---

## 3. Grading

### Standard Grading Checklist
- [ ] Item matches submission description
- [ ] Item is in the declared category
- [ ] Item is functional (power on test for electronics)
- [ ] Cosmetic condition assessed (scratches, dents, wear)
- [ ] All essential parts/accessories present
- [ ] No safety hazards identified
- [ ] Serial number recorded (if applicable)

### Evidence Capture
- Minimum 4 photos: front, back, detail of any defects, serial/label
- Grading notes: free-text describing condition
- Defects log: itemized list of issues found

### Band Assignment
1. Compare item against band criteria for its category
2. Assign final band (may differ from user's estimate)
3. If reclassified, record reason

### Quality Tier Assignment
- **New**: Sealed, unused, original packaging confirmed
- **Refurbished**: Professionally restored, tested, documented
- **Used-Good**: Functional, cosmetic wear within acceptable range
- **Collectible**: Grading standard applied (PSA, BGS, or operator assessment)

### Grading Decisions

| Decision | Criteria | Action |
|----------|----------|--------|
| **Accept** | Item meets pool standards | Assign bin, emit intake event, activate claim |
| **Reject** | Item doesn't meet standards, misrepresented, or excluded category | Record reason + evidence, notify contributor, no claim |
| **Quarantine** | Safety concern, suspected counterfeit, or requires further investigation | Move to quarantine lane, admin alert, no claim until resolved |

---

## 4. Storage & Bin Assignment

### Bin Naming Convention
```
[Warehouse]-[Hue]-[Band]-[Row]-[Slot]
Example: PDX-BLU-B3-R04-S12
```

### Storage Rules
- Items stored by pool (hue + band) for efficient picking
- High-value items (Band 4-5) in controlled-access cage
- Fragile items marked with handling instructions
- Maximum dwell time alert at 30 days — admin notified for items not claimed

---

## 5. Quarantine Procedures

### When to Quarantine
- Item appears unsafe (damaged batteries, sharp edges, chemical smell)
- Item appears counterfeit
- Serial number matches blacklist
- Item matches recall database
- Significant discrepancy between submission and physical item

### Quarantine Process
1. Move item to quarantine lane immediately
2. Record mandatory reason with photos
3. System blocks claim activation
4. Admin receives alert (high severity for safety items)
5. Admin reviews and decides: release to pool, reject and return, or destroy

### Resolution
- **Release**: Item cleared, moved to storage, claim activated
- **Reject**: Item returned to contributor (contributor pays return shipping)
- **Destroy**: Item disposed of per safety procedures (hazardous items)

---

## 6. Outbound Processing

### Picking
1. Reservation triggers pick task
2. Operator locates item by bin location
3. Operator scans item QR to confirm correct item
4. Item moved to outbound staging

### Packing
1. Pack item per category-specific packing standards
2. Include packing slip with order details
3. Attach shipping label
4. Photograph packed item

### Handoff
1. Carrier picks up or item dropped at carrier location
2. Scan event recorded (proof of handoff)
3. Tracking number linked to shipment record

---

## 7. High-Value Protocol (Band 4-5)

- **Dual control**: Two operators required for receiving, grading, and picking
- **Enhanced evidence**: Minimum 8 photos, video walkthrough recommended
- **Cage storage**: Locked cage with access log
- **Cycle counting**: Weekly count of high-value cage items
- **Escalation**: Any discrepancy reported to admin within 1 hour

---

## 8. Employee Role Separation

| Role | Responsibilities | Cannot Do |
|------|-----------------|-----------|
| **Receiver** | Package intake, QR scan, initial inspection | Grade items, assign bins |
| **Grader** | Condition assessment, band assignment, evidence capture | Pick outbound items |
| **Picker** | Locate and retrieve reserved items | Grade or receive items |
| **Supervisor** | Quarantine decisions, cage access, escalation | Override without documentation |

When staffing is limited (1-2 operators), the same person may perform multiple roles but each action must be independently logged.

---

## 9. Incident Procedures

### Missing Item
1. Check bin location and surrounding slots
2. Review CCTV for last known handling
3. Report to admin within 4 hours
4. If unresolved: log as shrinkage, contributor's claim remains valid

### Damaged Item (in warehouse)
1. Photograph damage
2. Move to quarantine
3. Assess if damage occurred pre- or post-receipt
4. If post-receipt: warehouse liability, resolve per insurance

### Theft Suspicion
1. Do not confront — escalate to supervisor immediately
2. Preserve CCTV footage
3. Lock down affected area
4. File incident report
5. Notify admin and legal

---

## 10. Metrics & Reporting

Warehouse operators track daily:
- Items received
- Items graded (accepted / rejected / quarantined)
- Items picked and shipped
- Quarantine queue length
- High-value cage count

Weekly reports include:
- Shrinkage rate
- Average grading time
- Quarantine resolution time
- Any incidents
