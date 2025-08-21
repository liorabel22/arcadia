use crate::{handlers::User, Arcadia};
use actix_web::{HttpResponse, web};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
  models::gift::{Gift, UserCreatedGift},
  repositories::gift_repository::create_gift,
};

#[utoipa::path(
    post,
    path = "/api/gift",
    responses(
        (status = 200, description = "Successfully sent the gift", body=Gift),
    )
)]
pub async fn send_gift(
    gift: web::Json<UserCreatedGift>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.bonus_points < gift.bonus_points {
        return Err(Error::NotEnoughBonusPointsAvailable);
    }
    if current_user.freeleech_tokens < gift.freeleech_tokens {
        return Err(Error::NotEnoughFreeleechTokensAvailable);
    }

    let gift = create_gift(arc.pool.borrow(), &gift, current_user.id).await?;

    Ok(HttpResponse::Created().json(gift))
}
