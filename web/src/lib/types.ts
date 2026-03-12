export interface User {
  user_id: string;
  nickname: string;
}

export interface JoinResponse {
  user_id: string;
  nickname: string;
  recovery_code: string;
}

export interface StackSummary {
  id: string;
  project_name: string;
  description: string;
  category: string;
  url: string | null;
  scale: string;
  upvotes: number;
  tool_count: number;
  comment_count: number;
  creator_nickname: string;
  created_at: string;
  updated_at: string | null;
}

export interface ToolResponse {
  id: string;
  name: string;
  category: string;
  why: string;
  cost: string | null;
  verdict: string;
}

export interface CommentResponse {
  id: string;
  content: string;
  creator_nickname: string;
  created_at: string;
}

export interface StackHistoryEntry {
  id: string;
  change_type: string;
  detail: string | null;
  created_at: string;
}

export interface StackDetail {
  id: string;
  project_name: string;
  description: string;
  category: string;
  url: string | null;
  scale: string;
  lessons: string | null;
  upvotes: number;
  creator_id: string;
  creator_nickname: string;
  created_at: string;
  updated_at: string | null;
  tools: ToolResponse[];
  comments: CommentResponse[];
  history: StackHistoryEntry[];
  forked_from?: string | null;
}

export interface ToolDirectoryEntry {
  name: string;
  category: string;
  stack_count: number;
  avg_verdict: string;
}

export interface ToolDetailEntry {
  stack_id: string;
  project_name: string;
  why: string;
  verdict: string;
  cost: string | null;
}

export interface StatsResponse {
  total_stacks: number;
  total_tools: number;
  total_users: number;
}

export interface VoteResponse {
  action: string;
  upvotes: number;
}

export interface CreateStackRequest {
  project_name: string;
  description: string;
  category: string;
  url?: string;
  scale: string;
  tools: CreateToolInput[];
  lessons?: string;
  forked_from?: string;
}

export interface CreateToolInput {
  name: string;
  category: string;
  why: string;
  cost?: string;
  verdict: string;
}

export interface UpdateStackRequest {
  project_name?: string;
  description?: string;
  category?: string;
  url?: string;
  scale?: string;
  lessons?: string;
  tools?: CreateToolInput[];
}

// Tool Alternatives
export interface ToolAlternative {
  name: string;
  category: string;
  times_chosen: number;
  avg_verdict: string;
}

// Tool Pairings
export interface ToolPairingEntry {
  name: string;
  category: string;
  pair_count: number;
}

// Tool Comparison
export interface VerdictDistribution {
  love: number;
  good: number;
  meh: number;
  regret: number;
}

export interface ToolComparisonEntry {
  name: string;
  category: string;
  stack_count: number;
  verdict_distribution: VerdictDistribution;
  sample_whys: string[];
  common_costs: string[];
}

export interface CompareResponse {
  tools: ToolComparisonEntry[];
  shared_stacks: number;
}

// Trending
export interface TrendingStack {
  id: string;
  project_name: string;
  description: string;
  category: string;
  scale: string;
  recent_votes: number;
  creator_nickname: string;
}

export interface TrendingTool {
  name: string;
  category: string;
  count: number;
}

export interface TrendingResponse {
  top_stacks: TrendingStack[];
  hot_tools: TrendingTool[];
  most_regretted: TrendingTool[];
}

// User Profiles
export interface UserProfileResponse {
  nickname: string;
  created_at: string;
  stack_count: number;
  sponsor_url: string | null;
  stacks: StackSummary[];
}

export const CATEGORIES = ["saas", "ecommerce", "api", "mobile", "desktop", "devtool", "other"] as const;
export const SCALES = ["hobby", "hundreds", "thousands", "tens_of_thousands", "hundreds_of_thousands", "millions"] as const;
export const TOOL_CATEGORIES = ["frontend", "backend", "database", "hosting", "auth", "payments", "monitoring", "cdn", "email", "storage", "other"] as const;
export const VERDICTS = ["love", "good", "meh", "regret"] as const;

export const SCALE_LABELS: Record<string, string> = {
  hobby: "Hobby",
  hundreds: "Hundreds",
  thousands: "Thousands",
  tens_of_thousands: "10k+",
  hundreds_of_thousands: "100k+",
  millions: "Millions",
};

export const VERDICT_COLORS: Record<string, string> = {
  love: "text-green-600 dark:text-green-400 bg-green-500/10 border-green-500/30",
  good: "text-blue-600 dark:text-blue-400 bg-blue-500/10 border-blue-500/30",
  meh: "text-yellow-600 dark:text-yellow-400 bg-yellow-500/10 border-yellow-500/30",
  regret: "text-red-600 dark:text-red-400 bg-red-500/10 border-red-500/30",
};

export const VERDICT_DOT: Record<string, string> = {
  love: "bg-green-500",
  good: "bg-blue-500",
  meh: "bg-yellow-500",
  regret: "bg-red-500",
};
