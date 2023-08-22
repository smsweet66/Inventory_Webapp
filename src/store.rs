use yewdux::prelude::*;
use reqwest::Client;

use crate::models::{cable_type_model::*, cable_model::*};

#[derive(Default, Clone, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct State {
    pub loading: bool,
    client: Client,
    pub cable_types: Vec<CableType>,
    pub cables: Vec<Cable>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cable_types == other.cable_types && self.cables == other.cables
    }
}

pub fn set_loading(loading: bool, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.loading = loading;
    });
}

pub fn set_cable_types(cable_types: Vec<CableType>, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cable_types = cable_types;
    });
}

pub fn set_cables(cables: Vec<Cable>, dispatch: Dispatch<State>) {
    dispatch.reduce_mut(move |state| {
        state.cables = cables;
    });
}