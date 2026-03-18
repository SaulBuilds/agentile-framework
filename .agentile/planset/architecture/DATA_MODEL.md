# DATA_MODEL.md — Gradient Barter Protocol

## Core Entities and Relationships

```
users ──────┬──── wallets
            │
            ├──── item_submissions ──── item_media
            │         │
            │         ├──── item_receipts (on-chain NFT reference)
            │         │         │
            │         │         └──── inventory_items ──── inventory_status_events
            │         │                     │
            │         │                     └──── qr_labels
            │         │
            │         └──── shipments ──── tracking_events
            │
            ├──── claims ──── claim_consumptions
            │
            ├──── courier_tasks ──── courier_events
            │
            └──── disputes ──── dispute_evidence

pools ──── pool_rules
  │
  └──── pricing_band_snapshots

warehouses (standalone, referenced by inventory_items)

restricted_item_rules (standalone, referenced by submission screening)
buy_on_site_programs (standalone, referenced by warehouse intake)
operator_actions (audit log, references operators + target entities)
notifications (references users + source events)
tax_reporting_profiles (references users)
```

## Entity Definitions

### users
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| email | string | unique, nullable |
| phone | string | unique, nullable |
| display_name | string | |
| kyc_status | enum | none, pending, approved, rejected |
| kyc_verified_at | timestamp | nullable |
| role | enum | contributor, operator, grader, courier, admin, super_admin |
| created_at | timestamp | |
| updated_at | timestamp | |

### wallets
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| user_id | uuid | FK → users |
| address | string | EVM address, unique |
| chain_id | int | |
| verified | boolean | nonce signature verified |
| linked_at | timestamp | |

### pools
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| hue | enum | green, blue, amber, violet, teal, red |
| band | int | 1-5 (value band) |
| region | string | geographic scope |
| quality_tier | enum | new, refurbished, used_good, collectible |
| on_chain_pool_id | string | smart contract reference |
| status | enum | active, paused, saturated |
| created_at | timestamp | |

### pool_rules
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| pool_id | uuid | FK → pools |
| rule_type | string | e.g., transfer_policy, max_items, holding_period |
| rule_value | json | flexible rule configuration |
| active | boolean | |

### item_submissions
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| user_id | uuid | FK → users |
| category | string | |
| estimated_band | int | user's estimate |
| condition_description | text | |
| status | enum | draft, submitted, in_transit, received, graded, accepted, rejected, quarantined |
| target_pool_id | uuid | FK → pools, nullable (assigned after grading) |
| submission_hash | string | for on-chain reference |
| created_at | timestamp | |
| updated_at | timestamp | |

### item_media
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| submission_id | uuid | FK → item_submissions |
| media_type | enum | photo, video |
| s3_key | string | object storage reference |
| uploaded_at | timestamp | |

### item_receipts
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| submission_id | uuid | FK → item_submissions |
| on_chain_token_id | string | ERC-721 token ID |
| on_chain_tx_hash | string | minting transaction |
| pool_id | uuid | FK → pools |
| final_band | int | warehouse-assigned band |
| condition_grade | string | |
| grader_id | uuid | FK → users (grader) |
| grading_notes | text | |
| evidence_hash | string | CID or hash of grading evidence |
| restrictions_flags | json | any restriction annotations |
| created_at | timestamp | |

### inventory_items
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| receipt_id | uuid | FK → item_receipts |
| warehouse_id | uuid | FK → warehouses |
| pool_id | uuid | FK → pools |
| bin_location | string | physical location in warehouse |
| status | enum | available, reserved, outbound, delivered, disputed, quarantined, liquidated, destroyed, returned |
| reserved_by | uuid | FK → users, nullable |
| reserved_at | timestamp | nullable |
| reservation_expires | timestamp | nullable |

### inventory_status_events
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| inventory_id | uuid | FK → inventory_items |
| previous_status | enum | |
| new_status | enum | |
| actor_id | uuid | FK → users |
| evidence_hash | string | nullable |
| on_chain_anchor | string | nullable, tx hash |
| created_at | timestamp | |

### claims
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| user_id | uuid | FK → users |
| pool_id | uuid | FK → pools |
| receipt_id | uuid | FK → item_receipts (source contribution) |
| on_chain_token_id | string | ERC-1155 token reference |
| status | enum | pending, active, consumed, expired, disputed |
| activated_at | timestamp | nullable |
| consumed_at | timestamp | nullable |

### claim_consumptions
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| claim_id | uuid | FK → claims |
| inventory_id | uuid | FK → inventory_items (item withdrawn) |
| consumed_at | timestamp | |
| on_chain_tx_hash | string | |

### shipments
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| submission_id | uuid | FK → item_submissions, nullable (inbound) |
| inventory_id | uuid | FK → inventory_items, nullable (outbound) |
| direction | enum | inbound, outbound |
| carrier | string | |
| tracking_number | string | |
| status | enum | created, picked_up, in_transit, delivered, exception |
| estimated_delivery | timestamp | |
| delivered_at | timestamp | nullable |
| proof_hash | string | nullable |

### tracking_events
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| shipment_id | uuid | FK → shipments |
| event_type | string | |
| location | string | nullable |
| timestamp | timestamp | |
| raw_data | json | carrier event payload |

### qr_labels
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| submission_id | uuid | FK → item_submissions |
| qr_data | string | encoded QR content |
| human_readable_code | string | fallback code |
| label_pdf_key | string | S3 key for PDF |
| generated_at | timestamp | |

### courier_tasks
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| shipment_id | uuid | FK → shipments |
| courier_id | uuid | FK → users, nullable (unassigned) |
| pickup_location | json | address + coordinates |
| dropoff_location | json | address + coordinates |
| fee | decimal | |
| tip | decimal | nullable |
| status | enum | posted, accepted, pickup_verified, in_transit, delivered, completed, failed, disputed |
| time_window_start | timestamp | |
| time_window_end | timestamp | |
| created_at | timestamp | |

### courier_events
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| task_id | uuid | FK → courier_tasks |
| event_type | enum | accepted, pickup_scan, milestone, dropoff_proof, completed, exception |
| proof_hash | string | nullable |
| location | json | nullable |
| created_at | timestamp | |

### disputes
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| opened_by | uuid | FK → users |
| object_type | enum | submission, receipt, inventory, claim, shipment, courier_task |
| object_id | uuid | polymorphic FK |
| reason | text | |
| status | enum | open, under_review, resolved, denied |
| resolution | enum | refund_claim, replacement, deny, goodwill_credit, nullable |
| sla_deadline | timestamp | |
| resolved_at | timestamp | nullable |
| resolved_by | uuid | FK → users, nullable |
| created_at | timestamp | |

### dispute_evidence
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| dispute_id | uuid | FK → disputes |
| submitted_by | uuid | FK → users |
| evidence_type | enum | photo, video, text, document |
| s3_key | string | nullable |
| content | text | nullable (for text evidence) |
| cid_hash | string | nullable |
| submitted_at | timestamp | |

### warehouses
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| name | string | |
| region | string | |
| address | json | |
| status | enum | active, paused, decommissioned |
| capacity | int | max items |
| current_count | int | current items stored |

### restricted_item_rules
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| category_pattern | string | matches against submission category |
| rule_type | enum | blocked, manual_review, recall |
| description | text | |
| active | boolean | |
| source | string | e.g., CPSC recall DB, internal policy |

### operator_actions
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| operator_id | uuid | FK → users |
| action_type | string | |
| target_type | string | entity type affected |
| target_id | uuid | entity ID affected |
| reason | text | mandatory |
| metadata | json | |
| created_at | timestamp | |

### notifications
| Column | Type | Notes |
|--------|------|-------|
| id | uuid | PK |
| user_id | uuid | FK → users |
| channel | enum | email, sms, push |
| event_type | string | |
| content | json | |
| sent_at | timestamp | nullable |
| read_at | timestamp | nullable |

## Modeling Rules

1. Every external physical event (scan, grade, pickup, delivery) must have:
   - A canonical DB row
   - A signed actor (user ID)
   - Timestamp
   - Evidence hash (nullable for low-risk events)
   - Optional on-chain anchor hash

2. Status transitions use event-sourced pattern via `*_status_events` / `*_events` tables — current status is the latest event.

3. All monetary amounts stored as decimal (not float).

4. All timestamps are UTC.

5. Polymorphic references (disputes.object_type + object_id) are acceptable for the dispute system but should not proliferate elsewhere.
