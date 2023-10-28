mod components;

use leptos::{component, provide_context, view, IntoView, Resource};

use super::store::ChatStore;
use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

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
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
