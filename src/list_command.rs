use crate::repo::Repo;
use crate::status::Status;
use anyhow::Result;
use colored::Colorize;

pub fn do_list(repo: &Repo) -> Result<Status> {
    let mut manifests = repo.list_manifests()?;
    manifests.sort_unstable_by_key(|m| m.manifest.meta_id);
    let manifests = manifests;
    if !manifests.is_empty() {
        println!("{}", "Metadirectories".green());
        for m in manifests {
            println!(
                "  {} ({})",
                m.manifest.meta_id.as_simple().to_string().yellow(),
                m.data_dir.display().to_string().blue()
            );
        }
    }

    let mut links = repo.list_links()?;
    links.sort_unstable_by_key(|l| l.link.project_dir.clone());
    let links = links;
    if !links.is_empty() {
        println!("{}", "Links".green());
        for l in links {
            println!(
                "  {} ({})\n    -> {} ({})",
                l.link.link_id.as_str().yellow(),
                l.link_path.display().to_string().blue(),
                l.link.meta_id.as_simple().to_string().yellow(),
                l.link.project_dir.display().to_string().bright_magenta(),
            );
        }
    }

    Ok(Status::Success)
}
