use yew::prelude::*;
use gloo::utils::document;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub host_id: String,
}

#[function_component]
pub fn ModalComponent(props: &ModalProps) -> Html {
    let modal_host = document()
        .get_element_by_id(&props.host_id)
        .expect(format!("Could not find element with id {}", &props.host_id).as_str());

    let on_click = {
        let cloned_modal_host = modal_host.clone();
        Callback::from(move |_| {
            cloned_modal_host.set_attribute("style", "display: none;").unwrap();
        })
    };

    create_portal(
        html!{
            <>
                {for props.children.iter()}
                <span onclick={on_click} class="w3-button w3-display-topright" title="Close Modal">{"&times;"}</span>
            </>
        },
        modal_host.into(),
    )
}