use std::collections::HashMap;

use crate::{
    builtins::{__lat_dbg, __lat_mapset, __lat_newmap},
    l_fn, l_str,
    types::Value,
};

#[derive(Debug)]
pub(crate) struct LatticeState {
    pub constants: Vec<Value>,
    pub globals: HashMap<String, Value>,
}

impl LatticeState {
    pub fn new() -> Self {
        let mut state = Self {
            constants: Vec::new(),
            globals: HashMap::new(),
        };

        state.populate_global_values();
        state.populate_global_functions();

        state
    }

    fn populate_global_values(&mut self) {
        self.globals.insert("__VERSION".into(), l_str!("0.1.0"));
    }

    fn populate_global_functions(&mut self) {
        self.globals.insert("__lat_dbg".into(), l_fn!(__lat_dbg));
        self.globals
            .insert("__lat_newmap".into(), l_fn!(__lat_newmap));
        self.globals.insert("__lat_mapset".into(), l_fn!(__lat_mapset));
    }
}
