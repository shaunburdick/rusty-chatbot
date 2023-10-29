use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, Default)]
/// Represents a conversation with a voice and a user
pub struct Conversation {
    /// ID of the conversation
    pub id: String,

    /// ID of the user involved in the conversation
    pub user_id: String,

    /// A name for the conversation
    pub name: String,

    /// The id of the voice used. Reference to Voice.id
    pub voice_id: String,

    /// Unix Timestamp of when the conversation was created
    pub created_at: i64,

    /// Unix Timestamp of when the conversation was deleted
    pub deleted_at: Option<i64>,
}

impl Conversation {
    /// Create a new Conversation that auto-generates the ID and created_at timestamp
    pub fn new(user_id: String, name: String, voice_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            name,
            voice_id,
            created_at: Utc::now().timestamp(),
            deleted_at: None,
        }
    }
}
