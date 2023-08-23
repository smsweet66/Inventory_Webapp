use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{cable_type_page::CableTypePage, create_cable_type_page::CreateCableTypePage, update_cable_type_page::UpdateCableTypePage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/cabletype")]
	CableTypePage,
	#[at("/createcabletype")]
	CreateCableTypePage,
	#[at("/updatecabletype/:id")]
	UpdateCableTypePage { id: i32 },
}

pub fn switch(routes: Route) -> Html {
	match routes {
		Route::CableTypePage => html! { <CableTypePage /> },
		Route::CreateCableTypePage => html! { <CreateCableTypePage /> },
		Route::UpdateCableTypePage { id } => html! { <UpdateCableTypePage cable_type_id={id}/> },
	}
}