use yew::prelude::*;
use crate::models::cable_type_model::CableType;

#[derive(PartialEq, Properties)]
pub struct CableTypeProps {
	pub data: CableType,
}

#[function_component]
pub fn CableTypeComponent(props: &CableTypeProps) -> Html {
	html! {
		<table>
			<tr>
				<td width="33.3%">{&props.data.name}</td>
				<td width="33.3%">{&props.data.cable_gender}</td>
				<td width="33.3%"><img src="data:image/png;base64,{&props.data.image}"/></td>
			</tr> 
		</table>
	}
}

#[derive(PartialEq, Properties)]
pub struct CableTypeListProps {
	pub data: Vec<CableType>,
}

#[function_component]
pub fn CableTypeListComponent(list_props: &CableTypeListProps) -> Html {
	html! { 
		<div>
			{for list_props.data.iter().map(|cable_type| html! {<CableTypeComponent data={cable_type.clone()}/>})}
		</div>
	}
}