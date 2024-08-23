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

#[post("/fetchMissingModList")]
pub async fn fetch_missiong_mod_list(
    managed_mod: Data<Arc<ManagedMod>>,
    req: Data<Arc<HttpRequest>>,
    req_class_name_mod_list: Data<Arc<Vec<ClassNameModHashList>>>,
) -> impl Responder {
    let req_mod_class_sha1_map = construct_mod_class_sha1_file_map(&req_class_name_mod_list);
    let mut ret = hashmap! {};
    for (class_name, sha1_file_map) in req_mod_class_sha1_map.iter() {
        if let Some(mod_class) = managed_mod.class_name_map.get(class_name) {
            let mut mod_sha1_map = hashmap! {};
            ret.insert(class_name, ClassNameModAndResidualHashList {
                className: class_name.clone(),
                modNameSha1Map: mod_class.get_missing_sha1_file_name_map(sha1_file_map.keymap(collect()),
                residualModNameSha1Map: hashmap! {}
            });
        } else {
            ret.insert(
                class_name,
                ClassNameModAndResidualHashList {
                    className: class_name.clone(),
                    modNameSha1Map: hashmap! {},
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
