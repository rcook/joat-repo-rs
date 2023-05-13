use crate::link::LinkEx;
use crate::manifest::ManifestEx;

#[derive(Debug)]
pub struct DirInfo {
    pub manifest: ManifestEx,
    pub link: LinkEx,
}
