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
use gloo::utils::document;

use crate::models::cable_model::*;
use crate::server_requests::cable_requests::create_cable;
use crate::store::{Store, set_page_loading, set_show_alert, add_cable};
use crate::components::{form_input::FormInputComponent, modal::*, loading_button::LoadingButtonComponent};
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
		let id_field = target.unchecked_into::<HtmlInputElement>().id();
		let value = id_field.split("_").collect::<Vec<&str>>()[2].to_string();
		get_input_callback(field, cloned_form.clone()).emit(value);
	})
}

fn open_modal(id: String) -> Callback<MouseEvent>{
	Callback::from(move |_| {
		let modal = document()
			.get_element_by_id(&id)
			.expect(format!("Could not find element with id {}", id).as_str());
		modal.set_attribute("style", "display: block;").unwrap();
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

	let cable_length_input_ref = NodeRef::default();

	let handle_end_a_input = get_button_callback(Field::EndA, form.clone());
	let handle_end_b_input = get_button_callback(Field::EndB, form.clone());
	let handle_cable_length_input = get_input_callback(Field::CableLength, form.clone());

	let on_submit = {
		let cloned_form = form.clone();
		let cloned_validation_errors = validation_errors.clone();
		let cloned_navigator = navigator.clone();
		let cloned_dispatch = dispatch.clone();

		Callback::from(move |event: SubmitEvent| {
			event.prevent_default();

			let dispatch = cloned_dispatch.clone();
			let navigator = cloned_navigator.clone();
			let form = cloned_form.clone();
			let validation_errors = cloned_validation_errors.clone();

			spawn_local(async move {
				match form.validate() {
					Ok(_) => {
						set_page_loading(true, dispatch.clone());
						let data = form.deref().clone();
						let response = create_cable(&data).await;
						match response {
							Ok(cable) => {
								set_page_loading(false, dispatch.clone());
								add_cable(cable, dispatch);
								navigator.go(-1);
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
			<ModalHostComponent id="end_a_modal" header="Select Cable Type for End A">
				<ModalInnerComponent host_id="end_a_modal">
					<CableTypePage on_click={handle_end_a_input}/>
				</ModalInnerComponent>
			</ModalHostComponent>

			<ModalHostComponent id="end_b_modal" header="Select Cable Type for End B">
				<ModalInnerComponent host_id="end_b_modal">
					<CableTypePage on_click={handle_end_b_input}/>
				</ModalInnerComponent>
			</ModalHostComponent>

			<div>
				<h1>{"Create Cable"}</h1>

				<form onsubmit={on_submit}>
					<button onclick={open_modal("end_a_modal".to_owned())} class="block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						{"Select Cable Type For End A"}
					</button>
					<button onclick={open_modal("end_b_modal".to_owned())} class="block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
						{"Select Cable Type For End B"}
					</button>
					<FormInputComponent
						label="Cable Length"
						name="cable_length"
						input_ref={cable_length_input_ref.clone()}
						handle_onchange={handle_cable_length_input}
						errors={&*validation_errors}
					/>
					<LoadingButtonComponent
						loading={store.loading}
						text_color={Some("text-ct-blue-600".to_string())}
					>
						{"Create Cable"}
					</LoadingButtonComponent>
				</form>
			</div>
		</>
	}
}