use std::vec;

use chrono::Utc;
use uuid::Uuid;
use yew::prelude::*;

use models::{Author, Conversation, Voice, Message};

const MESSAGE_USER_STYLE: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500";
const MESSAGE_VOICE_STYLE: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-zinc-700";

#[function_component]
pub fn ConversationDisplay() -> Html {
    let voice = Voice {
        id: Uuid::new_v4().to_string(),
        name: "Shaun".to_string(),
        description: "It's Shaun".to_string(),
        prefix: "He's a programmer".to_string(),
        created_at: 1234,
        deleted_at: None
    };

    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        name: "Conversation Name".to_string(),
        user_id: "1234".to_string(),
        voice_id: voice.id.clone(),
        created_at: 1234,
        deleted_at: None
    };

    let messages = vec![
        Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            author: Author::User,
            content: "Hello Bot!".to_string(),
            created_at: Utc::now().timestamp_millis(),
            deleted_at: None
        },
        Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            author: Author::Voice,
            content: "Hello User!".to_string(),
            created_at: Utc::now().timestamp_millis(),
            deleted_at: None
        }
    ];

    html! {
        // <!-- Conversation -->
        <div class="basis-3/4 flex flex-col border-zinc-700 bg-zinc-900 text-white">
            // <!-- Conversation Header -->
            <div class="fixed h-32 w-9/12 top-0 flex flex-col justify-center items-center p-5 border-b bg-zinc-800">
                // <!-- Conversation Info -->
                <div class="">
                    <h2 class="text-2xl">{conversation.name}</h2>
                </div>
                // <!-- Voice Info -->
                <div class="">
                    <div class="w-11 inline-flex p-2 mr-1 rounded-full justify-center font-bold border-2 bg-green-500">
                        {Voice::initials(&voice)}
                    </div>
                    <span>{voice.name}</span>
                </div>
            </div>

            // <!-- Conversation Messages -->
            <div class="pt-36 pb-24 h-screen flex flex-col overflow-y-auto p-5">
                {
                    messages.into_iter().map(|msg| {
                        let style = match msg.author {
                            Author::User => MESSAGE_USER_STYLE,
                            Author::Voice => MESSAGE_VOICE_STYLE,
                        };
                        html! {
                            <div class={style}>
                               {msg.content}
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            // <!-- Conversation Input -->
            <div class="h-24 w-9/12 fixed bottom-0 flex justify-center items-center p-5 border-t bg-zinc-900 border-zinc-700">
                <form class="w-full flex justify-center items-center gap-4">
                    <input class="w-2/3 p-4 border rounded-full input-field bg-zinc-700 border-zinc-700 text-white" type="text" placeholder="Ask a question!" />
                    <button class="h-full p-4 rounded-full cursor-pointer bg-green-700 hover:bg-green-600 text-white" type="submit">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
                    </svg>
                    </button>
                </form>
            </div>
        </div>
    }
}
