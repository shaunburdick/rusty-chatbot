mod components;

use leptos::{component, view, IntoView};

use super::store::ChatStore;
use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

#[component]
pub fn App() -> impl IntoView {
    let store = ChatStore::new();

    view! {
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
