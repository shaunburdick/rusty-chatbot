mod components;

use leptos::{component, IntoView, view};
use serde::Deserialize;

use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

#[derive(Debug, Deserialize)]
pub struct JsonApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<T>>,
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
