#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Data,
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSource {
    User,
    System,
    Imported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportClass {
    Exportable,
    LocalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectOwner {
    User,
    System,
}
