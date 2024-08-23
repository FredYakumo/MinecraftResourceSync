use std::collections::{HashMap, HashSet};

use base::{models::{Config, ModClass}, utils::res_loader::get_sha1_list_recursive};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use base::models::error::Result;
