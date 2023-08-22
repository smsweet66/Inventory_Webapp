use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    pub input_type: Option<String>,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub errors: Rc<RefCell<ValidationErrors>>,
}

#[function_component]
pub fn FormInputComponent(props: &FormInputProps) -> Html {
    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(|| "text".to_string());
    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    html! {
		<div>
			<label html={props.name.clone()} class="block text-ct-blue-600 mb-3">
				{props.label.clone()}
			</label>
			<input
				type={input_type}
				placeholder=""
				class="block w-full rounded-2xl appearance-none focus:outline-none py-2 px-4"
				ref={props.input_ref.clone()}
				onchange={onchange}
			/>
			<span class="text-red-500 text-xs pt-1 block">
				{error_message}
			</span>
		</div>
    }
}