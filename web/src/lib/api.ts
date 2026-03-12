import type {
  User,
  JoinResponse,
  StackSummary,
  StackDetail,
  ToolDirectoryEntry,
  ToolDetailEntry,
  StatsResponse,
  VoteResponse,
  CommentResponse,
  CreateStackRequest,
  UpdateStackRequest,
  ToolPairingEntry,
  ToolAlternative,
  CompareResponse,
  TrendingResponse,
  UserProfileResponse,
} from "./types";

const API = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API}${path}`, {
    credentials: "include",
    headers: { "Content-Type": "application/json", ...(options?.headers || {}) },
    ...options,
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error || res.statusText);
  }
  if (res.status === 204) return undefined as T;
  return res.json();
}

// Auth
export const join = (nickname: string) =>
  request<JoinResponse>("/api/join", {
    method: "POST",
    body: JSON.stringify({ nickname }),
  });

export const recover = (recovery_code: string) =>
  request<User>("/api/recover", {
    method: "POST",
    body: JSON.stringify({ recovery_code }),
  });

export const getMe = () => request<User>("/api/me");

// Stacks
export interface ListStacksParams {
  page?: number;
  limit?: number;
  sort?: string;
  category?: string;
  tool?: string;
  scale?: string;
  search?: string;
}

export const listStacks = (params?: ListStacksParams) => {
  const query = new URLSearchParams();
  if (params) {
    Object.entries(params).forEach(([k, v]) => {
      if (v !== undefined && v !== "" && v !== "all") query.set(k, String(v));
    });
  }
  const qs = query.toString();
  return request<StackSummary[]>(`/api/stacks${qs ? `?${qs}` : ""}`);
};

export const getStack = (id: string) => request<StackDetail>(`/api/stacks/${id}`);

export const createStack = (data: CreateStackRequest) =>
  request<StackDetail>("/api/stacks", {
    method: "POST",
    body: JSON.stringify(data),
  });

export const updateStack = (id: string, data: UpdateStackRequest) =>
  request<StackDetail>(`/api/stacks/${id}`, {
    method: "PUT",
    body: JSON.stringify(data),
  });

export const deleteStack = (id: string) =>
  request<void>(`/api/stacks/${id}`, { method: "DELETE" });

export const vote = (stackId: string, direction: "up" | "down") =>
  request<VoteResponse>(`/api/stacks/${stackId}/vote`, {
    method: "POST",
    body: JSON.stringify({ direction }),
  });

export const getMyVote = (stackId: string) =>
  request<{ direction: string }>(`/api/stacks/${stackId}/vote`);

// Comments
export const getComments = (stackId: string) =>
  request<CommentResponse[]>(`/api/stacks/${stackId}/comments`);

export const createComment = (stackId: string, content: string) =>
  request<CommentResponse>(`/api/stacks/${stackId}/comments`, {
    method: "POST",
    body: JSON.stringify({ content }),
  });

// Tools
export const listTools = () => request<ToolDirectoryEntry[]>("/api/tools");

export const getTool = (name: string) =>
  request<ToolDetailEntry[]>(`/api/tools/${encodeURIComponent(name)}`);

export const getToolPairs = (name: string) =>
  request<ToolPairingEntry[]>(`/api/tools/${encodeURIComponent(name)}/pairs`);

export const getToolAlternatives = (name: string) =>
  request<ToolAlternative[]>(`/api/tools/${encodeURIComponent(name)}/alternatives`);

// Bookmarks list
export const getMyBookmarks = () => request<StackSummary[]>("/api/me/bookmarks");

// Compare
export const compareTools = (tool1: string, tool2: string) =>
  request<CompareResponse>(
    `/api/tools/compare?tools=${encodeURIComponent(tool1)},${encodeURIComponent(tool2)}`
  );

// Trending
export const getTrending = () => request<TrendingResponse>("/api/trending");

// Stats
export const getStats = () => request<StatsResponse>("/api/stats");

// Users
export const getUserProfile = (nickname: string) =>
  request<UserProfileResponse>(`/api/users/${encodeURIComponent(nickname)}`);

// Profile
export const updateProfile = (data: { sponsor_url?: string }) =>
  request<{ ok: boolean }>("/api/me/profile", {
    method: "PUT",
    body: JSON.stringify(data),
  });

// Bookmarks
export const bookmarkStack = (stackId: string) =>
  request<void>(`/api/stacks/${stackId}/bookmark`, { method: "POST" });

export const unbookmarkStack = (stackId: string) =>
  request<void>(`/api/stacks/${stackId}/bookmark`, { method: "DELETE" });

export const isBookmarked = (stackId: string) =>
  request<{ bookmarked: boolean }>(`/api/stacks/${stackId}/bookmark`).then(
    (r) => r.bookmarked
  );

// Seed
export const seed = () =>
  request<{ message: string }>("/api/seed", { method: "POST" });
