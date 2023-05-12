use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use log::{error, info};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use uuid::Uuid;

fn prompt_for_meta_id(repo: &Repo) -> Result<Option<Uuid>> {
    let manifests = repo.list_manifests()?;
    let manifest_count = manifests.len();

    if manifest_count == 0 {
        return Ok(None);
    }

    for (idx, manifest) in manifests.iter().enumerate() {
        println!(
            "({}): {}: {:#?}",
            idx + 1,
            manifest.manifest.meta_id.as_simple(),
            manifest
        )
    }

    if manifest_count > 1 {
        print!("Enter 1 to {} or Q to quit: ", manifest_count);
    } else {
        print!("Enter 1 or Q to quit: ");
    }
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let line = line.trim().to_lowercase();
    if line == "q" {
        return Ok(None);
    }

    let index = match line.parse::<usize>() {
        Ok(value) => value,
        _ => return Ok(None),
    };

    if index < 1 || index > manifest_count {
        return Ok(None);
    }

    Ok(Some(manifests[index - 1].manifest.meta_id))
}

pub fn do_link(repo: &Repo, meta_id: &Option<Uuid>, project_dir: &Path) -> Result<Status> {
    if let Some(link) = repo.read_link(project_dir)? {
        error!(
            "Link {} already exists for directory {}",
            link.link.link_id.as_str(),
            project_dir.display()
        );
        return Ok(Status::Failure);
    }

    let meta_id = match *meta_id {
        Some(value) => value,
        None => match prompt_for_meta_id(repo)? {
            Some(value) => value,
            None => {
                error!(
                    "Could not create link for directory {}",
                    project_dir.display()
                );
                return Ok(Status::Failure);
            }
        },
    };

    Ok(match repo.link_metadir(&meta_id, project_dir)? {
        Some(metadir) => {
            info!("{:#?}", metadir);
            Status::Success
        }
        None => {
            error!(
                "Could not create link for directory {}",
                project_dir.display()
            );
            Status::Failure
        }
    })
}