use crate::{
    Arcadia, handlers::UserId,
};
use actix_web::{HttpResponse, web};
use arcadia_storage::{
    models::{
        conversation::{
          Conversation, ConversationHierarchy, ConversationMessage, ConversationsOverview, UserCreatedConversation,
          UserCreatedConversationMessage
        },
        user::User
    },
    repositories::conversation_repository::{
        create_conversation, create_conversation_message, find_conversation, find_user_conversations
    }
  };
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;
use arcadia_common::error::Result;

#[utoipa::path(
    post,
    path = "/api/conversation",
    responses(
        (status = 200, description = "Successfully created the conversation and first message", body=Conversation),
    )
)]
pub async fn add_conversation(
    mut conversation: web::Json<UserCreatedConversation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // creates a conversation and the first message, empty conversations should not be allowed
    let conversation = create_conversation(arc.pool.borrow(), &mut conversation, current_user.id).await?;

    Ok(HttpResponse::Created().json(conversation))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetConversationQuery {
    id: i64,
}

#[utoipa::path(
    get,
    params(GetConversationQuery),
    path = "/api/conversation",
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn get_conversation(
    query: web::Query<GetConversationQuery>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let conversation_with_messages =
        find_conversation(arc.pool.borrow(), query.id, current_user_id.0, true).await?;

    Ok(HttpResponse::Ok().json(conversation_with_messages))
}

#[utoipa::path(
    get,
    path = "/api/conversations",
    responses(
        (status = 200, description = "Found the conversations and some of their metadata", body=ConversationsOverview),
    )
)]
pub async fn get_user_conversations(
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let conversations = find_user_conversations(arc.pool.borrow(), current_user_id.0).await?;

    Ok(HttpResponse::Ok().json(json!({"conversations": conversations})))
}

#[utoipa::path(
    post,
    path = "/api/conversation/message",
    responses(
        (status = 200, description = "Successfully created the conversation's message", body=ConversationMessage),
    )
)]
pub async fn add_conversation_message(
    message: web::Json<UserCreatedConversationMessage>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let message = create_conversation_message(arc.pool.borrow(), &message, current_user.id).await?;

    Ok(HttpResponse::Created().json(message))
}
