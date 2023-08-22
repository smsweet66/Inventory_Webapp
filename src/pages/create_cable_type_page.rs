use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

use crate::models::cable_type_model::*;
use crate::server_requests::cable_type_requests::create_cable_type;
use crate::store::{State, set_page_loading};

enum Field {
	Name,
	Gender,
	Image
}

fn get_input_callback(field: Field, cloned_form: UseStateHandle<NewCableType>,
) -> Callback<Event> {
    let callback = Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match &field {
            Field::Name => data.name = value,
			Field::Gender => data.cable_gender = Gender::from_string(&value),
			Field::Image => data.image = value.into_bytes(),
		}
        cloned_form.set(data);
    });

	Callback::from(move |event: Event| {
		let target = event.target().unwrap();
		let value = target.unchecked_into::<HtmlInputElement>().value();
		callback.emit(value);
	})
}

/// Page for creating a new cable type
/// has a text input for the name, a drop down list for the gender,
/// and a file input for the image
#[function_component]
pub fn create_cable_type_page() -> Html {
	let (state, dispatch) = use_store::<State>();
	let form = use_state(|| NewCableType::default());
	let navigator = use_navigator().unwrap();

	let name_input_ref = NodeRef::default();
	let gender_input_ref = NodeRef::default();
	let image_input_ref = NodeRef::default();

	let handle_name_input = get_input_callback(Field::Name, form.clone());
	let handle_gender_input = get_input_callback(Field::Gender, form.clone());
	let handle_image_input = get_input_callback(Field::Image, form.clone());

	let on_submit = {
		let cloned_client = state.as_ref().client.clone();
		let cloned_form = form.clone();
		let cloned_navigator = navigator.clone();
		let cloned_dispatch = dispatch.clone();

		Callback::from(move |event: SubmitEvent| {
			event.prevent_default();

			let client = cloned_client.clone();
			let dispatch = cloned_dispatch.clone();
			let navigator = cloned_navigator.clone();
			let form = cloned_form.clone();

			spawn_local(async move {
				let data = form.deref().clone();

				set_page_loading(true, dispatch.clone());

				let res = create_cable_type(&client, &data).await;
				match res {
					Ok(_) => {
						set_page_loading(false, dispatch);
						todo!("navigate to cable type page");
					}
					Err(_) => {
						set_page_loading(false, dispatch);
						todo!("display error message");
					}
				}
			});
		})
	};

	html! {
		<div>
			<h1>{ "Create Cable Type" }</h1>

			<form onsubmit={on_submit}>
				<label for="name">{ "Name" }</label>
				<input name="name" id="name"
					ref={name_input_ref}
					type="text"
					placeholder="Name"
					onchange={handle_name_input}
				/>
				
				<label for="gender">{ "Cable Gender" }</label>
				<select name="gender" id="gender" ref={gender_input_ref} onchange={handle_gender_input}>
					<option value="Male">{ "Male" }</option>
					<option value="Female">{ "Female" }</option>
				</select>

				<label for="image">{ "Image" }</label>
				<input name="image" id="image"
					ref={image_input_ref}
					type="image"
					placeholder="Image"
					onchange={handle_image_input}
				/>
			</form>
		</div>
	}
}