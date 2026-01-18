use crate::data_builder::transform::ir::*;

#[derive(Debug)]
pub struct FlatField {
    pub name: String,
    pub bit_offset: usize,
    pub bit_size: usize,
    pub condition: Option<IRCondition>,
}

pub fn extract_fields(layout: &IRLayout) -> Vec<FlatField> {
    let mut fields = Vec::new();
    walk(layout, 0, None, &mut fields);
    fields
}

fn walk(
    layout: &IRLayout,
    base_offset: usize,
    condition: Option<IRCondition>,
    out: &mut Vec<FlatField>,
) {
    match layout {
        IRLayout::Primitive { bits } => {
            out.push(FlatField {
                name: "value".into(),
                bit_offset: base_offset,
                bit_size: *bits,
                condition,
            });
        }

        IRLayout::Sequence { elements } => {
            let mut offset = base_offset;
            for el in elements {
                let size = estimate_size(&el.layout);
                walk(&el.layout, offset, condition.clone(), out);
                offset += size;
            }
        }

        IRLayout::Optional { condition: cond, node } => {
            walk(&node.layout, base_offset, Some(cond.clone()), out);
        }

        IRLayout::Repetition { .. } => {
            // ASTERIX: repetition NON genera API diretta
            // va trattata separatamente
        }
    }
}

fn estimate_size(layout: &IRLayout) -> usize {
    match layout {
        IRLayout::Primitive { bits } => *bits,
        IRLayout::Sequence { elements } => {
            elements.iter().map(|e| estimate_size(&e.layout)).sum()
        }
        IRLayout::Optional { node, .. } => estimate_size(&node.layout),
        IRLayout::Repetition { .. } => 0,
    }
}
