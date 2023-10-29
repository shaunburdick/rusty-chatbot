use leptos::{component, use_context, view, IntoView, Resource, SignalGet};
use leptos_router::Route;

use models::{Author, Voice};

use crate::store::ChatStore;

#[component(transparent)]
pub fn VoiceRoutes() -> impl IntoView {
    view! {
        <Route path="/voices" view=|| view! { <p>"Main View"</p> } />
        <Route path="/voices/:id" view=VoiceListDisplay />
        <Route path="/voices/:id/edit" view=|| view! { <p>"Edit View"</p> }/>
    }
}

#[component]
pub fn VoiceHero(voice: Voice) -> impl IntoView {
    view! {
        <div class="w-11 inline-flex p-2 mr-1 rounded-full justify-center font-bold border-2 bg-green-500">
            {Voice::initials(&voice)}
        </div>
    }
}

#[component]
pub fn VoiceItem(voice: Voice) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="flex flex-col">
                <VoiceHero voice=voice.clone() />
            </div>
            <div class="flex flex-col">
                {voice.name}
            </div>
            <div class="flex flex-col">
                {voice.description}
            </div>
        </div>
    }
}

#[component]
pub fn VoiceListDisplay() -> impl IntoView {
    let store = use_context::<Resource<(), ChatStore>>().expect("to have store set");

    view! {
        // Voice Header
        <div class="fixed h-32 w-9/12 top-0 flex flex-col justify-center items-center p-5 border-b bg-zinc-800">
            <div class="">
                <h2 class="text-2xl">"Voices"</h2>
            </div>
        </div>

        // <!-- Voice List -->
        <div class="pt-36 pb-24 h-screen flex flex-col overflow-y-auto p-5">
            {move || match store.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(s) => {
                    s.voices.into_values().map(|voice| {
                        view! {
                            <VoiceItem voice />
                        }
                    }).collect::<Vec<_>>().into_view()
                }
            }}
        </div>

        // New Voice Button
    }
}
