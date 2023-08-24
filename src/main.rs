use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use wasm_bindgen_futures::spawn_local;

use crate::components::{alert::AlertComponent, spinner::SpinnerComponent};
use crate::router::*;
use crate::store::Store;
use crate::server_requests::{cable_requests::get_cables, cable_type_requests::get_cable_types};

mod models;
mod server_requests;
mod store;
mod components;
mod pages;
mod router;

#[function_component]
fn App() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let cloned_dispatch = dispatch.clone();
    spawn_local(async move {
        let cable_types = get_cable_types().await;
        match cable_types {
            Ok(cable_types) => {
                crate::store::set_cable_types(cable_types, cloned_dispatch.clone());
            },
            Err(error) => {
                crate::store::set_show_alert(error.to_string(), cloned_dispatch.clone());
            }
        }

        let cables = get_cables().await;
        match cables {
            Ok(cables) => {
                crate::store::set_cables(cables, cloned_dispatch);
            },
            Err(error) => {
                crate::store::set_show_alert(error.to_string(), cloned_dispatch);
            }
        }
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            if store.alert_input.show_alert {
                <AlertComponent message={store.alert_input.alert_message.clone()}  delay_ms=5000/>
            }            
            if store.loading {
                <div class="pt-4 pl-2 top-[5.5rem] fixed">
                    <SpinnerComponent width={Some("1.5rem")} height={Some("1.5rem")} color="text-ct-yellow-600" />
                </div>
            }
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}