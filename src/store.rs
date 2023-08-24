use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

use crate::models::{cable_type_model::*, cable_model::*};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, Clone, Store, Serialize, Deserialize, Debug, PartialEq)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub loading: bool,
    pub alert_input: AlertInput,
    pub cable_types: Vec<CableType>,
    pub cables: Vec<Cable>,
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    });
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            show_alert: true,
            alert_message: message,
        };
    });
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    });
}

pub fn set_cable_types(cable_types: Vec<CableType>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.cable_types = cable_types;
    });
}

pub fn add_cable_type(cable_type: CableType, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.cable_types.push(cable_type);
    });
}

pub fn update_cable_type(cable_type: CableType, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        let index = store.cable_types.iter().position(|x| x.id == cable_type.id).unwrap();
        store.cable_types[index] = cable_type;
    });
}

pub fn delete_cable_type(cable_type_id: i32, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        let index = store.cable_types.iter().position(|x| x.id == cable_type_id).unwrap();
        store.cable_types.remove(index);

        store.cables.retain(|x| x.end_a != cable_type_id && x.end_b != cable_type_id);
    });
}

pub fn set_cables(cables: Vec<Cable>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.cables = cables;
    });
}

pub fn add_cable(cable: Cable, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.cables.push(cable);
    });
}

pub fn update_cable(cable: Cable, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        let index = store.cables.iter().position(|x| x.id == cable.id).unwrap();
        store.cables[index] = cable;
    });
}

pub fn delete_cable(cable_id: i32, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        let index = store.cables.iter().position(|x| x.id == cable_id).unwrap();
        store.cables.remove(index);
    });
}