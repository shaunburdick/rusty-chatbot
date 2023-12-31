use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// A voice is a description of the responder in the conversation
pub struct Voice {
    /// ID of the voice
    pub id: String,

    /// A name for the voice
    pub name: String,

    /// A description of the voice
    pub description: String,

    /// The LLM prefix description of the voice, used in the prompt
    pub prefix: String,

    /// Unix Timestamp of when the voice was created
    pub created_at: i64,

    /// Unix Timestamp of when the voice was deleted
    pub deleted_at: Option<i64>,
}

impl Voice {
    /// Create a new Voice that auto-generates the ID and created_at timestamp
    pub fn new(name: String, description: String, prefix: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            prefix,
            created_at: Utc::now().timestamp(),
            deleted_at: None,
        }
    }

    /// Generate a set of initials from a voice
    /// Will only generate a 2 character string
    pub fn initials(voice: &Voice) -> String {
        voice
            .name
            .splitn(2, ' ')
            .map(|word| word.chars().nth(0).unwrap_or(' '))
            .collect::<String>()
    }
}
