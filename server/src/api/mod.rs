use axum::{routing::{get, post, delete, put}, Router};

use crate::AppState;

pub mod auth;
pub mod bookmarks;
pub mod comments;
pub mod compare;
pub mod seed;
pub mod stacks;
pub mod tools;
pub mod trending;
pub mod users;

pub fn router() -> Router<AppState> {
    Router::new()
        // Auth
        .route("/api/join", post(auth::join))
        .route("/api/recover", post(auth::recover))
        .route("/api/me", get(auth::me))
        // Stacks
        .route("/api/stacks", post(stacks::create_stack))
        .route("/api/stacks", get(stacks::list_stacks))
        .route("/api/stacks/{id}", get(stacks::get_stack))
        .route("/api/stacks/{id}", put(stacks::update_stack))
        .route("/api/stacks/{id}", delete(stacks::delete_stack))
        .route("/api/stacks/{id}/vote", post(stacks::vote))
        .route("/api/stacks/{id}/vote", get(stacks::check_vote))
        // Comments
        .route("/api/stacks/{id}/comments", post(comments::create_comment))
        .route("/api/stacks/{id}/comments", get(comments::list_comments))
        // Bookmarks
        .route("/api/stacks/{id}/bookmark", post(bookmarks::add_bookmark))
        .route("/api/stacks/{id}/bookmark", delete(bookmarks::remove_bookmark))
        .route("/api/stacks/{id}/bookmark", get(bookmarks::check_bookmark))
        // Tools — compare MUST be before {name} to avoid path conflict
        .route("/api/tools", get(tools::list_tools))
        .route("/api/tools/compare", get(compare::compare_tools))
        .route("/api/tools/{name}/alternatives", get(tools::get_tool_alternatives))
        .route("/api/tools/{name}/pairs", get(tools::get_tool_pairs))
        .route("/api/tools/{name}", get(tools::get_tool))
        // Users
        .route("/api/users/{nickname}", get(users::get_user_profile))
        .route("/api/me/profile", put(users::update_profile))
        .route("/api/me/bookmarks", get(bookmarks::list_bookmarks))
        // Trending
        .route("/api/trending", get(trending::get_trending))
        // Stats
        .route("/api/stats", get(tools::stats))
        // Seed (dev only)
        .route("/api/seed", post(seed::seed))
}
