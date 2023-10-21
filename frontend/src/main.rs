mod app;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
