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

use crate::models::cable_type_model::*;
use crate::server_requests::cable_type_requests;
use crate::store::{Store, set_page_loading, set_show_alert, update_cable_type};
use crate::components::{form_input::FormInputComponent, form_select::FormSelectComponent, loading_button::LoadingButtonComponent};

enum Field {
	Name,
	Gender,
	Image
}

fn get_input_callback(field: Field, cloned_form: UseStateHandle<CableType>) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match &field {
            Field::Name => data.name = value,
			Field::Gender => data.cable_gender = Gender::from_string(&value),
			Field::Image => data.image = value.into_bytes(),
		}
        cloned_form.set(data);
    })
}

#[derive(Properties, PartialEq, Clone)]
pub struct UpdateCableTypePageProps {
	pub cable_type_id: i32,
}

/// Page for creating a new cable type
/// has a text input for the name, a drop down list for the gender,
/// and a file input for the image
#[function_component]
pub fn UpdateCableTypePage(props: &UpdateCableTypePageProps) -> Html {
	let (store, dispatch) = use_store::<Store>();
	let form = use_state(|| store.cable_types.iter().find(|cable_type| cable_type.id == props.cable_type_id).unwrap().clone());
	let navigator = use_navigator().unwrap();
	let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

	let name_input_ref = NodeRef::default();
	let gender_input_ref = NodeRef::default();
	let image_input_ref = NodeRef::default();

	let handle_name_input = get_input_callback(Field::Name, form.clone());
	let handle_gender_input = get_input_callback(Field::Gender, form.clone());
	let handle_image_input = get_input_callback(Field::Image, form.clone());

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

						set_page_loading(true, dispatch.clone());
		
						let res = cable_type_requests::update_cable_type(&client, &data).await;
						match res {
							Ok(cable_type) => {
								set_page_loading(false, dispatch.clone());
								update_cable_type(cable_type, dispatch);
								navigator.go(-1);
							}
							Err(e) => {
								set_page_loading(false, dispatch.clone());
								set_show_alert(e.to_string(), dispatch);
							}
						}
					}
					Err(errors) => {
						validation_errors.set(Rc::new(RefCell::new(errors)));
					}
				}
			});
		})
	};

	html! {
		<div>
			<h1>{ "Update Cable Type" }</h1>

			<form onsubmit={on_submit}>
				<FormInputComponent
					label="Cable Type"
					name="name"
					input_ref={name_input_ref.clone()}
					handle_onchange={handle_name_input}
					errors={&*validation_errors}
				/>
				<FormSelectComponent
					label="Cable Gender"
					name="gender"
					input_ref={gender_input_ref.clone()}
					handle_onchange={handle_gender_input}
					errors={&*validation_errors}
					options={Gender::to_string_vec()}
				/>
				<FormInputComponent
					label="Image"
					name="image"
					input_ref={image_input_ref.clone()}
					handle_onchange={handle_image_input}
					errors={&*validation_errors}
					input_type="file"
				/>
				<LoadingButtonComponent
					loading={store.loading}
					text_color={Some("text-ct-blue-600".to_string())}
				>
					{"Create Cable Type"}
				</LoadingButtonComponent>
			</form>
		</div>
	}
}