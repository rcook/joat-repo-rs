use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use colored::Colorize;

pub fn do_list(repo: &Repo) -> Result<Status> {
    println!("{}", "Metadirectories".green());
    let mut manifests = repo.list_manifests()?;
    manifests.sort_unstable_by_key(|m| m.manifest.meta_id);
    let manifests = manifests;
    for m in manifests {
        println!(
            "  {} ({})",
            m.manifest.meta_id.as_simple().to_string().yellow(),
            m.data_dir.display().to_string().blue()
        );
    }

    println!("{}", "Links".green());
    let mut links = repo.list_links()?;
    links.sort_unstable_by_key(|l| l.link.project_dir.clone());
    let links = links;
    for l in links {
        println!(
            "  {} ({}) ({})",
            l.link.link_id.as_str().yellow(),
            l.link.project_dir.display().to_string().blue(),
            l.link.meta_id.as_simple().to_string().yellow()
        );
    }

    Ok(Status::Success)
}
