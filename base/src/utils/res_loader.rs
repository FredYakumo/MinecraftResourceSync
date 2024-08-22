use crate::models::error::Result;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_sha1_list_recursive(
    base_path: &str,
    recursive_sub_path: &str,
) -> Result<Vec<(String, String)>> {
    let base_path = Path::new(base_path);
    let mut ret_list: Vec<(String, String)> = Vec::new();
    let entries = fs::read_dir(base_path.join(recursive_sub_path))?;
    for entry in entries {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            // recurse into subdirs if it's a directory
            ret_list.append(&mut get_sha1_list_recursive(
                base_path.join(recursive_sub_path).to_str().unwrap(),
                entry.file_name().to_str().unwrap(),
            )?);
        }
        if entry.metadata()?.is_dir() { // recurse into subdirs if it's a directory
            ret_list.append(&mut get_sha1_list_recursive(base_path.to_str().unwrap(),
             entry.file_name().to_str().unwrap())?);
            continue;
        }
        let mut file = File::open(base_path.join(recursive_sub_path).join(entry.file_name()))?;
        let mut sha1 = Sha1::new();

        let mut buffer = [0; 1024];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            sha1.input(&buffer[..bytes_read]);
        }
        ret_list.push((
            String::from(
                Path::new(recursive_sub_path)
                    .join(entry.file_name())
                    .to_str()
                    .unwrap(),
            ),
            sha1.result_str(),
        )); // add the SHA1 hash to the list
    }

    Ok(ret_list)
}
