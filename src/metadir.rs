use crate::link::LinkEx;
use crate::manifest::ManifestEx;

#[derive(Debug)]
pub struct Metadir {
    pub manifest: ManifestEx,
    pub link: LinkEx,
}
