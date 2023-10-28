use leptos::{component, use_context, view, IntoView, Resource, SignalGet};

use models::{Conversation, Voice};

use crate::store::ChatStore;

#[component]
pub fn ConversationItem(conversation: Conversation, voice: Voice) -> impl IntoView {
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
    let store = use_context::<Resource<(), ChatStore>>().expect("to have store set");

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
                    move || match store.get() {
                        None => view ! { <p>"Loading..."</p> }.into_view(),
                        Some(s) => {
                            s.conversations.into_values().map(|conversation| {
                                let voice = s.voices.get(&conversation.voice_id);
                                view! {
                                    <ConversationItem conversation voice=voice.unwrap().clone() />
                                }
                            }).collect::<Vec<_>>().into_view()
                        }
                    }
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
