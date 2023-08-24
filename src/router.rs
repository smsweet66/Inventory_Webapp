use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{cable_type_page::CableTypePage, create_cable_type_page::CreateCableTypePage, update_cable_type_page::UpdateCableTypePage, cable_page::CablePage, create_cable_page::CreateCablePage, update_cable_page::UpdateCablePage, home_page::HomePage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	HomePage,
	#[at("/cabletype")]
	CableTypePage,
	#[at("/createcabletype")]
	CreateCableTypePage,
	#[at("/updatecabletype/:id")]
	UpdateCableTypePage { id: i32 },
	#[at("/cable")]
	CablePage,
	#[at("/createcable")]
	CreateCablePage,
	#[at("/updatecable/:id")]
	UpdateCablePage { id: i32 },
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::HomePage => html! { <HomePage /> },
		Route::CableTypePage => html! { <CableTypePage /> },
		Route::CreateCableTypePage => html! { <CreateCableTypePage /> },
		Route::UpdateCableTypePage { id } => html! { <UpdateCableTypePage cable_type_id={id}/> },
		Route::CablePage => html! { <CablePage /> },
		Route::CreateCablePage => html! { <CreateCablePage /> },
		Route::UpdateCablePage { id } => html! { <UpdateCablePage cable_id={id}/> },
	}
}