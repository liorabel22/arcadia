use serde_json::Value;
use sqlx::PgPool;
use arcadia_common::error::{Error, Result};
use crate::{
    models::{
        series::{Series, UserCreatedSeries},
        user::User,
    },
};

pub async fn create_series(
    pool: &PgPool,
    series: &UserCreatedSeries,
    current_user: &User,
) -> Result<Series> {
    let created_series = sqlx::query_as!(
        Series,
        r#"
            INSERT INTO series (name,description,created_by_id,covers,banners,tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#,
        series.name,
        series.description,
        current_user.id,
        &series.covers,
        &series.banners,
        &series.tags
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateSeries)?;

    Ok(created_series)
}

pub async fn find_series(pool: &PgPool, series_id: &i64) -> Result<Value> {
    let found_series = sqlx::query!(
        r#"
        SELECT
            jsonb_build_object(
                'series', to_jsonb(s),
                'title_groups', COALESCE(
                    jsonb_agg(tgd.title_group_data),
                    '[]'::jsonb
                )
            ) AS series_and_title_groups
        FROM
            series s
        LEFT JOIN
            title_groups tg ON s.id = tg.series_id
        LEFT JOIN
            get_title_groups_and_edition_group_and_torrents_lite() AS tgd ON tg.id = tgd.title_group_id
        WHERE
            s.id = $1
        GROUP BY
            s.id, s.*;
        "#,
        series_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::SeriesWithIdNotFound(*series_id))?;

    // Ok(serde_json::from_value(found_series.series_and_groups.unwrap()).unwrap())
    Ok(found_series.series_and_title_groups.unwrap())
}
