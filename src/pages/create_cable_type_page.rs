use std::ops::Deref;

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::models::cable_type_model::*;
use crate::store::State;

#[function_component]
pub fn create_cable_type_page() -> Html {
	let (state, dispatch) = use_store::<State>();
	let form = use_state(|| NewCableType::default());
	let navigator = use_navigator().unwrap();

	let name_input_ref = NodeRef::default();
	let gender_input_ref = NodeRef::default();
	let image_input_ref = NodeRef::default();

	let cloned_form: UseStateHandle<NewCableType> = form.clone();
	let handle_name_input = Callback::from(move |e: String| {
		cloned_form.set(NewCableType {
			name: e,
			..cloned_form.deref().clone()
		});
	});

	let cloned_form: UseStateHandle<NewCableType> = form.clone();
	let handle_gender_input = Callback::from(move |e: Gender| {
		cloned_form.set(NewCableType {
			cable_gender: e,
			..cloned_form.deref().clone()
		});
	});

	let cloned_form: UseStateHandle<NewCableType> = form.clone();
	let handle_image_input = Callback::from(move |e: Vec<u8>| {
		cloned_form.set(NewCableType {
			image: e,
			..cloned_form.deref().clone()
		});
	});

	html! {

	}
}