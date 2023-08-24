use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component]
pub fn HeaderComponent() -> Html {
	html! {
		<header class="bg-white h-20">
			<nav class="h-full flex justify-between container items-center">
				<ul class="flex items-center gap-4">
					<li>
						<Link<Route> to={Route::CablePage} classes="text-ct-dark-600">{"Cables"}</Link<Route>>
					</li>
					<li>
						<Link<Route> to={Route::CableTypePage} classes="text-ct-dark-600">{"Cable Types"}</Link<Route>>
					</li>
				</ul>
			</nav>
		</header>
	}
}