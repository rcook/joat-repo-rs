use crate::manifest::ManifestEx;
use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use log::error;
use std::collections::HashMap;

use std::io::{stdin, stdout, Write};
use uuid::Uuid;

#[derive(Debug)]
struct ManifestStatus {
    manifest: ManifestEx,
    is_used: bool,
}

pub fn do_clean(repo: &Repo) -> Result<Status> {
    let mut manifest_status_map = repo
        .list_manifests()?
        .into_iter()
        .map(|m| {
            (
                m.manifest.meta_id,
                ManifestStatus {
                    manifest: m,
                    is_used: false,
                },
            )
        })
        .collect::<HashMap<Uuid, _>>();

    for link in repo.list_links()? {
        match manifest_status_map.get_mut(&link.link.meta_id) {
            Some(m) => m.is_used = true,
            None => todo!(),
        }
    }

    let mut delete_count = 0;
    for m in manifest_status_map.values() {
        if !m.is_used {
            if delete_count == 0 {
                println!("The following metadirectories are unreferenced:")
            }
            delete_count += 1;
            println!("({}) {:#?}", delete_count, m.manifest);
        }
    }

    if delete_count == 0 {
        println!("Nothing to clean");
        return Ok(Status::Success);
    }

    print!("Type DELETE to delete them: ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let line = line.trim().to_lowercase();
    if line != "delete" {
        error!("Aborting clean-up");
        return Ok(Status::Failure);
    }

    /*
    for m in manifest_status_map.values() {
        if !m.is_used {
            remove_dir_all(&m.manifest.data_dir)?;
        }
    }
    */

    Ok(Status::Success)
}
