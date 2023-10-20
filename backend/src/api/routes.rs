use actix_web::http::StatusCode;
use actix_web::{get, post, put, web, HttpResponse, delete};
use models::{Voice, Conversation, Message};
use serde::Deserialize;

use crate::db::DB;
use crate::api::error::HttpError;
use crate::api::response::JsonApiResponse;

#[get("/voices")]
async fn voices_find_all(db: web::Data<DB>) -> Result<HttpResponse, HttpError> {
    // An empty response is a valid response, so unwrap to an empty vec instead of 404 error
    let voices = db.get_voices(false).await.unwrap_or(Vec::new());
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(voices, None)))
}

#[get("/voices/{voice_id}")]
async fn voices_find_one(db: web::Data<DB>, path: web::Path<String>) -> Result<HttpResponse, HttpError> {
    let voice_id = path.into_inner();
    let voice = db.get_voice(&voice_id).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![voice], None)))
}

#[post("/voices")]
async fn voices_new(db: web::Data<DB>, new_voice: web::Json<Voice>) -> Result<HttpResponse, HttpError> {
    db.save_voice(&new_voice).await?;
    Ok(HttpResponse::Created().json(JsonApiResponse::success(vec![new_voice], None)))
}

#[put("/voices/{voice_id}")]
async fn voices_save(db: web::Data<DB>, path: web::Path<String>, voice: web::Json<Voice>) -> Result<HttpResponse, HttpError> {
    // Check that the ID of the object matches the ID of the path
    let path_id = path.into_inner();
    if path_id != voice.id {
        return Err(HttpError::new(
            StatusCode::BAD_REQUEST.as_u16(),
            format!("Path id {} does not match object id {}", path_id, voice.id)
        ));
    }

    db.save_voice(&voice).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![voice], None)))
}

#[derive(Deserialize)]
struct ConversationsQuery {
    user_id: String,
}

#[get("/conversations")]
async fn conversations_find_all(db: web::Data<DB>, query_params: web::Query<ConversationsQuery>) -> Result<HttpResponse, HttpError> {
    // An empty response is a valid response, so unwrap to an empty vec instead of 404 error
    let conversations = db.get_conversations(&query_params.user_id, false).await.unwrap_or(Vec::new());
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(conversations, None)))
}

#[get("/conversations/{conversation_id}")]
async fn conversations_find_one(db: web::Data<DB>, path: web::Path<String>) -> Result<HttpResponse, HttpError> {
    let conversation_id = path.into_inner();
    let conversation = db.get_conversation(&conversation_id).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![conversation], None)))
}

#[post("/conversations")]
async fn conversations_new(db: web::Data<DB>, new_conversation: web::Json<Conversation>) -> Result<HttpResponse, HttpError> {
    db.save_conversation(&new_conversation).await?;
    Ok(HttpResponse::Created().json(JsonApiResponse::success(vec![new_conversation], None)))
}

#[put("/conversations/{conversation_id}")]
async fn conversations_save(db: web::Data<DB>, path: web::Path<String>, conversation: web::Json<Conversation>) -> Result<HttpResponse, HttpError> {
    // Check that the ID of the object matches the ID of the path
    let path_id = path.into_inner();
    if path_id != conversation.id {
        return Err(HttpError::new(
            StatusCode::BAD_REQUEST.as_u16(),
            format!("Path id {} does not match object id {}", path_id, conversation.id)
        ));
    }

    db.save_conversation(&conversation).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![conversation], None)))
}

#[delete("/conversations/{conversation_id}")]
async fn conversations_delete(db: web::Data<DB>, path: web::Path<String>) -> Result<HttpResponse, HttpError> {
    let conversation_id = path.into_inner();
    db.delete_conversation(&conversation_id).await?;
    db.delete_messages_by_conversation(&conversation_id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
struct MessagesQuery {
    conversation_id: String,
}

#[get("/messages")]
async fn messages_find_all(db: web::Data<DB>, query_params: web::Query<MessagesQuery>) -> Result<HttpResponse, HttpError> {
    // An empty response is a valid response, so unwrap to an empty vec instead of 404 error
    let messages = db.get_messages(&query_params.conversation_id, false).await.unwrap_or(Vec::new());
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(messages, None)))
}

#[get("/messages/{message_id}")]
async fn messages_find_one(db: web::Data<DB>, path: web::Path<String>) -> Result<HttpResponse, HttpError> {
    let message_id = path.into_inner();
    let message = db.get_message(&message_id).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![message], None)))
}

#[post("/messages")]
async fn messages_new(db: web::Data<DB>, new_message: web::Json<Message>) -> Result<HttpResponse, HttpError> {
    db.save_message(&new_message).await?;
    Ok(HttpResponse::Created().json(JsonApiResponse::success(vec![new_message], None)))
}

#[put("/messages/{message_id}")]
async fn messages_save(db: web::Data<DB>, path: web::Path<String>, message: web::Json<Message>) -> Result<HttpResponse, HttpError> {
    // Check that the ID of the object matches the ID of the path
    let path_id = path.into_inner();
    if path_id != message.id {
        return Err(HttpError::new(
            StatusCode::BAD_REQUEST.as_u16(),
            format!("Path id {} does not match object id {}", path_id, message.id)
        ));
    }

    db.save_message(&message).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(vec![message], None)))
}

/// Populate all the routes onto an App Service Configuration
pub fn init_routes(config: &mut web::ServiceConfig) {
    // Voices
    config.service(voices_find_all);
    config.service(voices_find_one);
    config.service(voices_new);
    config.service(voices_save);

    // Conversations
    config.service(conversations_find_all);
    config.service(conversations_find_one);
    config.service(conversations_new);
    config.service(conversations_save);
    config.service(conversations_delete);

    // Messages
    config.service(messages_find_all);
    config.service(messages_find_one);
    config.service(messages_new);
    config.service(messages_save);
}
