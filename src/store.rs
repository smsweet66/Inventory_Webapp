use yewdux::prelude::*;
use reqwest::Client;

use crate::models::{cable_type_model::*, cable_model::*};

#[derive(Default, Clone, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct State {
    pub loading: bool,
    pub client: Client,
    pub cable_types: Vec<CableType>,
    pub cables: Vec<Cable>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cable_types == other.cable_types && self.cables == other.cables
    }
}

pub fn set_page_loading(loading: bool, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.loading = loading;
    });
}

pub fn set_cable_types(cable_types: Vec<CableType>, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cable_types = cable_types;
    });
}

pub fn add_cable_type(cable_type: CableType, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cable_types.push(cable_type);
    });
}

pub fn update_cable_type(cable_type: CableType, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cable_types.iter().position(|x| x.id == cable_type.id).unwrap();
        state.cable_types[index] = cable_type;
    });
}

pub fn delete_cable_type(cable_type_id: i32, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cable_types.iter().position(|x| x.id == cable_type_id).unwrap();
        state.cable_types.remove(index);
    });
}

pub fn set_cables(cables: Vec<Cable>, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cables = cables;
    });
}

pub fn add_cable(cable: Cable, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cables.push(cable);
    });
}

pub fn update_cable(cable: Cable, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cables.iter().position(|x| x.id == cable.id).unwrap();
        state.cables[index] = cable;
    });
}

pub fn delete_cable(cable_id: i32, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cables.iter().position(|x| x.id == cable_id).unwrap();
        state.cables.remove(index);
    });
}