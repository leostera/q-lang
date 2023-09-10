pub trait Diagnostic: Send + Sync + std::fmt::Debug {
}


#[derive(Debug)]
pub struct Diagnostics {
    pub errors: Vec<Box<dyn Diagnostic>>,
    pub warnings: Vec<Box<dyn Diagnostic>>
}
