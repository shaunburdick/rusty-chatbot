
use std::collections::HashMap;

use models::{Voice, Conversation, Message};
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct ChatStore {
    voices: HashMap<String, Voice>,
    conversations: HashMap<String, Conversation>,
    messages: Vec<Message>,
}
