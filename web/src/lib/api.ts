const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";
const API_KEY = process.env.NEXT_PUBLIC_API_KEY || "";

export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface ToolInfo {
  name: string;
  category: string;
  description: string;
  risk: string;
}

export async function callTool<T = unknown>(
  toolName: string,
  params: Record<string, unknown> = {}
): Promise<ApiResponse<T>> {
  const res = await fetch(`${API_BASE}/api/tools/${toolName}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_KEY}`,
    },
    body: JSON.stringify(params),
  });
  return res.json();
}

export async function getHealth(): Promise<ApiResponse> {
  const res = await fetch(`${API_BASE}/api/health`);
  return res.json();
}

export async function getTools(): Promise<ToolInfo[]> {
  const res = await fetch(`${API_BASE}/api/tools`);
  return res.json();
}
