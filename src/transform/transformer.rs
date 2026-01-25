use crate::{
    parse::xml_model::*, 
    transform::ir::*
};

/// Entry point: XML model → IR
pub fn to_ir(cat: Category) -> IR {
    IR {
        category: to_ir_category(cat),
    }
}

fn to_ir_category(cat: Category) -> IRCategory {
    IRCategory {
        id: cat.id,
        items: cat.items.into_iter().map(to_ir_item).collect(),
    }
}

fn to_ir_item(item: Item) -> IRItem {
    IRItem {
        id: item.id,
        frn: item.frn,
        node: IrNode {
            name: format!("Item{:03}", item.id),
            layout: to_ir_sequence(item.body),
        },
    }
}

//
// Sequence
//

fn to_ir_sequence(seq: Sequence) -> IRLayout {

    IRLayout::Sequence {
        elements: seq.elements.into_iter().filter_map(to_ir_element).collect(),
    }
}
//
// Element → IR
//

fn to_ir_element(el: SubItemStructure) -> Option<IrNode> {
    match el {
        SubItemStructure::Primitive(p) => Some(IrNode {
            name: p.name.unwrap_or_else(|| "value".into()),
            layout: IRLayout::Primitive { bits: p.bits },
        }),

        SubItemStructure::Sequence(s) => Some(IrNode {
            name: "sequence".into(),
            layout: to_ir_sequence(s),
        }),

        SubItemStructure::Optional(o) => {
            let condition = IRCondition::BitSet {
                byte: (o.frn / 8) as usize,
                bit: 7 - (o.frn % 8),
            };

            let node = to_ir_element(*o.element)?;

            Some(IrNode {
                name: node.name.clone(),
                layout: IRLayout::Optional {
                    condition,
                    node: Box::new(node),
                },
            })
        }

        SubItemStructure::Repeat(r) => {
            let counter = parse_counter(&r.counter);
            let node = to_ir_element(*r.element)?;

            Some(IrNode {
                name: node.name.clone(),
                layout: IRLayout::Repetition {
                    counter,
                    node: Box::new(node),
                },
            })
        },
        SubItemStructure::Spare(s) => {
            let bits = s.bits;
            Some(IrNode { 
                name: "spare".into(), 
                layout: IRLayout::Spare {
                    bits: bits
                },
            })
        }
    }
}

//
// Counter parsing
//
fn parse_counter(counter: &str) -> IRCounter {
    if let Some(rest) = counter.strip_prefix("fixed:") {
        IRCounter::Fixed(
            rest.parse::<usize>()
                .expect("Invalid fixed repetition count"),
        )
    } else if let Some(rest) = counter.strip_prefix("field:") {
        IRCounter::FromField {
            bits: rest
                .parse::<usize>()
                .expect("Invalid repetition field size"),
        }
    } else {
        panic!("Unsupported repetition counter format: {}", counter);
    }
}
