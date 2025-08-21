use crate::{
    Arcadia, Result,
    models::title_group::TitleGroupLite,
    repositories::{
        forum_repository::find_first_thread_posts_in_sub_category,
        stats_repository::find_home_stats, title_group_repository::find_title_group_info_lite,
    },
};
use actix_web::{HttpResponse, web};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostAndThreadName {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub content: String,
    pub sticky: bool,
    pub forum_thread_name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct HomeStats {
    pub enabled_users: i64,
    pub users_active_today: i64,
    pub users_active_this_week: i64,
    pub users_active_this_month: i64,
    pub torrents: i64,
    pub torrents_uploaded_today: i64,
    pub titles: i64,
    pub artists: i64,
    pub entities: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HomePage {
    recent_announcements: Vec<ForumPostAndThreadName>,
    stats: HomeStats,
    latest_uploads: Vec<TitleGroupLite>,
}

#[utoipa::path(
    get,
    path = "/api/home",
    responses(
        (status = 200, description = "", body=HomePage),
    )
)]
pub async fn get_home(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    let recent_announcements = find_first_thread_posts_in_sub_category(&arc.pool, 1, 5).await?;
    let stats = find_home_stats(&arc.pool).await?;
    let latest_uploads_in_title_groups =
        find_title_group_info_lite(&arc.pool, None, Some(""), &None, 5).await?;

    Ok(HttpResponse::Created().json(json!({
        "recent_announcements":recent_announcements,
        "stats": stats,
        "latest_uploads": latest_uploads_in_title_groups,
    })))
}
