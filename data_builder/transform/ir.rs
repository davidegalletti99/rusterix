// ir.rs
#[derive(Debug)]
pub struct IR {
    pub categories: Vec<IRCategory>,
}

#[derive(Debug)]
pub struct IRCategory {
    pub id: u8,
    pub items: Vec<IRItem>,
}

#[derive(Debug)]
pub struct IRItem {
    pub id: u8,
    pub layout: IRLayout,
}

#[derive(Debug)]
pub enum IRLayout {
    Fixed(IRFixed),
    Extended(IRExtended),
    Repetitive(IRRepetitive),
    Compound(IRCompound),
}

#[derive(Debug)]
pub struct IRFixed {
    pub length: usize,
    pub fields: Vec<IRField>,
}

#[derive(Debug)]
pub struct IRRepetitive {
    pub length: usize,
    pub fields: Vec<IRField>,
}

#[derive(Debug)]
pub struct IRExtended {
    pub primary: IRFixed,
    pub secondary: IRFixed,
}

#[derive(Debug)]
pub struct IRCompound {
    pub parts: Vec<IRFixed>,
}

#[derive(Debug)]
pub struct IRField {
    pub name: String,
    pub size: usize,
    pub offset: Option<usize>,
}
