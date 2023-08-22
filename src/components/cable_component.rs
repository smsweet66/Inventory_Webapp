use yew::prelude::*;
use yewdux::prelude::*;
use crate::models::cable_model::Cable;
use crate::store::State;

#[derive(PartialEq, Properties)]
pub struct CableProps {
	pub data: Cable,
}

#[function_component]
pub fn CableComponent(props: &CableProps) -> Html {
	let (store, dispatch) = use_store::<State>();
	let cable_types = store.cable_types.clone();
	let cable_type_a = cable_types.iter().find(|cable_type| cable_type.id == props.data.end_a).unwrap();
	let cable_type_b = cable_types.iter().find(|cable_type| cable_type.id == props.data.end_b).unwrap();

	html! {
		<table>
			<tr>
				<td width="20%">{&cable_type_a.name}</td>
				<td width="20%">{&cable_type_a.cable_gender}</td>
				<td width="20%">{&cable_type_b.name}</td>
				<td width="20%">{&cable_type_b.cable_gender}</td>
				<td width="20%">{&props.data.cable_length}</td>
			</tr>
		</table>
	}
}

#[derive(PartialEq, Properties)]
pub struct CableListProps {
	pub data: Vec<Cable>,
}

#[function_component]
pub fn CableListComponent(list_props: &CableListProps) -> Html {
	html! { 
		<div>
			{for list_props.data.iter().map(|cable| html! {<CableComponent data={cable.clone()}/>})}
		</div>
	}
}