use std::str::FromStr;

use chrono::Utc;
use serde::Serialize;
use strum::Display;
use uuid::Uuid;

#[derive(PartialEq, Eq, Debug, Clone, Display, Serialize)]
#[strum(serialize_all = "lowercase")]

pub enum Author {
    User,
    Voice
}

impl FromStr for Author {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Author::User),
            "voice" => Ok(Author::Voice),
            _ => Err(format!("Invalid author: {}", s)),
        }
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
/// A message is a bit of text as part of the conversation
pub struct Message {
    /// ID of the message
    pub id: String,

    /// ID of the conversation this message is associated with. Reference to Conversation.id
    pub conversation_id: String,

    /// The author of the message.
    pub author: Author,

    /// The content of the message
    pub content: String,

    /// Unix Timestamp of when the message was created
    pub created_at: i64,

    /// Unix Timestamp of when the message was deleted
    pub deleted_at: Option<i64>,
}

impl Message {
    /// Create a new Message that auto-generates the ID and created_at timestamp
    pub fn new(conversation_id: String, author: Author, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            conversation_id,
            author,
            content,
            created_at: Utc::now().timestamp(),
            deleted_at: None,
        }
    }
}
