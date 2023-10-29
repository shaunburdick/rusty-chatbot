use std::vec;

use chrono::Utc;
use leptos::{
    component, create_action,
    ev::SubmitEvent,
    html::{Input, Select},
    leptos_dom::logging::console_error,
    use_context, view, IntoView, NodeRef, Resource, SignalGet, SignalWith,
};
use leptos_router::{use_navigate, use_params_map, Route};
use uuid::Uuid;

use models::{Author, Conversation, Message, Voice};

use crate::store::ChatStore;

const MESSAGE_USER_STYLE: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500";
const MESSAGE_VOICE_STYLE: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-zinc-700";

#[component(transparent)]
pub fn ConversationRoutes() -> impl IntoView {
    view! {
        <Route path="/conversations" view=|| view! { <p>"Main View"</p> } />
        <Route path="/conversations/:id" view=ConversationDisplay />
        <Route path="/conversations/:id/edit" view=|| view! { <p>"Edit View"</p> }/>
    }
}

#[component]
pub fn MessageItem(message: Message) -> impl IntoView {
    let style = match message.author {
        Author::User => MESSAGE_USER_STYLE,
        Author::Voice => MESSAGE_VOICE_STYLE,
    };
    view! {
        <div class={style}>
            {message.content}
        </div>
    }
}

#[component]
pub fn ConversationDisplay() -> impl IntoView {
    let params = use_params_map();
    let store = use_context::<Resource<(), ChatStore>>().expect("to have store set");
    let conversation_id =
        move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let voice = Voice {
        id: Uuid::new_v4().to_string(),
        name: "Shaun".to_string(),
        description: "It's Shaun".to_string(),
        prefix: "He's a programmer".to_string(),
        created_at: 1234,
        deleted_at: None,
    };

    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        name: "Conversation Name".to_string(),
        user_id: "1234".to_string(),
        voice_id: voice.id.clone(),
        created_at: 1234,
        deleted_at: None,
    };

    let messages = vec![
        Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            author: Author::User,
            content: "Hello Bot!".to_string(),
            created_at: Utc::now().timestamp_millis(),
            deleted_at: None,
        },
        Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            author: Author::Voice,
            content: "Hello User!".to_string(),
            created_at: Utc::now().timestamp_millis(),
            deleted_at: None,
        },
    ];

    view! {
        // <!-- Conversation Header -->
        <div class="fixed h-32 w-9/12 top-0 flex flex-col justify-center items-center p-5 border-b bg-zinc-800">
            // <!-- Conversation Info -->
            <div class="">
                <h2 class="text-2xl">{format!("{}({})", conversation.name, conversation_id())}</h2>
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
                messages.into_iter().map(|message| {
                    view! {
                        <MessageItem message />
                    }
                }).collect::<Vec<_>>()
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
    }
}

#[component]
pub fn ConversationCreate() -> impl IntoView {
    let store = use_context::<Resource<(), ChatStore>>().expect("to have store set");

    let name_element: NodeRef<Input> = NodeRef::new();
    let voice_element: NodeRef<Select> = NodeRef::new();

    let create_conversation = create_action(move |input: &(String, String, String)| {
        let (user_id, name, voice_id) = input.to_owned();
        async move {
            match ChatStore::new_conversation(user_id, name, voice_id).await {
                Ok(conversation) => {
                    // add conversation to the store
                    store
                        .get()
                        .expect("store to exist")
                        .conversations
                        .insert(conversation.id.clone(), conversation);
                    let navigate = use_navigate();
                    navigate("/conversations", Default::default());
                }
                Err(_) => {
                    console_error("Could not create new conversation");
                }
            };
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let store = store.get().expect("store to exist");

        let user = store.user_config;
        let name = name_element.get().expect("name to exist").value();
        let voice_id = voice_element.get().expect("voice_id to exist").value();

        create_conversation.dispatch((user.id, name, voice_id));
    };

    view! {
        // Conversation Header
        <div class="fixed h-32 w-9/12 top-0 flex flex-col justify-center items-center p-5 border-b bg-zinc-800">
            // Conversation Info
            <div class="">
                <h2 class="text-2xl">"Create New Conversation"</h2>
            </div>
        </div>

        // New Conversation Form
        <div class="pt-36 pb-24 h-screen flex flex-col overflow-y-auto p-5">
            <form class="" on:submit=on_submit>
                <div>
                    Name: <input type="text" node_ref=name_element class="text-black" />
                </div>
                <div>
                    Voice:
                    <select node_ref=voice_element class="text-black">
                        {move || match store.get() {
                            None => view! { <option>"Loading..."</option> }.into_view(),
                            Some(s) => {
                                s.voices.into_values().map(|voice| {
                                    view! {
                                        <option value={voice.id}>{voice.name}</option>
                                    }
                                }).collect::<Vec<_>>().into_view()
                            }
                        }}
                    </select>
                </div>
                <button class="h-full p-4 rounded- cursor-pointer bg-green-700 hover:bg-green-600 text-white" type="submit">
                    "Submit"
                </button>
            </form>
        </div>
    }
}
