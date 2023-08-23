use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use reqwest::Client;
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
use crate::components::{form_input::FormInputComponent, form_select::FormSelectComponent, loading_button::LoadingButtonComponent, modal::ModalComponent};
use crate::pages::cable_type_page::CableTypePage;

#[derive(Clone, Copy)]
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

fn get_button_callback(field: Field, cloned_form: UseStateHandle<NewCable>) -> Callback<MouseEvent> {
	Callback::from(move |event: MouseEvent| {
		let target = event.target().unwrap();
		let value = target.unchecked_into::<HtmlInputElement>().value();
		get_input_callback(field, cloned_form.clone()).emit(value);
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

	let end_a_input_ref = NodeRef::default();
	let end_b_input_ref = NodeRef::default();
	let cable_length_input_ref = NodeRef::default();

	let handle_end_a_input = get_button_callback(Field::EndA, form.clone());
	let handle_end_b_input = get_button_callback(Field::EndB, form.clone());
	let handle_cable_length_input = get_input_callback(Field::CableLength, form.clone());

	let on_submit = {
		let cloned_client = store.as_ref().client.clone();
		let cloned_form = form.clone();
		let cloned_validation_errors = validation_errors.clone();
		let cloned_navigator = navigator.clone();
		let cloned_dispatch = dispatch.clone();

		Callback::from(move |event: SubmitEvent| {
			event.prevent_default();

			let client = cloned_client.clone();
			let dispatch = cloned_dispatch.clone();
			let navigator = cloned_navigator.clone();
			let form = cloned_form.clone();
			let validation_errors = cloned_validation_errors.clone();

			spawn_local(async move {
				match form.validate() {
					Ok(_) => {
						let data = form.deref().clone();
						let response = create_cable(&client, &data).await;
						match response {
							Ok(_) => {
								set_page_loading(true, dispatch.clone());
								todo!("go back to the cable list page");
							},
							Err(err) => {
								set_page_loading(false, dispatch.clone());
								set_show_alert(err.to_string(), dispatch);
							}
						}
					},
					Err(errors) => {
						validation_errors.set(Rc::new(RefCell::new(errors)));
					}
				}
			})
		})
	};

	html! {
		<>
			<ModalComponent host_id="end_a_modal">
				<>
					<h1>{"Select Cable Type for End A"}</h1>
					<CableTypePage on_click={handle_end_a_input}/>
				</>
			</ModalComponent>

			<ModalComponent host_id="end_b_modal">
				<>
					<h1>{"Select Cable Type for End B"}</h1>
					<CableTypePage on_click={handle_end_b_input}/>
				</>
			</ModalComponent>

			<div>
				<h1>{"Create Cable"}</h1>

				<form onsubmit={on_submit}>
					<button class="w3-button" data-toggle="modal" data-target="#end_a_modal">
						{"End A"}
					</button>
					<button class="w3-button" data-toggle="modal" data-target="#end_b_modal">
						{"End B"}
					</button>
					<FormInputComponent
						label="Cable Length"
						name="cable_length"
						input_ref={cable_length_input_ref.clone()}
						handle_onchange={handle_cable_length_input}
						errors={&*validation_errors}
					/>
				</form>
			</div>
		</>
	}
}