use super::spinner::SpinnerComponent;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct LoadingButtonProps {
    pub loading: bool,
    pub btn_color: Option<String>,
    pub text_color: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn LoadingButtonComponent(props: &LoadingButtonProps) -> Html {
	let text_color = props
		.text_color
		.clone()
		.unwrap_or_else(|| "text-white".to_string());
	let btn_color = props
		.btn_color
		.clone()
		.unwrap_or_else(|| "bg-ct-yellow-600".to_string());

	html! {
		<button
			type="submit"
			class={format!(
				"w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center {}",
				if props.loading {"bg-[#ccc]"} else {btn_color.as_str()}
			)}
		>
			if props.loading {
				<div class="flex items-center gap-3">
					<SpinnerComponent />
					<span class="text-slate-500 inline-block">{"Loading..."}</span>
				</div>
			} else {
				<span class={text_color.to_owned()}>{props.children.clone()}</span>
			}
		</button>
	}
}