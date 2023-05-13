use super::super::{prompt, Status};
use anyhow::Result;
use faf::Repo;
use log::{error, info};
use std::path::Path;
use uuid::Uuid;

fn prompt_for_meta_id(repo: &Repo) -> Result<Option<Uuid>> {
    let manifests = repo.list_manifests()?;
    let manifest_count = manifests.len();

    if manifest_count == 0 {
        error!("No metadirectories exist yet!");
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

    let line = if manifest_count > 1 {
        prompt(&format!("Enter 1-{} or Q to quit", manifest_count))?
    } else {
        prompt("Enter 1 or Q to quit")?
    };
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
            None => return Ok(Status::Failure),
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
