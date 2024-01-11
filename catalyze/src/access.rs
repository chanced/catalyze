
pub trait Package {
    fn package(&self) -> Option<crate::package::Package>;
}

pub trait File {
    fn file(&self) -> Option<crate::file::File>;
}
