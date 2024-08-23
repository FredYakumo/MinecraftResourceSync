use std::collections::HashMap;

use serde::Deserialize;

use super::ModClass;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ClientFetchModListRequest {
    pub mod_class_map: Vec<ModClass>
}