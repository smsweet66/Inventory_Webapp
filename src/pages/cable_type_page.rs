use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::{prelude::*, navigator};

use crate::models::cable_type_model::*;
use crate::router::Route;
use crate::store::Store;
use crate::components::cable_type_component::CableTypeComponent;

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
		let target = event.target().unwrap();
		let value = target.unchecked_into::<HtmlInputElement>().value();
		let id = value.parse::<i32>().unwrap();
		cloned_navigator.push(&Route::UpdateCableTypePage { id });
	}));

	let create_cable = Callback::from(move |_| {
		navigator.push(&Route::CreateCableTypePage);
	});

	html! {
		<div>
			<h1>{"Cable Types"}</h1>
			
			{for cable_types.iter().map(|cable_type| html! {
				<CableTypeComponent data={cable_type.clone()} on_click={&on_click}/>
			})}

			<button onclick={create_cable}>{"Create Cable Type"}</button>
		</div>
	}
}