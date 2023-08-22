use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::models::cable_type_model::CableType;
use crate::store::{self, Store};
use crate::server_requests::cable_type_requests;

#[derive(PartialEq, Properties)]
pub struct CableTypeProps {
	pub data: CableType,
}

/// displays the name of the cable, its gender, and an image of the cable in a table row
/// the image is stored as a u8 array in the database, so it must be converted to a string
/// also displays a trash can icon that deletes the cable from the database when clicked
#[function_component]
pub fn CableTypeComponent(props: &CableTypeProps) -> Html {
	let (store, dispatch) = use_store::<Store>();
	
	// if the trash can icon is clicked, delete the cable from the database
	let delete_cable = {
		let cloned_dispatch = dispatch.clone();
		let cloned_client = store.as_ref().client.clone();
		let cable_type_id = props.data.id;
		
		Callback::from(move |_: MouseEvent| {
			let dispatch = cloned_dispatch.clone();
			let client = cloned_client.clone();

			spawn_local(async move {
				let res = cable_type_requests::delete_cable_type(&client, cable_type_id).await;
				match res {
					Ok(_) => {
						store::set_show_alert("Cable type deleted successfully".to_string(), dispatch.clone());
						store::delete_cable_type(cable_type_id, dispatch);
					},
					Err(_) => {
						store::set_show_alert("Error deleting cable type".to_string(), dispatch);
					}
				}
			})
		})
	};

	html! {
		<div>
			<table>
				<tr>
					<td width="20%">{&props.data.name}</td>
					<td width="20%">{&props.data.cable_gender}</td>
					<td width="40%"><img src={format!("data:image/png;base64, {}", String::from_utf8_lossy(&props.data.image))} /></td>
					<td width="20%"><i class="fas fa-trash" onclick={delete_cable}></i></td>
				</tr>
			</table>
		</div>
	}
}