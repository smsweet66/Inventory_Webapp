use yew::prelude::*;

mod models;
mod server_requests;
mod store;
mod components;
mod pages;
mod router;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"Cable Types"}</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}