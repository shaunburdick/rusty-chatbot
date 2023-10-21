use std::vec;

use uuid::Uuid;
use leptos::{component, IntoView, view};

use models::{Conversation, Voice};

#[component]
pub fn ConversationItem(
    conversation: Conversation,
    voice: Voice
) -> impl IntoView {
    view! {
        // <!-- Conversation List Item -->
        <div class="p-5 border-t border-b cursor-pointer overflow-hidden border-slate-500 hover:bg-slate-600">
            <div class="w-11 inline-flex p-2 mr-1 rounded-full justify-center font-bold border-2 bg-green-500">
            {Voice::initials(&voice)}
            </div>
            <p class="text-ellipsis overflow-hidden">{conversation.name}</p>
        </div>
    }
}

#[component]
pub fn SidebarDisplay() -> impl IntoView {
    let voices = vec![
            Voice {
            id: Uuid::new_v4().to_string(),
            name: "Shaun".to_string(),
            description: "It's Shaun".to_string(),
            prefix: "He's a programmer".to_string(),
            created_at: 1234,
            deleted_at: None
        },
    ];

    let conversations = vec![
        Conversation {
            id: Uuid::new_v4().to_string(),
            name: "Conversation 1".to_string(),
            user_id: "1234".to_string(),
            voice_id: voices.first().unwrap().id.clone(),
            created_at: 1234,
            deleted_at: None
        },
        Conversation {
            id: Uuid::new_v4().to_string(),
            name: "Conversation 2".to_string(),
            user_id: "1234".to_string(),
            voice_id: voices.first().unwrap().id.clone(),
            created_at: 1234,
            deleted_at: None
        },
        Conversation {
            id: Uuid::new_v4().to_string(),
            name: "Conversation 3".to_string(),
            user_id: "1234".to_string(),
            voice_id: voices.first().unwrap().id.clone(),
            created_at: 1234,
            deleted_at: None
        },
    ];

    view! {
        // <!-- Sidebar -->
        <div class="basis-1/4 flex flex-col bg-slate-700 text-white">
            // <!-- Title -->
            <div class="fixed top-0 h-32 w-3/12 p-5 flex justify-center items-center border-b bg-slate-700">
                <h2 class="text-2xl">{"Conversations"}</h2>
            </div>

            // <!-- Conversation List -->
            <div class="pt-32 pb-24 h-screen overflow-y-auto">
                {
                    conversations.into_iter().map(|conversation| {
                        let voice = voices.first().unwrap();
                        view! {
                            <ConversationItem conversation voice=voice.clone() />
                        }
                    }).collect::<Vec<_>>()
                }

            </div>

            // <!-- New Conversation Button -->
            <div class="fixed bottom-0 p-5">
                <span class="p-3 text-3xl rounded-2xl hover:cursor-pointer bg-green-700 hover:bg-green-600 ">
                    {"+"}
                </span>
            </div>
        </div>
    }
}
