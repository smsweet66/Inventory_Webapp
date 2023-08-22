use yew::prelude::*;
use gloo::utils::document;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        html!{ {for props.children.iter()} },
        modal_host.into(),
    )
}