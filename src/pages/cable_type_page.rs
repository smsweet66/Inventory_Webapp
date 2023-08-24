use web_sys::HtmlElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::store::Store;
use crate::components::{cable_type_component::CableTypeComponent, header::HeaderComponent};

#[derive(Properties, PartialEq, Clone)]
pub struct CableTypePageProps {
	pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn CableTypePage(props: &CableTypePageProps) -> Html {
	let (store, _) = use_store::<Store>();
	let cable_types = store.cable_types.clone();
	let navigator = use_navigator().unwrap();

	let cloned_navigator = navigator.clone();
	let on_click = props.on_click.clone().unwrap_or(Callback::from(move |event: MouseEvent| {
		let target = event.target().unwrap().unchecked_into::<HtmlElement>();
		let id_field = target.id();
		let id = id_field.split("_").collect::<Vec<&str>>()[2].parse::<i32>().unwrap();
		cloned_navigator.push(&Route::UpdateCableTypePage { id });
	}));

	let create_cable = Callback::from(move |_| {
		navigator.push(&Route::CreateCableTypePage);
	});

	html! {
		<>
			<HeaderComponent />
			<div>
				<h1>{"Cable Types"}</h1>
				
				<table class="table-auto">
					<thead>
						<tr>
							<th>{"Cable Type"}</th>
							<th>{"Cable Gender"}</th>
							<th>{"Cable Diagram"}</th>
						</tr>
					</thead>
					<tbody>
						{for cable_types.iter().map(|cable_type| html! {
							<CableTypeComponent data={cable_type.clone()} on_click={&on_click}/>
						})}
					</tbody>
				</table>
				<button onclick={create_cable}>{"Create Cable Type"}</button>
			</div>
		</>
	}
}