mod components;

use leptos::{component, IntoView, view};

use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
