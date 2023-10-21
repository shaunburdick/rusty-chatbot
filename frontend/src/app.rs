mod components;
use super::store::ChatStore;

use models::Voice;
use serde::Deserialize;
use yew::prelude::*;
use yewdux::prelude::*;

use components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

#[derive(Debug, Deserialize)]
pub struct JsonApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<T>>,
}

#[function_component]
pub fn App() -> Html {
    let (store, dispatch) = use_store::<ChatStore>();

    html! {
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
