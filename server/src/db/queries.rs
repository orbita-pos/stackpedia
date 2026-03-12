use sqlx::PgPool;
use uuid::Uuid;

use super::models::*;

// --- Users ---

pub async fn insert_user(
    pool: &PgPool,
    id: Uuid,
    nickname: &str,
    recovery_code_hash: &str,
    recovery_prefix: Option<&str>,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"INSERT INTO users (id, nickname, recovery_code_hash, recovery_prefix)
           VALUES ($1, $2, $3, $4)
           RETURNING id, nickname, recovery_code_hash, created_at, sponsor_url, recovery_prefix"#,
    )
    .bind(id)
    .bind(nickname)
    .bind(recovery_code_hash)
    .bind(recovery_prefix)
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, nickname, recovery_code_hash, created_at, sponsor_url, recovery_prefix FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, nickname, recovery_code_hash, created_at, sponsor_url, recovery_prefix FROM users",
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_nickname(pool: &PgPool, nickname: &str) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, nickname, recovery_code_hash, created_at, sponsor_url, recovery_prefix FROM users WHERE LOWER(nickname) = LOWER($1)",
    )
    .bind(nickname)
    .fetch_optional(pool)
    .await
}

pub async fn get_users_by_recovery_prefix(pool: &PgPool, prefix: &str) -> Result<Vec<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, nickname, recovery_code_hash, created_at, sponsor_url, recovery_prefix FROM users WHERE recovery_prefix = $1",
    )
    .bind(prefix)
    .fetch_all(pool)
    .await
}

pub async fn get_stacks_by_creator(pool: &PgPool, user_id: Uuid) -> Result<Vec<StackSummaryRow>, sqlx::Error> {
    sqlx::query_as::<_, StackSummaryRow>(
        r#"SELECT s.id, s.project_name, s.description, s.category, s.url, s.scale, s.upvotes,
                  (SELECT COUNT(*) FROM stack_tools st WHERE st.stack_id = s.id) AS tool_count,
                  (SELECT COUNT(*) FROM comments c WHERE c.stack_id = s.id) AS comment_count,
                  u.nickname AS creator_nickname,
                  s.created_at,
                  s.updated_at
           FROM stacks s
           JOIN users u ON u.id = s.creator_id
           WHERE s.creator_id = $1
           ORDER BY s.created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn update_sponsor_url(pool: &PgPool, user_id: Uuid, sponsor_url: Option<&str>) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET sponsor_url = $2 WHERE id = $1")
        .bind(user_id)
        .bind(sponsor_url)
        .execute(pool)
        .await?;
    Ok(())
}

fn escape_like(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

// --- Stacks ---

pub async fn insert_stack(
    pool: &PgPool,
    id: Uuid,
    creator_id: Uuid,
    project_name: &str,
    description: &str,
    category: &str,
    url: Option<&str>,
    scale: &str,
    lessons: Option<&str>,
    forked_from: Option<Uuid>,
) -> Result<StackRow, sqlx::Error> {
    sqlx::query_as::<_, StackRow>(
        r#"INSERT INTO stacks (id, creator_id, project_name, description, category, url, scale, lessons, forked_from)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING id, creator_id, project_name, description, category, url, scale, lessons, upvotes, created_at, updated_at, forked_from"#,
    )
    .bind(id)
    .bind(creator_id)
    .bind(project_name)
    .bind(description)
    .bind(category)
    .bind(url)
    .bind(scale)
    .bind(lessons)
    .bind(forked_from)
    .fetch_one(pool)
    .await
}

pub async fn insert_stack_tool(
    pool: &PgPool,
    id: Uuid,
    stack_id: Uuid,
    name: &str,
    category: &str,
    why: &str,
    cost: Option<&str>,
    verdict: &str,
) -> Result<StackToolRow, sqlx::Error> {
    sqlx::query_as::<_, StackToolRow>(
        r#"INSERT INTO stack_tools (id, stack_id, name, category, why, cost, verdict)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, stack_id, name, category, why, cost, verdict"#,
    )
    .bind(id)
    .bind(stack_id)
    .bind(name)
    .bind(category)
    .bind(why)
    .bind(cost)
    .bind(verdict)
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct StackSummaryRow {
    pub id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub url: Option<String>,
    pub scale: String,
    pub upvotes: i32,
    pub tool_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub creator_nickname: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list_stacks(
    pool: &PgPool,
    query: &ListStacksQuery,
) -> Result<Vec<StackSummaryRow>, sqlx::Error> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let sort_clause = match query.sort.as_deref() {
        Some("top") => "s.upvotes DESC",
        Some("discussed") => "comment_count DESC",
        _ => "s.created_at DESC",
    };

    // Build dynamic WHERE clauses
    let mut conditions = Vec::new();
    let mut param_idx = 1u32;

    if query.category.is_some() {
        conditions.push(format!("s.category = ${}", param_idx));
        param_idx += 1;
    }
    if query.scale.is_some() {
        conditions.push(format!("s.scale = ${}", param_idx));
        param_idx += 1;
    }
    let tool_names: Vec<String> = query
        .tool
        .as_deref()
        .map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    for _ in &tool_names {
        conditions.push(format!(
            "EXISTS (SELECT 1 FROM stack_tools st_f WHERE st_f.stack_id = s.id AND LOWER(st_f.name) = LOWER(${}))",
            param_idx
        ));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(s.project_name ILIKE ${0} OR s.description ILIKE ${0})",
            param_idx
        ));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        r#"SELECT s.id, s.project_name, s.description, s.category, s.url, s.scale, s.upvotes,
                  (SELECT COUNT(*) FROM stack_tools st WHERE st.stack_id = s.id) AS tool_count,
                  (SELECT COUNT(*) FROM comments c WHERE c.stack_id = s.id) AS comment_count,
                  u.nickname AS creator_nickname,
                  s.created_at,
                  s.updated_at
           FROM stacks s
           JOIN users u ON u.id = s.creator_id
           {}
           ORDER BY {}
           LIMIT ${} OFFSET ${}"#,
        where_clause,
        sort_clause,
        param_idx,
        param_idx + 1
    );

    let mut q = sqlx::query_as::<_, StackSummaryRow>(&sql);

    if let Some(ref cat) = query.category {
        q = q.bind(cat);
    }
    if let Some(ref scale) = query.scale {
        q = q.bind(scale);
    }
    for tool_name in &tool_names {
        q = q.bind(tool_name.clone());
    }
    if let Some(ref search) = query.search {
        q = q.bind(format!("%{}%", escape_like(search)));
    }

    q = q.bind(limit).bind(offset);

    q.fetch_all(pool).await
}

pub async fn get_stack_by_id(pool: &PgPool, id: Uuid) -> Result<Option<StackRow>, sqlx::Error> {
    sqlx::query_as::<_, StackRow>(
        r#"SELECT id, creator_id, project_name, description, category, url, scale, lessons, upvotes, created_at, updated_at, forked_from
           FROM stacks WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn get_stack_tools(pool: &PgPool, stack_id: Uuid) -> Result<Vec<StackToolRow>, sqlx::Error> {
    sqlx::query_as::<_, StackToolRow>(
        "SELECT id, stack_id, name, category, why, cost, verdict FROM stack_tools WHERE stack_id = $1",
    )
    .bind(stack_id)
    .fetch_all(pool)
    .await
}

pub async fn delete_stack(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM stacks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_creator_nickname(pool: &PgPool, creator_id: Uuid) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT nickname FROM users WHERE id = $1")
        .bind(creator_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

// --- Votes ---

pub async fn get_vote(
    pool: &PgPool,
    user_id: Uuid,
    stack_id: Uuid,
) -> Result<Option<VoteRow>, sqlx::Error> {
    sqlx::query_as::<_, VoteRow>(
        "SELECT id, user_id, stack_id, direction, created_at FROM votes WHERE user_id = $1 AND stack_id = $2",
    )
    .bind(user_id)
    .bind(stack_id)
    .fetch_optional(pool)
    .await
}

pub async fn insert_vote(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    stack_id: Uuid,
    direction: i16,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO votes (id, user_id, stack_id, direction) VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(user_id)
    .bind(stack_id)
    .bind(direction)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_vote(pool: &PgPool, vote_id: Uuid, direction: i16) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE votes SET direction = $1 WHERE id = $2")
        .bind(direction)
        .bind(vote_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_vote(pool: &PgPool, vote_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM votes WHERE id = $1")
        .bind(vote_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn recalc_upvotes(pool: &PgPool, stack_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE stacks SET upvotes = COALESCE(
             (SELECT SUM(direction::int) FROM votes WHERE stack_id = $1), 0
           ) WHERE id = $1"#,
    )
    .bind(stack_id)
    .execute(pool)
    .await?;
    Ok(())
}

// --- Comments ---

pub async fn insert_comment(
    pool: &PgPool,
    id: Uuid,
    stack_id: Uuid,
    creator_id: Uuid,
    content: &str,
) -> Result<CommentRow, sqlx::Error> {
    sqlx::query_as::<_, CommentRow>(
        r#"INSERT INTO comments (id, stack_id, creator_id, content)
           VALUES ($1, $2, $3, $4)
           RETURNING id, stack_id, creator_id, content, created_at"#,
    )
    .bind(id)
    .bind(stack_id)
    .bind(creator_id)
    .bind(content)
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct CommentWithNickname {
    pub id: Uuid,
    pub content: String,
    pub creator_nickname: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_comments_for_stack(
    pool: &PgPool,
    stack_id: Uuid,
) -> Result<Vec<CommentWithNickname>, sqlx::Error> {
    sqlx::query_as::<_, CommentWithNickname>(
        r#"SELECT c.id, c.content, u.nickname AS creator_nickname, c.created_at
           FROM comments c
           JOIN users u ON u.id = c.creator_id
           WHERE c.stack_id = $1
           ORDER BY c.created_at DESC"#,
    )
    .bind(stack_id)
    .fetch_all(pool)
    .await
}

// --- Tools directory ---

#[derive(sqlx::FromRow)]
pub struct ToolDirectoryRow {
    pub name: Option<String>,
    pub category: Option<String>,
    pub stack_count: Option<i64>,
    pub verdicts: Option<String>,
}

pub async fn get_tools_directory(pool: &PgPool) -> Result<Vec<ToolDirectoryRow>, sqlx::Error> {
    sqlx::query_as::<_, ToolDirectoryRow>(
        r#"SELECT name,
                  (ARRAY_AGG(category ORDER BY category))[1] AS category,
                  COUNT(DISTINCT stack_id) AS stack_count,
                  STRING_AGG(verdict, ',') AS verdicts
           FROM stack_tools
           GROUP BY name
           ORDER BY stack_count DESC"#,
    )
    .fetch_all(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct ToolDetailRow {
    pub stack_id: Option<Uuid>,
    pub project_name: Option<String>,
    pub why: String,
    pub verdict: String,
    pub cost: Option<String>,
}

pub async fn get_stacks_for_tool(
    pool: &PgPool,
    tool_name: &str,
) -> Result<Vec<ToolDetailRow>, sqlx::Error> {
    sqlx::query_as::<_, ToolDetailRow>(
        r#"SELECT st.stack_id, s.project_name, st.why, st.verdict, st.cost
           FROM stack_tools st
           JOIN stacks s ON s.id = st.stack_id
           WHERE st.name = $1
           ORDER BY s.created_at DESC"#,
    )
    .bind(tool_name)
    .fetch_all(pool)
    .await
}

// --- Stats ---

#[derive(sqlx::FromRow)]
pub struct StatsRow {
    pub total_stacks: Option<i64>,
    pub total_tools: Option<i64>,
    pub total_users: Option<i64>,
}

pub async fn get_stats(pool: &PgPool) -> Result<StatsRow, sqlx::Error> {
    sqlx::query_as::<_, StatsRow>(
        r#"SELECT
             (SELECT COUNT(*) FROM stacks) AS total_stacks,
             (SELECT COUNT(DISTINCT name) FROM stack_tools) AS total_tools,
             (SELECT COUNT(*) FROM users) AS total_users"#,
    )
    .fetch_one(pool)
    .await
}

// --- Tool Pairings ---

#[derive(sqlx::FromRow)]
pub struct ToolPairingRow {
    pub name: Option<String>,
    pub category: Option<String>,
    pub pair_count: Option<i64>,
}

pub async fn get_tool_pairs(pool: &PgPool, tool_name: &str) -> Result<Vec<ToolPairingRow>, sqlx::Error> {
    sqlx::query_as::<_, ToolPairingRow>(
        r#"SELECT st2.name, (ARRAY_AGG(st2.category))[1] AS category, COUNT(DISTINCT st2.stack_id) AS pair_count
           FROM stack_tools st1
           JOIN stack_tools st2 ON st2.stack_id = st1.stack_id AND st2.name != st1.name
           WHERE LOWER(st1.name) = LOWER($1)
           GROUP BY st2.name
           ORDER BY pair_count DESC
           LIMIT 10"#,
    )
    .bind(tool_name)
    .fetch_all(pool)
    .await
}

// --- Tool Comparison ---

#[derive(sqlx::FromRow)]
pub struct ToolComparisonRow {
    pub stack_count: Option<i64>,
    pub love: Option<i64>,
    pub good: Option<i64>,
    pub meh: Option<i64>,
    pub regret: Option<i64>,
    pub category: Option<String>,
}

pub async fn get_tool_comparison(pool: &PgPool, tool_name: &str) -> Result<ToolComparisonRow, sqlx::Error> {
    sqlx::query_as::<_, ToolComparisonRow>(
        r#"SELECT COUNT(DISTINCT stack_id) AS stack_count,
                  COUNT(*) FILTER (WHERE verdict='love') AS love,
                  COUNT(*) FILTER (WHERE verdict='good') AS good,
                  COUNT(*) FILTER (WHERE verdict='meh') AS meh,
                  COUNT(*) FILTER (WHERE verdict='regret') AS regret,
                  (ARRAY_AGG(category))[1] AS category
           FROM stack_tools WHERE LOWER(name) = LOWER($1)"#,
    )
    .bind(tool_name)
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct SharedStacksRow {
    pub count: Option<i64>,
}

pub async fn get_shared_stacks_count(pool: &PgPool, tool1: &str, tool2: &str) -> Result<i64, sqlx::Error> {
    let row = sqlx::query_as::<_, SharedStacksRow>(
        r#"SELECT COUNT(*) AS count FROM (
             SELECT stack_id FROM stack_tools WHERE LOWER(name) = LOWER($1)
             INTERSECT
             SELECT stack_id FROM stack_tools WHERE LOWER(name) = LOWER($2)
           ) shared"#,
    )
    .bind(tool1)
    .bind(tool2)
    .fetch_one(pool)
    .await?;
    Ok(row.count.unwrap_or(0))
}

#[derive(sqlx::FromRow)]
pub struct WhyRow {
    pub why: String,
}

pub async fn get_sample_whys(pool: &PgPool, tool_name: &str, limit: i64) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, WhyRow>(
        r#"SELECT why FROM stack_tools WHERE LOWER(name) = LOWER($1) AND why != '' LIMIT $2"#,
    )
    .bind(tool_name)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.why).collect())
}

#[derive(sqlx::FromRow)]
pub struct CostRow {
    pub cost: Option<String>,
}

pub async fn get_common_costs(pool: &PgPool, tool_name: &str, limit: i64) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_as::<_, CostRow>(
        r#"SELECT DISTINCT cost FROM stack_tools WHERE LOWER(name) = LOWER($1) AND cost IS NOT NULL LIMIT $2"#,
    )
    .bind(tool_name)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().filter_map(|r| r.cost).collect())
}

// --- Trending ---

#[derive(sqlx::FromRow)]
pub struct TrendingStackRow {
    pub id: Uuid,
    pub project_name: String,
    pub description: String,
    pub category: String,
    pub scale: String,
    pub recent_votes: Option<i64>,
    pub creator_nickname: Option<String>,
}

pub async fn get_trending_stacks(pool: &PgPool, days: i32) -> Result<Vec<TrendingStackRow>, sqlx::Error> {
    sqlx::query_as::<_, TrendingStackRow>(
        r#"SELECT s.id, s.project_name, s.description, s.category, s.scale,
                  COUNT(v.id) AS recent_votes,
                  u.nickname AS creator_nickname
           FROM stacks s
           JOIN users u ON u.id = s.creator_id
           LEFT JOIN votes v ON v.stack_id = s.id AND v.direction = 1
                AND v.created_at >= now() - make_interval(days => $1)
           GROUP BY s.id, s.project_name, s.description, s.category, s.scale, u.nickname
           HAVING COUNT(v.id) > 0
           ORDER BY recent_votes DESC
           LIMIT 5"#,
    )
    .bind(days)
    .fetch_all(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct TrendingToolRow {
    pub name: Option<String>,
    pub category: Option<String>,
    pub count: Option<i64>,
}

pub async fn get_hot_tools(pool: &PgPool, days: i32) -> Result<Vec<TrendingToolRow>, sqlx::Error> {
    sqlx::query_as::<_, TrendingToolRow>(
        r#"SELECT st.name, (ARRAY_AGG(st.category))[1] AS category, COUNT(DISTINCT st.stack_id) AS count
           FROM stack_tools st
           JOIN stacks s ON s.id = st.stack_id
           WHERE s.created_at >= now() - make_interval(days => $1)
           GROUP BY st.name
           ORDER BY count DESC
           LIMIT 5"#,
    )
    .bind(days)
    .fetch_all(pool)
    .await
}

pub async fn get_most_regretted(pool: &PgPool) -> Result<Vec<TrendingToolRow>, sqlx::Error> {
    sqlx::query_as::<_, TrendingToolRow>(
        r#"SELECT name, (ARRAY_AGG(category))[1] AS category, COUNT(*) AS count
           FROM stack_tools
           WHERE verdict = 'regret'
           GROUP BY name
           HAVING COUNT(*) >= 2
           ORDER BY count DESC
           LIMIT 5"#,
    )
    .fetch_all(pool)
    .await
}

// --- Stack Updates ---

pub async fn update_stack(
    pool: &PgPool,
    id: Uuid,
    project_name: Option<&str>,
    description: Option<&str>,
    category: Option<&str>,
    url: Option<&str>,
    scale: Option<&str>,
    lessons: Option<&str>,
) -> Result<StackRow, sqlx::Error> {
    sqlx::query_as::<_, StackRow>(
        r#"UPDATE stacks SET
             project_name = COALESCE($2, project_name),
             description = COALESCE($3, description),
             category = COALESCE($4, category),
             url = COALESCE($5, url),
             scale = COALESCE($6, scale),
             lessons = COALESCE($7, lessons),
             updated_at = now()
           WHERE id = $1
           RETURNING id, creator_id, project_name, description, category, url, scale, lessons, upvotes, created_at, updated_at, forked_from"#,
    )
    .bind(id)
    .bind(project_name)
    .bind(description)
    .bind(category)
    .bind(url)
    .bind(scale)
    .bind(lessons)
    .fetch_one(pool)
    .await
}

pub async fn delete_stack_tools(pool: &PgPool, stack_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM stack_tools WHERE stack_id = $1")
        .bind(stack_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_stack_history(
    pool: &PgPool,
    id: Uuid,
    stack_id: Uuid,
    changed_by: Uuid,
    change_type: &str,
    detail: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO stack_history (id, stack_id, changed_by, change_type, detail)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(id)
    .bind(stack_id)
    .bind(changed_by)
    .bind(change_type)
    .bind(detail)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct StackHistoryRow {
    pub id: Uuid,
    pub change_type: String,
    pub detail: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_stack_history(pool: &PgPool, stack_id: Uuid) -> Result<Vec<StackHistoryRow>, sqlx::Error> {
    sqlx::query_as::<_, StackHistoryRow>(
        r#"SELECT id, change_type, detail, created_at
           FROM stack_history
           WHERE stack_id = $1
           ORDER BY created_at DESC
           LIMIT 20"#,
    )
    .bind(stack_id)
    .fetch_all(pool)
    .await
}

// --- Bookmarks ---

pub async fn add_bookmark(pool: &PgPool, id: Uuid, user_id: Uuid, stack_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO bookmarks (id, user_id, stack_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
    )
    .bind(id)
    .bind(user_id)
    .bind(stack_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_bookmark(pool: &PgPool, user_id: Uuid, stack_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM bookmarks WHERE user_id = $1 AND stack_id = $2")
        .bind(user_id)
        .bind(stack_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct BookmarkExistsRow {
    pub exists: Option<bool>,
}

pub async fn is_bookmarked(pool: &PgPool, user_id: Uuid, stack_id: Uuid) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_as::<_, BookmarkExistsRow>(
        "SELECT EXISTS(SELECT 1 FROM bookmarks WHERE user_id = $1 AND stack_id = $2) AS exists",
    )
    .bind(user_id)
    .bind(stack_id)
    .fetch_one(pool)
    .await?;
    Ok(row.exists.unwrap_or(false))
}

pub async fn get_bookmarked_stacks(pool: &PgPool, user_id: Uuid) -> Result<Vec<StackSummaryRow>, sqlx::Error> {
    sqlx::query_as::<_, StackSummaryRow>(
        r#"SELECT s.id, s.project_name, s.description, s.category, s.url, s.scale, s.upvotes,
                  (SELECT COUNT(*) FROM stack_tools st WHERE st.stack_id = s.id) AS tool_count,
                  (SELECT COUNT(*) FROM comments c WHERE c.stack_id = s.id) AS comment_count,
                  u.nickname AS creator_nickname,
                  s.created_at, s.updated_at
           FROM bookmarks b
           JOIN stacks s ON s.id = b.stack_id
           JOIN users u ON u.id = s.creator_id
           WHERE b.user_id = $1
           ORDER BY b.created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

// --- Tool Alternatives ---

#[derive(sqlx::FromRow)]
pub struct ToolAlternativeRow {
    pub name: Option<String>,
    pub category: Option<String>,
    pub times_chosen: Option<i64>,
    pub avg_verdict: Option<String>,
}

pub async fn get_tool_alternatives(pool: &PgPool, tool_name: &str) -> Result<Vec<ToolAlternativeRow>, sqlx::Error> {
    sqlx::query_as::<_, ToolAlternativeRow>(
        r#"SELECT st2.name, (ARRAY_AGG(st2.category))[1] AS category,
                  COUNT(DISTINCT st2.stack_id) AS times_chosen,
                  (ARRAY_AGG(st2.verdict ORDER BY st2.verdict))[1] AS avg_verdict
           FROM stack_tools st1
           JOIN stack_tools st2 ON st2.stack_id = st1.stack_id
             AND st2.name != st1.name
             AND st2.category = st1.category
           WHERE LOWER(st1.name) = LOWER($1)
             AND st1.verdict = 'regret'
             AND st2.verdict IN ('love', 'good')
           GROUP BY st2.name
           ORDER BY times_chosen DESC
           LIMIT 5"#,
    )
    .bind(tool_name)
    .fetch_all(pool)
    .await
}
