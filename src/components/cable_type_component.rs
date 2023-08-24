use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::models::cable_type_model::CableType;
use crate::store::{self, Store};
use crate::server_requests::cable_type_requests;

#[derive(PartialEq, Properties)]
pub struct CableTypeProps {
	pub data: CableType,
	pub on_click: Callback<MouseEvent>,
}

/// displays the name of the cable, its gender, and an image of the cable in a table row
/// the image is stored as a u8 array in the database, so it must be converted to a string
/// also displays a trash can icon that deletes the cable from the database when clicked
#[function_component]
pub fn CableTypeComponent(props: &CableTypeProps) -> Html {
	let (_, dispatch) = use_store::<Store>();
	
	// if the trash can icon is clicked, delete the cable from the database
	let delete_cable = {
		let cloned_dispatch = dispatch.clone();
		let cable_type_id = props.data.id;
		
		Callback::from(move |event: MouseEvent| {
			event.prevent_default();
			event.stop_propagation();
			let dispatch = cloned_dispatch.clone();

			spawn_local(async move {
				let res = cable_type_requests::delete_cable_type(cable_type_id).await;
				match res {
					Ok(_) => {
						store::set_show_alert("Cable type deleted successfully".to_string(), dispatch.clone());
						store::delete_cable_type(cable_type_id, dispatch);
					},
					Err(e) => {
						store::set_show_alert(e.to_string(), dispatch);
					}
				}
			})
		})
	};

	html! {
		<tr id={format!("cable_type_{}", props.data.id)} onclick={&props.on_click}>
			<td id={format!("cable_name_{}", props.data.id)}>{&props.data.name}</td>
			<td id={format!("cable_gender_{}", props.data.id)}>{&props.data.cable_gender}</td>
			<td id={format!("cable_image_{}", props.data.id)}><img id={format!("inner_image_{}", props.data.id)} src={format!("data:image/png;base64,{}", String::from_utf8_lossy(&props.data.image))}/></td>
			<td><button type="button" onclick={delete_cable} class="focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900">{"Delete Cable Type"}</button></td>
		</tr>
	}
}