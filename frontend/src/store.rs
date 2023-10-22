
use std::collections::HashMap;
use leptos::{SignalUpdate, SignalGet};
use leptos_use::storage::use_storage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use models::{Voice, Conversation, Message};

#[derive(PartialEq, Eq, Serialize, Default, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub id: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ChatStore {
    voices: HashMap<String, Voice>,
    conversations: HashMap<String, Conversation>,
    messages: Vec<Message>,
    user_config: UserConfig,
}

impl ChatStore {
    pub fn new() -> ChatStore {
        ChatStore {
            voices: HashMap::new(),
            conversations: HashMap::new(),
            messages: Vec::new(),
            user_config: UserConfig::default()
        }
    }

    pub async fn init(&mut self) {
        self.voices = Self::init_voices().await;
        self.user_config = Self::init_user_config();
    }

    async fn init_voices() -> HashMap<String, Voice> {
        todo!();
    }

    fn init_user_config() -> UserConfig {
        // Get user id, if it doesn't exist, create a new user id and store it
        let default_user_config = UserConfig {
            id: Uuid::new_v4().to_string(),
        };

        let (
            user_config,
            update_user_config,
            _
        ) = use_storage("rusty_chat_user_config", default_user_config);

        update_user_config.update(|uc| {uc.id = uc.id.clone()} );

        user_config.get()
    }
}
