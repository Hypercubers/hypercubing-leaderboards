use itertools::Itertools;

use super::{Category, CategoryQuery, ProgramQuery, Puzzle, VariantQuery};
use crate::traits::Linkable;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Event {
    pub puzzle: Puzzle,
    pub category: Category,
}
impl Linkable for Event {
    fn relative_url(&self) -> String {
        format!("/puzzle?id={}{}", self.puzzle.id.0, self.url_query_params())
    }

    fn md_text(&self) -> String {
        self.name()
    }
}
impl Event {
    fn category_query(&self) -> CategoryQuery {
        match &self.category {
            Category::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                material,
            } => {
                let default_filters = match variant {
                    Some(v) => v.primary_filters,
                    None => self.puzzle.primary_filters,
                };
                let default_macros = match variant {
                    Some(v) => v.primary_macros,
                    None => self.puzzle.primary_macros,
                };
                let default_material = match variant {
                    Some(v) => v.material_by_default,
                    None => false, // virtual by default for all puzzles
                };
                CategoryQuery::Speed {
                    average: *average,
                    blind: *blind,
                    filters: (*filters != default_filters).then_some(*filters),
                    macros: (*macros != default_macros).then_some(*macros),
                    one_handed: *one_handed,
                    variant: match variant {
                        Some(v) => VariantQuery::Named(v.abbr.clone()),
                        None => VariantQuery::Default,
                    },
                    program: if *material == default_material {
                        ProgramQuery::Default
                    } else {
                        match material {
                            true => ProgramQuery::Material,
                            false => ProgramQuery::Virtual,
                        }
                    },
                }
            }

            Category::Fmc { computer_assisted } => CategoryQuery::Fmc {
                computer_assisted: *computer_assisted,
            },
        }
    }

    fn url_query_params(&self) -> String {
        self.category_query().url_query_params()
    }

    pub fn name(&self) -> String {
        let mut s = String::new();

        let mut paren_modifiers = Vec::with_capacity(3);

        match &self.category {
            Category::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                material,
            } => {
                if *blind {
                    s += "Blind ";
                }
                if *one_handed {
                    s += "One-Handed ";
                }

                let material_by_default = match variant {
                    Some(variant) => variant.material_by_default,
                    None => false,
                };
                if material_by_default != *material {
                    match material {
                        true => s += "Material ",
                        false => s += "Virtual ",
                    }
                }
                if let Some(variant) = variant {
                    s += &variant.prefix;
                    s += &self.puzzle.name;
                    s += &variant.suffix;
                } else {
                    s += &self.puzzle.name;
                }

                if *average {
                    s += " Average";
                }

                let primary_filters = match variant {
                    Some(v) => v.primary_filters,
                    None => self.puzzle.primary_filters,
                };
                if *filters != primary_filters {
                    paren_modifiers.push(if *filters { "filters" } else { "no filters" });
                }

                let primary_macros = match variant {
                    Some(v) => v.primary_macros,
                    None => self.puzzle.primary_macros,
                };
                if *macros != primary_macros {
                    paren_modifiers.push(if *macros { "macros" } else { "no macros" });
                }
            }

            Category::Fmc { computer_assisted } => {
                s += &self.puzzle.name;
                s += " Fewest Moves";
                if *computer_assisted {
                    paren_modifiers.push("computer assisted");
                }
            }
        }

        if paren_modifiers.is_empty() {
            s
        } else {
            format!("{s} ({})", paren_modifiers.into_iter().join(", "))
        }
    }
}
