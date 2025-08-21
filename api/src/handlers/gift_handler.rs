use crate::{
    Arcadia, Error, Result,
    models::{
        gift::{Gift, UserCreatedGift},
        user::User,
    },
    repositories::gift_repository::create_gift,
};
use actix_web::{HttpResponse, web};

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

    let gift = create_gift(&arc.pool, &gift, current_user.id).await?;

    Ok(HttpResponse::Created().json(gift))
}
