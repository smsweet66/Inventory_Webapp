use crate::components::header::HeaderComponent;
use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
	html! {
		<>
			<HeaderComponent />
			<section class="bg-ct-blue-600 min-h-screen pt-20">
				<div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
					<p class="text-3xl font-semibold">{"Cable Inventory"}</p>
				</div>
			</section>
		</>
	}
}