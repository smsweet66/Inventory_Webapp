use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::store::Store;
use crate::components::{cable_component::CableComponent, header::HeaderComponent};

#[function_component]
pub fn CablePage() -> Html {
	let (store, _) = use_store::<Store>();
	let cables = store.cables.clone();
	let navigator = use_navigator().unwrap();
	
	let create_cable = Callback::from(move |_| {
		navigator.push(&Route::CreateCablePage);
	});

	html! {
		<>
			<HeaderComponent />
			<div>
				<h1>{"Cables"}</h1>
				
				<table class="table-auto">
					<thead>
						<tr>
							<th>{"Cable Type A"}</th>
							<th>{"Cable Gender A"}</th>
							<th>{"Cable Type B"}</th>
							<th>{"Cable Gender B"}</th>
							<th>{"Cable Length"}</th>
						</tr>
					</thead>
					<tbody>
						{for cables.iter().map(|cable| html! {
							<CableComponent data={cable.clone()}/>
						})}
					</tbody>
				</table>

				<button onclick={create_cable}>{"Create Cable"}</button>
			</div>
		</>
	}
}