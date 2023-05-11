use crate::link::LinkEx;
use crate::manifest::ManifestEx;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Metadirectory {
    pub manifest: ManifestEx,
    pub link: LinkEx,
}
