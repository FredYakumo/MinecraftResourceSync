use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use maplit::{hashmap, hashset};
use serde::Deserialize;

use crate::util::res_manager::{ManagedMod, ModClass};

fn get_ip_info_from_request(_req: &HttpRequest) -> String {
    let addr = _req.peer_addr().expect("unknown addr");
    return addr.ip().to_string();
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ClassNameModHashList {
    pub className: String,
    pub modSha1FileNameMap: HashMap<String, String>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ClassNameModAndResidualHashList {
    pub className: String,
    pub modSha1List: Vec<String>,
    pub residualModNameSha1Map: HashMap<String, String>,
}

#[post("/fetchMissingModList")]
pub async fn fetch_missiong_mod_list(
    managed_mod: Data<Arc<ManagedMod>>,
    req: &HttpRequest,
    req_class_name_mod_list: Data<Arc<Vec<ClassNameModHashList>>>,
) -> impl Responder {
    let req_mod_class_sha1_map = construct_mod_class_sha1_file_map(&req_class_name_mod_list);
    let mut ret = hashmap! {};
    for (class_name, sha1_file_map) in req_mod_class_sha1_map.iter() {
        if let Some(mod_class) = managed_mod.class_name_map.get(class_name) {
            ret.insert(class_name)
        } else {
            ret.insert(
                class_name,
                ClassNameModAndResidualHashList {
                    className: class_name.clone(),
                    modSha1List: vec![],
                    residualModNameSha1Map: sha1_file_map
                        .iter()
                        .map(|(sha1, file)| (file.clone(), sha1.clone()))
                        .collect(),
                },
            );
        }
    }
    HttpResponse::Ok()
}

fn construct_mod_class_sha1_file_map(
    class_name_mod_hash_list: &Vec<ClassNameModHashList>,
) -> HashMap<String, HashMap<String, String>> {
    let mut ret = hashmap! {};
    for class in class_name_mod_hash_list {
        let sha1_map = ret.entry(class.className.clone()).or_insert(hashmap! {});
        for (sha1, file_name) in &class.modSha1FileNameMap {
            sha1_map.insert(sha1.clone(), file_name.clone());
        }
    }
    ret
}
