mod components;

use leptos::{component, IntoView, view};

use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};
use super::store::ChatStore;

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
