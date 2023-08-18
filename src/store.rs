use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use reqwest::Client;

use crate::models::{cable_type_model::*, cable_model::*};
use crate::server_requests::{cable_type_requests::*, cable_requests::*};

#[derive(Default, Clone, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct State {
    client: Client,
    pub cable_types: Vec<CableType>,
    pub cables: Vec<Cable>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cable_types == other.cable_types && self.cables == other.cables
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cable_types: Vec::new(),
            cables: Vec::new(),
        }
    }

    async fn fetch_cable_types(&mut self) {
        let cable_types = get_cable_types(&self.client).await.unwrap();
        self.cable_types = cable_types;
    }

    async fn fetch_cables(&mut self) {
        let cables = get_cables(&self.client).await.unwrap();
        self.cables = cables;
    }
}

pub fn init(dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.fetch_cable_types();
        state.fetch_cables();
    });
}

pub fn add_cable_type(cable_type: CableType, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cable_types.push(cable_type);
    });
}

pub fn update_cable_type(cable_type: CableType, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cable_types.iter().position(|cable_type| cable_type.id == cable_type.id).unwrap();
        state.cable_types[index] = cable_type;
    });
}

pub fn delete_cable_type(id: i32, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cable_types.retain(|cable_type| cable_type.id != id);
    });
}

pub fn add_cable(cable: Cable, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cables.push(cable);
    });
}

pub fn update_cable(cable: Cable, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        let index = state.cables.iter().position(|cable| cable.id == cable.id).unwrap();
        state.cables[index] = cable;
    });
}

pub fn delete_cable(id: i32, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cables.retain(|cable| cable.id != id);
    });
}