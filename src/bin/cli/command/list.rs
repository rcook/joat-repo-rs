use super::super::Status;
use anyhow::Result;
use colored::Colorize;
use joat_repo::Repo;

pub fn do_list(repo: &Repo) -> Result<Status> {
    let mut manifests = repo.list_manifests()?;
    manifests.sort_by_cached_key(|m| m.manifest.meta_id.clone());
    let manifests = manifests;
    if !manifests.is_empty() {
        println!(
            "{}",
            format!("Metadirectories ({})", manifests.len()).green()
        );
        for m in manifests {
            println!(
                "  {} ({})",
                m.manifest.meta_id.to_string().yellow(),
                m.data_dir.display().to_string().blue()
            );
        }
    }

    let mut links = repo.list_links()?;
    links.sort_by_cached_key(|l| l.link.project_dir.clone());
    let links = links;
    if !links.is_empty() {
        println!("{}", format!("Links ({})", links.len()).green());
        for l in links {
            println!(
                "  {} ({})\n    -> {} ({})",
                l.link.link_id.to_string().yellow(),
                l.link_path.display().to_string().blue(),
                l.link.meta_id.to_string().yellow(),
                l.link.project_dir.display().to_string().bright_magenta(),
            );
        }
    }

    Ok(Status::Success)
}
