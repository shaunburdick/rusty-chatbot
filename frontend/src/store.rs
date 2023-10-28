use gloo::{
    net::{http::Request, Error},
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use models::{Conversation, JsonApiResponse, Message, Voice};

#[derive(PartialEq, Eq, Serialize, Default, Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatStore {
    pub voices: HashMap<String, Voice>,
    pub conversations: HashMap<String, Conversation>,
    pub messages: Vec<Message>,
    pub user_config: UserConfig,
}

const LS_USER_ID_KEY: &str = "rusty_chat_user_config";

impl ChatStore {
    /// Create a new ChatStore with default values
    pub fn new() -> ChatStore {
        ChatStore {
            voices: HashMap::new(),
            conversations: HashMap::new(),
            messages: Vec::new(),
            user_config: UserConfig::default(),
        }
    }

    /// Initialize the chat store
    /// This will initialize the user config, then fetch voices and conversations
    pub async fn init(&mut self) {
        self.user_config = Self::init_user_config();
        if let Ok(voices) = Self::get_voices().await {
            self.voices = voices;
        };
        if let Ok(conversations) = Self::get_conversations(self.user_config.id.clone()).await {
            self.conversations = conversations;
        };
    }

    /// Fetch voices from the API
    pub async fn get_voices() -> Result<HashMap<String, Voice>, Error> {
        let resp = Request::get("/api/voices")
            .send()
            .await?
            .json::<JsonApiResponse<Voice>>()
            .await?;

        let mut voice_map = HashMap::new();
        for voice in resp.data.unwrap() {
            voice_map.insert(voice.id.clone(), voice);
        }

        Ok(voice_map)
    }

    /// Fetch conversations from the API
    pub async fn get_conversations(
        user_id: String,
    ) -> Result<HashMap<String, Conversation>, Error> {
        let resp = Request::get("/api/conversations")
            .query([("user_id", user_id)])
            .send()
            .await?
            .json::<JsonApiResponse<Conversation>>()
            .await?;

        let mut conversation_map = HashMap::new();
        for conversation in resp.data.unwrap() {
            conversation_map.insert(conversation.id.clone(), conversation);
        }

        Ok(conversation_map)
    }

    pub async fn get_messages(conversation_id: String) -> Result<Vec<Message>, Error> {
        let resp = Request::get("/api/messages")
            .query([("conversation_id", conversation_id)])
            .send()
            .await?
            .json::<JsonApiResponse<Message>>()
            .await?;

        Ok(match resp.data {
            Some(messages) => messages,
            None => Vec::new(),
        })
    }

    fn init_user_config() -> UserConfig {
        // Get user id, if it doesn't exist, create a new user id and store it
        let default_user_config = UserConfig {
            id: Uuid::new_v4().to_string(),
        };

        match LocalStorage::get::<UserConfig>(LS_USER_ID_KEY) {
            Ok(uc) => uc,
            Err(_) => {
                let _ = LocalStorage::set(LS_USER_ID_KEY, default_user_config.clone());
                default_user_config
            }
        }
    }
}
