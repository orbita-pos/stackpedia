use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- DB row types ---

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserRow {
    pub id: Uuid,
    pub nickname: String,
    pub recovery_code_hash: String,
    pub created_at: DateTime<Utc>,
    pub sponsor_url: Option<String>,
    pub recovery_prefix: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct StackRow {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub url: Option<String>,
    pub scale: String,
    pub lessons: Option<String>,
    pub upvotes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub forked_from: Option<Uuid>,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct StackToolRow {
    pub id: Uuid,
    pub stack_id: Uuid,
    pub name: String,
    pub category: String,
    pub why: String,
    pub cost: Option<String>,
    pub verdict: String,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct CommentRow {
    pub id: Uuid,
    pub stack_id: Uuid,
    pub creator_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct VoteRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stack_id: Uuid,
    pub direction: i16,
    pub created_at: Option<DateTime<Utc>>,
}

// --- API request types ---

#[derive(Deserialize)]
pub struct JoinRequest {
    pub nickname: String,
}

#[derive(Deserialize)]
pub struct RecoverRequest {
    pub recovery_code: String,
}

#[derive(Deserialize)]
pub struct CreateStackRequest {
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub url: Option<String>,
    pub scale: String,
    pub tools: Vec<CreateToolInput>,
    pub lessons: Option<String>,
    pub forked_from: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateToolInput {
    pub name: String,
    pub category: String,
    pub why: String,
    pub cost: Option<String>,
    pub verdict: String,
}

#[derive(Deserialize)]
pub struct VoteRequest {
    pub direction: String,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct ListStacksQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub category: Option<String>,
    pub tool: Option<String>,
    pub scale: Option<String>,
    pub search: Option<String>,
}

// --- API response types ---

#[derive(Serialize)]
pub struct JoinResponse {
    pub user_id: Uuid,
    pub nickname: String,
    pub recovery_code: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub nickname: String,
}

#[derive(Serialize)]
pub struct StackSummary {
    pub id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub url: Option<String>,
    pub scale: String,
    pub upvotes: i32,
    pub tool_count: i64,
    pub comment_count: i64,
    pub creator_nickname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct StackDetail {
    pub id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub url: Option<String>,
    pub scale: String,
    pub lessons: Option<String>,
    pub upvotes: i32,
    pub creator_id: Uuid,
    pub creator_nickname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tools: Vec<ToolResponse>,
    pub comments: Vec<CommentResponse>,
    pub history: Vec<StackHistoryEntry>,
    pub forked_from: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ToolResponse {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub why: String,
    pub cost: Option<String>,
    pub verdict: String,
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub creator_nickname: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ToolDirectoryEntry {
    pub name: String,
    pub category: String,
    pub stack_count: i64,
    pub avg_verdict: String,
}

#[derive(Serialize)]
pub struct ToolDetailEntry {
    pub stack_id: Uuid,
    pub project_name: String,
    pub why: String,
    pub verdict: String,
    pub cost: Option<String>,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_stacks: i64,
    pub total_tools: i64,
    pub total_users: i64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// --- Tool Pairings ---

#[derive(Serialize)]
pub struct ToolPairingEntry {
    pub name: String,
    pub category: String,
    pub pair_count: i64,
}

// --- Tool Comparison ---

#[derive(Deserialize)]
pub struct CompareToolsQuery {
    pub tools: String,
}

#[derive(Serialize)]
pub struct VerdictDistribution {
    pub love: i64,
    pub good: i64,
    pub meh: i64,
    pub regret: i64,
}

#[derive(Serialize)]
pub struct ToolComparisonEntry {
    pub name: String,
    pub category: String,
    pub stack_count: i64,
    pub verdict_distribution: VerdictDistribution,
    pub sample_whys: Vec<String>,
    pub common_costs: Vec<String>,
}

#[derive(Serialize)]
pub struct CompareResponse {
    pub tools: Vec<ToolComparisonEntry>,
    pub shared_stacks: i64,
}

// --- Trending ---

#[derive(Serialize)]
pub struct TrendingStack {
    pub id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub scale: String,
    pub recent_votes: i64,
    pub creator_nickname: String,
}

#[derive(Serialize)]
pub struct TrendingTool {
    pub name: String,
    pub category: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct TrendingResponse {
    pub top_stacks: Vec<TrendingStack>,
    pub hot_tools: Vec<TrendingTool>,
    pub most_regretted: Vec<TrendingTool>,
}

// --- Stack Updates ---

#[derive(Deserialize)]
pub struct UpdateStackRequest {
    pub project_name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub url: Option<String>,
    pub scale: Option<String>,
    pub lessons: Option<String>,
    pub tools: Option<Vec<CreateToolInput>>,
}

#[derive(Serialize)]
pub struct StackHistoryEntry {
    pub id: Uuid,
    pub change_type: String,
    pub detail: Option<String>,
    pub created_at: DateTime<Utc>,
}

// --- Tool Alternatives ---

#[derive(Serialize)]
pub struct ToolAlternative {
    pub name: String,
    pub category: String,
    pub times_chosen: i64,
    pub avg_verdict: String,
}

// --- Bookmarks ---

#[derive(Serialize)]
pub struct BookmarkCheckResponse {
    pub bookmarked: bool,
}

// --- User Profile ---

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub nickname: String,
    pub created_at: DateTime<Utc>,
    pub stack_count: i64,
    pub sponsor_url: Option<String>,
    pub stacks: Vec<StackSummary>,
}

// --- Update Profile ---

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub sponsor_url: Option<String>,
}

// --- Validation ---

const VALID_CATEGORIES: &[&str] = &["saas", "ecommerce", "api", "mobile", "desktop", "devtool", "other"];
const VALID_SCALES: &[&str] = &["hobby", "hundreds", "thousands", "tens_of_thousands", "hundreds_of_thousands", "millions"];
const VALID_TOOL_CATEGORIES: &[&str] = &["frontend", "backend", "database", "hosting", "auth", "payments", "monitoring", "cdn", "email", "storage", "other"];
const VALID_VERDICTS: &[&str] = &["love", "good", "meh", "regret"];

fn validate_url(url: &str) -> Result<(), String> {
    if !url.is_empty() {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("url must start with http:// or https://".into());
        }
        if url.len() > 500 {
            return Err("url must be max 500 chars".into());
        }
    }
    Ok(())
}

fn validate_tools(tools: &[CreateToolInput]) -> Result<(), String> {
    if tools.len() > 30 {
        return Err("maximum 30 tools allowed".into());
    }
    for (i, tool) in tools.iter().enumerate() {
        if tool.name.is_empty() || tool.name.len() > 100 {
            return Err(format!("tools[{}].name must be 1-100 chars", i));
        }
        if !VALID_TOOL_CATEGORIES.contains(&tool.category.as_str()) {
            return Err(format!("tools[{}].category must be one of: {}", i, VALID_TOOL_CATEGORIES.join(", ")));
        }
        if tool.why.is_empty() || tool.why.len() > 300 {
            return Err(format!("tools[{}].why must be 1-300 chars", i));
        }
        if !VALID_VERDICTS.contains(&tool.verdict.as_str()) {
            return Err(format!("tools[{}].verdict must be one of: {}", i, VALID_VERDICTS.join(", ")));
        }
    }
    Ok(())
}

impl CreateStackRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.project_name.is_empty() || self.project_name.len() > 100 {
            return Err("project_name must be 1-100 chars".into());
        }
        if self.description.is_empty() || self.description.len() > 200 {
            return Err("description must be 1-200 chars".into());
        }
        if !VALID_CATEGORIES.contains(&self.category.as_str()) {
            return Err(format!("category must be one of: {}", VALID_CATEGORIES.join(", ")));
        }
        if !VALID_SCALES.contains(&self.scale.as_str()) {
            return Err(format!("scale must be one of: {}", VALID_SCALES.join(", ")));
        }
        if self.tools.is_empty() {
            return Err("at least one tool is required".into());
        }
        if let Some(ref url) = self.url {
            validate_url(url)?;
        }
        validate_tools(&self.tools)?;
        if let Some(ref lessons) = self.lessons {
            if lessons.len() > 500 {
                return Err("lessons must be max 500 chars".into());
            }
        }
        Ok(())
    }
}

impl UpdateStackRequest {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.project_name {
            if name.is_empty() || name.len() > 100 {
                return Err("project_name must be 1-100 chars".into());
            }
        }
        if let Some(ref desc) = self.description {
            if desc.is_empty() || desc.len() > 200 {
                return Err("description must be 1-200 chars".into());
            }
        }
        if let Some(ref cat) = self.category {
            if !VALID_CATEGORIES.contains(&cat.as_str()) {
                return Err(format!("category must be one of: {}", VALID_CATEGORIES.join(", ")));
            }
        }
        if let Some(ref scale) = self.scale {
            if !VALID_SCALES.contains(&scale.as_str()) {
                return Err(format!("scale must be one of: {}", VALID_SCALES.join(", ")));
            }
        }
        if let Some(ref url) = self.url {
            validate_url(url)?;
        }
        if let Some(ref lessons) = self.lessons {
            if lessons.len() > 500 {
                return Err("lessons must be max 500 chars".into());
            }
        }
        if let Some(ref tools) = self.tools {
            if tools.is_empty() {
                return Err("at least one tool is required".into());
            }
            validate_tools(tools)?;
        }
        Ok(())
    }
}
