use std::{cell::RefCell, rc::Rc};

use validator::ValidationErrors;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{HtmlInputElement, FileReader};
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
    let cloned_input_type = input_type.clone();
    let on_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        match cloned_input_type.as_str() {
            "file" => {
                let file = target.files().unwrap().get(0).unwrap();
                log::info!("Selected file: {}", file.name());

                let reader = FileReader::new().unwrap();
                reader.read_as_data_url(&file).unwrap();
                let cloned_handle_onchange = handle_onchange.clone();
                let on_load_end = Closure::wrap(Box::new(move |event: Event| {
                    let handle_onchange = cloned_handle_onchange.clone();
                    let reader = event.target().unwrap().unchecked_into::<FileReader>();
                    let result = reader.result().unwrap().as_string().unwrap();
                    let base64 = result.split(",").collect::<Vec<&str>>()[1];
                    handle_onchange.emit(base64.to_string());
                }) as Box<dyn FnMut(_)>);
                reader.set_onloadend(Some(on_load_end.as_ref().unchecked_ref()));
                on_load_end.forget();
            }
            "text" => {
                let value = target.value();
                handle_onchange.emit(value);
            }
            _ => {}
        }
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
				onchange={on_change}
			/>
			<span class="text-red-500 text-xs pt-1 block">
				{error_message}
			</span>
		</div>
    }
}