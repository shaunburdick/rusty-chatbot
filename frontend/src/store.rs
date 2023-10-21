
use std::collections::HashMap;

use models::{Voice, Conversation, Message};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ChatStore {
    voices: HashMap<String, Voice>,
    conversations: HashMap<String, Conversation>,
    messages: Vec<Message>,
}
