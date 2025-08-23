use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::artist::ArtistLite;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistLiteQuery {
    name: String,
}

#[utoipa::path(
    get,
    path = "/api/search/artist/lite",
    params (GetArtistLiteQuery),
    description = "Case insensitive",
    responses(
        (status = 200, description = "Successfully got the artists and some data about them", body=Vec<ArtistLite>),
    )
)]
pub async fn exec(
    query: web::Query<GetArtistLiteQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artists = arc.pool.find_artists_lite(&query.name).await?;

    Ok(HttpResponse::Ok().json(artists))
}
