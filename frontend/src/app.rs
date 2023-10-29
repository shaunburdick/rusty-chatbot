mod components;

use leptos::{component, provide_context, view, IntoView, Resource};
use leptos_router::{Route, Router, Routes};

use super::store::ChatStore;
use components::{
    conversation::ConversationCreate, conversation::ConversationDisplay, sidebar::SidebarDisplay,
    voice::VoiceListDisplay,
};

#[component]
pub fn App() -> impl IntoView {
    let store = Resource::new(
        || (),
        |_| async move {
            let mut store = ChatStore::new();
            store.init().await;

            store
        },
    );

    provide_context(store);

    view! {
        <Router>
            <div class="flex flex-row h-full">
                <div class="basis-1/4 flex flex-col bg-slate-700 text-white">
                    <SidebarDisplay />
                </div>
                <div class="basis-3/4 flex flex-col border-zinc-700 bg-zinc-900 text-white">
                    <Routes>
                        <Route path="/conversations" view=|| view! { <p>"Conversation List"</p> } />
                        <Route path="/conversations/new" view=ConversationCreate />
                        <Route path="/conversations/:id" view=ConversationDisplay />
                        <Route path="/conversations/:id/edit" view=|| view! { <p>"Edit Conversation"</p> }/>
                        <Route path="/voices" view=VoiceListDisplay />
                        <Route path="/voices/new" view=|| view! { <p>"New Voice"</p> }  />
                        <Route path="/voices/:id" view=|| view! { <p>"Voice View"</p> } />
                        <Route path="/voices/:id/edit" view=|| view! { <p>"Edit View"</p> }/>
                        <Route path="" view=|| view! { <p>"Home View"</p> }/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
