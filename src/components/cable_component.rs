use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::models::cable_model::Cable;
use crate::store::{Store, self};
use crate::server_requests::cable_requests;

#[derive(PartialEq, Properties)]
pub struct CableProps {
	pub data: Cable,
}

#[function_component]
pub fn CableComponent(props: &CableProps) -> Html {
	let (store, dispatch) = use_store::<Store>();
	let cable_types = store.cable_types.clone();
	let cable_type_a = cable_types.iter().find(|cable_type| cable_type.id == props.data.end_a).unwrap();
	let cable_type_b = cable_types.iter().find(|cable_type| cable_type.id == props.data.end_b).unwrap();

	let delete_cable = {
		let cloned_dispatch = dispatch.clone();
		let cable_id = props.data.id;
		
		Callback::from(move |_: MouseEvent| {
			let dispatch = cloned_dispatch.clone();

			spawn_local(async move {
				let res = cable_requests::delete_cable(cable_id).await;
				match res {
					Ok(_) => {
						store::set_show_alert("Cable deleted successfully".to_string(), dispatch.clone());
						store::delete_cable(cable_id, dispatch);
					},
					Err(_) => {
						store::set_show_alert("Error deleting cable".to_string(), dispatch);
					}
				}
			})
		})
	};

	html! {
		<tr>
			<td>{&cable_type_a.name}</td>
			<td>{&cable_type_a.cable_gender}</td>
			<td>{&cable_type_b.name}</td>
			<td>{&cable_type_b.cable_gender}</td>
			<td>{&props.data.cable_length}</td>
			<td><button type="button" onclick={delete_cable} class="focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900">{"Delete Cable"}</button></td>
		</tr>
	}
}