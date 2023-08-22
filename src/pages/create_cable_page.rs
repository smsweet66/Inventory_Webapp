use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use validator::{ValidationErrors, Validate};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::models::cable_model::*;
use crate::server_requests::cable_requests::create_cable;
use crate::store::{Store, set_page_loading, set_show_alert};
use crate::components::{form_input::FormInputComponent, form_select::FormSelectComponent, loading_button::LoadingButtonComponent};

enum Field {
	EndA,
	EndB,
	CableLength
}

fn get_input_callback(field: Field, cloned_form: UseStateHandle<NewCable>) -> Callback<String> {
	Callback::from(move |value: String| {
		let mut data = cloned_form.deref().clone();
		match &field {
			Field::EndA => data.end_a = value.parse::<i32>().unwrap_or(0),
			Field::EndB => data.end_b = value.parse::<i32>().unwrap_or(0),
			Field::CableLength => data.cable_length = value.parse::<f32>().unwrap_or(0.0),
		}
		cloned_form.set(data);
	})
}

/// Page for creating a new cable
/// has buttons for selecting the end points and a text input for the cable length
/// Clicking either button will open a modal with a list of end points to choose from
#[function_component]
pub fn CreateCablePage() -> Html {
	let (store, dispatch) = use_store::<Store>();
	let form = use_state(|| NewCable::default());
	let navigator = use_navigator().unwrap();
	let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
	

	html! {

	}
}