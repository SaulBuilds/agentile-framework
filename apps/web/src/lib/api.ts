const API_BASE = 'http://localhost:3001/api/v1';

export function getToken(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem('gradient_token');
}

export function setToken(token: string): void {
  localStorage.setItem('gradient_token', token);
}

export function clearToken(): void {
  localStorage.removeItem('gradient_token');
}

export async function apiFetch<T = unknown>(
  path: string,
  options: RequestInit = {},
): Promise<T> {
  const token = getToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...((options.headers as Record<string, string>) ?? {}),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers,
  });

  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.message ?? `API error ${res.status}`);
  }

  return res.json();
}

/* ---------- Auth ---------- */

export async function register(email: string, password: string) {
  return apiFetch<{ userId: string; token: string }>('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
}

export async function login(email: string, password: string) {
  const data = await apiFetch<{ token: string }>('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
  setToken(data.token);
  return data;
}

/* ---------- Submissions ---------- */

export async function getSubmissions() {
  return apiFetch<{ data: Submission[] }>('/submissions');
}

export async function createSubmission(body: {
  category: string;
  conditionDescription: string;
  estimatedBand: number;
}) {
  return apiFetch<Submission>('/submissions', {
    method: 'POST',
    body: JSON.stringify(body),
  });
}

/* ---------- Claims ---------- */

export async function getClaims() {
  return apiFetch<{ data: Claim[] }>('/claims');
}

/* ---------- Shipments ---------- */

export async function getShipments() {
  return apiFetch<{ data: Shipment[] }>('/shipments');
}

/* ---------- Notifications ---------- */

export async function getNotifications() {
  return apiFetch<{ data: Notification[] }>('/notifications');
}

/* ---------- Courier ---------- */

export async function getCourierTasks() {
  return apiFetch<{ data: CourierTask[] }>('/courier/tasks');
}

export async function acceptCourierTask(taskId: string) {
  return apiFetch(`/courier/tasks/${taskId}/accept`, {
    method: 'POST',
  });
}

/* ---------- Disputes ---------- */

export async function createDispute(body: {
  objectType: string;
  objectId: string;
  reason: string;
}) {
  return apiFetch('/disputes', {
    method: 'POST',
    body: JSON.stringify(body),
  });
}

export async function getDisputes() {
  return apiFetch<{ data: Dispute[] }>('/disputes');
}

export async function getAdminDisputeQueue() {
  return apiFetch<{ data: Dispute[] }>('/admin/disputes/queue');
}

/* ---------- Types ---------- */

export interface Submission {
  id: string;
  category: string;
  conditionDescription: string;
  estimatedBand: number;
  status: string;
  createdAt: string;
}

export interface Claim {
  id: string;
  poolId: number;
  receiptId: number;
  status: string;
  createdAt: string;
}

export interface Shipment {
  id: string;
  trackingNumber: string;
  carrier: string;
  direction: 'INBOUND' | 'OUTBOUND';
  status: string;
  createdAt: string;
}

export interface Notification {
  id: string;
  message: string;
  read: boolean;
  createdAt: string;
}

export interface CourierTask {
  id: string;
  pickupCity: string;
  dropoffCity: string;
  fee: string;
  timeWindow: string;
  status: string;
}

export interface Dispute {
  id: string;
  objectType: string;
  objectId: string;
  reason: string;
  status: string;
  slaDeadline: string;
  createdAt: string;
}
