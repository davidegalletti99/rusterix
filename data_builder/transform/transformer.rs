use crate::data_builder::parse::xml_model::*;
use crate::data_builder::transform::ir::*;

/// Entry point: XML model → IR
/// Una singola categoria per IR
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
        node: IrNode {
            name: item
                .name
                .unwrap_or_else(|| format!("Item{:03}", item.id)),
            layout: to_ir_sequence(item.body),
        },
    }
}

//
// ─────────────────────────────
// Sequence (Scelta B)
// ─────────────────────────────
//

fn to_ir_sequence(seq: Sequence) -> IRLayout {
    // fspec è strutturale: NON genera IR node
    // viene ignorato qui (serve solo a livello semantico / condizioni)
    let _fspec = seq.fspec;

    IRLayout::Sequence {
        elements: seq
            .elements
            .into_iter()
            .filter_map(to_ir_element)
            .collect(),
    }
}

//
// ─────────────────────────────
// Element → IR
// ─────────────────────────────
//

fn to_ir_element(el: Element) -> Option<IrNode> {
    match el {
        Element::Primitive(p) => Some(IrNode {
            name: p.name.unwrap_or_else(|| "value".into()),
            layout: IRLayout::Primitive { bits: p.bits },
        }),

        Element::Sequence(s) => Some(IrNode {
            name: "sequence".into(),
            layout: to_ir_sequence(s),
        }),

        Element::Optional(o) => {
            let condition = parse_condition(&o.condition);
            let node = to_ir_element(*o.element)?;

            Some(IrNode {
                name: node.name.clone(),
                layout: IRLayout::Optional {
                    condition,
                    node: Box::new(node),
                },
            })
        }

        Element::Repeat(r) => {
            let counter = parse_counter(&r.counter);
            let node = to_ir_element(*r.element)?;

            Some(IrNode {
                name: node.name.clone(),
                layout: IRLayout::Repetition {
                    counter,
                    node: Box::new(node),
                },
            })
        }

        Element::Text(_) => None, // ignora whitespace / testo
    }
}

//
// ─────────────────────────────
// Condition / Counter parsing
// ─────────────────────────────
//

fn parse_condition(cond: &str) -> IRCondition {
    // formato richiesto:
    // fspec:BYTE.BIT   es: fspec:0.3
    if let Some(rest) = cond.strip_prefix("fspec:") {
        let mut parts = rest.split('.');

        let byte = parts
            .next()
            .and_then(|v| v.parse::<usize>().ok())
            .expect("Invalid fspec byte index");

        let bit = parts
            .next()
            .and_then(|v| v.parse::<u8>().ok())
            .expect("Invalid fspec bit index");

        IRCondition::BitSet { byte, bit }
    } else {
        panic!("Unsupported condition format: {}", cond);
    }
}

fn parse_counter(counter: &str) -> IRCounter {
    // formati supportati:
    // - fixed:N
    // - field:BITS

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
