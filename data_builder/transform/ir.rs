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
    pub node: IrNode,
}

/// A generic IR node.
#[derive(Debug)]
pub struct IrNode {
    pub name: String,
    pub layout: IRLayout,
}

/// Structural layout description.
#[derive(Debug)]
pub enum IRLayout {
    /// A primitive binary field.
    Primitive {
        bits: usize,
    },

    /// A sequence of nodes laid out in order.
    Sequence {
        elements: Vec<IrNode>,
    },

    /// An optional node guarded by a condition.
    Optional {
        condition: IRCondition,
        node: Box<IrNode>,
    },

    /// A repeated node.
    Repetition {
        counter: IRCounter,
        node: Box<IrNode>,
    },
}

/// Presence condition for optional nodes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRCondition {
    /// A specific bit must be set.
    BitSet {
        byte: usize,
        bit: u8,
    },
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