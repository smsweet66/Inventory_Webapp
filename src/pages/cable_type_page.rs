use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::models::cable_type_model::*;
use crate::store::Store;
use crate::components::cable_type_component::CableTypeComponent;

#[function_component]
pub fn CableTypePage() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let cable_types = store.cable_types.clone();

	html! {
		<div>
			<h1>{"Cable Types"}</h1>
			
			{for cable_types.iter().map(|cable_type| html! {
				<CableTypeComponent data={cable_type.clone()} />
			})}
		</div>
	}
}