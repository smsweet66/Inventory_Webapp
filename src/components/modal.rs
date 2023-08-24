use yew::prelude::*;
use gloo::utils::document;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub host_id: String,
}

#[function_component]
pub fn ModalInnerComponent(props: &ModalProps) -> Html {
    let modal_host = document()
        .get_element_by_id(&props.host_id)
        .expect(format!("Could not find element with id {}", &props.host_id).as_str());

    let on_click = {
        let cloned_modal_host = modal_host.clone();
        Callback::from(move |_| {
            cloned_modal_host.set_attribute("style", "display: none;").unwrap();
        })
    };

    create_portal(
        html!{
            <>
                {for props.children.iter()}
				<div class="flex items-center p-6 space-x-2 border-t border-gray-200 rounded-b dark:border-gray-600">
					<button onclick={on_click} type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Accept"}</button>
				</div>
            </>
        },
        modal_host.into(),
    )
}

#[derive(Properties, PartialEq)]
pub struct ModalHostProps {
	#[prop_or_default]
	pub children: Children,
	pub id: String,
	pub header: String,
}

#[function_component]
pub fn ModalHostComponent(props: &ModalHostProps) -> Html {
	html! {
		<div id={props.id.clone()} tabindex="-1" aria-hidden="true" class="fixed top-0 left-0 right-0 z-50 hidden w-full p-4 overflow-x-hidden overflow-y-auto md:inset-0 h-[calc(100%-1rem)] max-h-full">
			<div class="relative w-full max-w-2xl max-h-full">
				<div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
					<div class="flex items-start justify-between p-4 border-b rounded-t dark:border-gray-600">
						<h3 class="text-xl font-semibold text-gray-900 dark:text-white">
							{props.header.clone()}
						</h3>
					</div>
					<div class="p-6 space-y-6">
						{for props.children.iter()}
					</div>
				</div>
			</div>
		</div>
	}
}