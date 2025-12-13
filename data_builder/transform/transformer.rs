use crate::data_builder::parse::xml_model::*;
use super::ir::*;

pub fn to_ir(categories: Vec<Category>) -> IR {
    IR {
        categories: categories
            .into_iter()
            .map(to_ir_category)
            .collect(),
    }
}

fn to_ir_category(cat: Category) -> IRCategory {
    IRCategory {
        id: cat.id,
        items: cat.items.into_iter().map(to_ir_item).collect(),
    }
}

fn to_ir_item(item: Item) -> IRItem {
    let layout = match_item_layout(item.layout);

    IRItem {
        id: item.id,
        layout,
    }
}

fn match_item_layout(layout: ItemLayout) -> IRLayout {
    match layout {
        ItemLayout::Fixed(f) => IRLayout::Fixed(IRFixed {
            length: f.length,
            fields: f.fields.into_iter().map(to_ir_field).collect(),
        }),

        ItemLayout::Extended(e) => IRLayout::Extended(IRExtended {
            primary: IRFixed {
                length: e.primary_part.length,
                fields: e
                    .primary_part
                    .fields
                    .into_iter()
                    .map(to_ir_field)
                    .collect(),
            },
            secondary: IRFixed {
                length: e.secondary_parts.length,
                fields: e
                    .secondary_parts
                    .fields
                    .into_iter()
                    .map(to_ir_field)
                    .collect(),
            },
        }),

        ItemLayout::Repetitive(r) => IRLayout::Repetitive(IRRepetitive {
            length: r.length,
            fields: r.fields.into_iter().map(to_ir_field).collect(),
        }),

        ItemLayout::Compound(c) => IRLayout::Compound(IRCompound {
            parts: c
                .parts
                .into_iter()
                .map(|p| IRFixed {
                    length: p.length,
                    fields: p.fields.into_iter().map(to_ir_field).collect(),
                })
                .collect(),
        }),
    }
}

fn to_ir_field(f: Field) -> IRField {
    IRField {
        name: f.name,
        size: f.size,
        offset: f.offset,
    }
}
