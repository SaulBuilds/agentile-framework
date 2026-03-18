// Shared type definitions for Gradient Barter Protocol
// These types are shared between the API and Web packages

export interface ApiResponse<T> {
  data: T;
  meta?: PaginationMeta;
  error?: ApiError;
}

export interface PaginationMeta {
  total: number;
  page: number;
  limit: number;
  pages: number;
}

export interface ApiError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
}

// Pool taxonomy
export enum PoolHue {
  GREEN = 'GREEN',
  BLUE = 'BLUE',
  AMBER = 'AMBER',
  VIOLET = 'VIOLET',
  TEAL = 'TEAL',
  RED = 'RED',
}

export enum QualityTier {
  NEW = 'NEW',
  REFURBISHED = 'REFURBISHED',
  USED_GOOD = 'USED_GOOD',
  COLLECTIBLE = 'COLLECTIBLE',
}

// Submission statuses
export enum SubmissionStatus {
  DRAFT = 'DRAFT',
  SUBMITTED = 'SUBMITTED',
  IN_TRANSIT = 'IN_TRANSIT',
  RECEIVED = 'RECEIVED',
  GRADED = 'GRADED',
  ACCEPTED = 'ACCEPTED',
  REJECTED = 'REJECTED',
  QUARANTINED = 'QUARANTINED',
}

// Inventory statuses
export enum InventoryStatus {
  AVAILABLE = 'AVAILABLE',
  RESERVED = 'RESERVED',
  OUTBOUND = 'OUTBOUND',
  DELIVERED = 'DELIVERED',
  DISPUTED = 'DISPUTED',
  QUARANTINED = 'QUARANTINED',
}

// Claim statuses
export enum ClaimStatus {
  PENDING = 'PENDING',
  ACTIVE = 'ACTIVE',
  CONSUMED = 'CONSUMED',
  EXPIRED = 'EXPIRED',
  DISPUTED = 'DISPUTED',
}

// User roles
export enum UserRole {
  CONTRIBUTOR = 'CONTRIBUTOR',
  OPERATOR = 'OPERATOR',
  GRADER = 'GRADER',
  COURIER = 'COURIER',
  ADMIN = 'ADMIN',
  SUPER_ADMIN = 'SUPER_ADMIN',
}
