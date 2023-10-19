mod components;

use yew::prelude::*;

use crate::app::components::{conversation::ConversationDisplay, sidebar::SidebarDisplay};

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="flex flex-row h-full">
            <SidebarDisplay />
            <ConversationDisplay />
        </div>
    }
}
