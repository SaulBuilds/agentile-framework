@sprint-2 @priority-high @backend
Feature: Prisma Database Schema
  As a backend developer
  I want a complete Prisma schema matching the data model
  So that the database structure is defined as code and ready for migration

  Scenario: All core entities are defined as Prisma models
    Given the data model specifies 20+ entities
    When the Prisma schema is written
    Then it includes models for users, wallets, pools, pool_rules, item_submissions, item_media, item_receipts, inventory_items, inventory_status_events, claims, claim_consumptions, shipments, tracking_events, qr_labels, courier_tasks, courier_events, disputes, dispute_evidence, warehouses, restricted_item_rules, operator_actions, notifications, and tax_reporting_profiles

  Scenario: Foreign key relationships are properly defined
    Given entities have relational dependencies
    When the schema defines relations
    Then wallets reference users
    And item_submissions reference users and pools
    And claims reference users, pools, and item_receipts
    And inventory_items reference item_receipts, warehouses, and pools
    And all foreign keys have appropriate cascade rules

  Scenario: Enums are defined for all status fields
    Given multiple entities use status enums
    When the schema defines enums
    Then SubmissionStatus covers draft through quarantined
    And InventoryStatus covers available through returned
    And ClaimStatus covers pending through disputed
    And all enum values match the data model specification

  Scenario: Schema validates against empty database
    Given a fresh PostgreSQL database
    When prisma migrate is run
    Then all tables are created without errors
    And all indexes are applied
    And all enum types are registered
