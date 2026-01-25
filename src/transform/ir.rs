/// Intermediate Representation (IR).
#[derive(Debug)]
pub struct IR {
    pub category: IRCategory,
}

/// A category (e.g. CAT048).
#[derive(Debug)]
pub struct IRCategory {
    pub id: u8,
    pub items: Vec<IRItem>,
}

/// A data item inside a category.
#[derive(Debug)]
pub struct IRItem {
    pub id: u8,
    pub frn: u8,
    pub node: IRNode,
}

/// A generic IR node.
#[derive(Debug)]
pub struct IRNode {
    pub bytes: usize,
    pub layout: IRLayout,
}

/// Structural layout description.
#[derive(Debug)]
pub enum IRLayout {
    Fixed {
        elements: Vec<IRNode>,
    },

    Explicit {
        elements: Vec<IRNode>,
    },

    Extended {
        elements: Vec<IRNode>,
    },
    Repetition {
        counter: IRCounter,
        elements: Vec<IRNode>,
    },
    Compound {
        layout: Box<IRNode>
    },
}
pub enum IRElement {
    Field{
        bits: usize,
        name: String,
    },
    Enum(IREnum),
}


/// Repetition counter description.
#[derive(Debug)]
pub enum IRCounter {
    /// Fixed number of repetitions.
    Fixed(usize),

    /// Number of repetitions read from the stream.
    FromField {
        bits: usize,
    },
}