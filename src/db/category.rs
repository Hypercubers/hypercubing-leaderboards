use super::{ProgramQuery, PuzzleId, SolveFlags, Variant, VariantId, VariantQuery};

#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MainPageQuery {
    Speed {
        average: bool,
        blind: bool,
        filters: Option<bool>,
        macros: Option<bool>,
        one_handed: bool,
    },
    Fmc {
        computer_assisted: bool,
    },
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CategoryQuery {
    Speed {
        average: bool,
        blind: bool,
        filters: Option<bool>,
        macros: Option<bool>,
        one_handed: bool,
        variant: VariantQuery,
        program: ProgramQuery,
    },
    Fmc {
        computer_assisted: bool,
    },
}
impl Default for CategoryQuery {
    fn default() -> Self {
        Self::Speed {
            average: false,
            blind: false,
            filters: None,
            macros: None,
            one_handed: false,
            variant: VariantQuery::Default,
            program: ProgramQuery::Default,
        }
    }
}
impl CategoryQuery {
    pub fn url_query_params(&self, single_puzzle: bool) -> String {
        let mut ret = String::new();
        match self {
            CategoryQuery::Speed {
                average,
                blind,
                filters,
                macros,
                one_handed,
                variant,
                program,
            } => {
                if *average {
                    ret += "&average=true";
                }
                if *blind {
                    ret += "&blind=true";
                }
                if let Some(filters) = filters {
                    ret += &format!("&filters={filters}");
                }
                if let Some(macros) = macros {
                    ret += &format!("&macros={macros}");
                }
                if *one_handed {
                    ret += &format!("&one_handed={one_handed}");
                }
                if single_puzzle {
                    if *variant != VariantQuery::Default {
                        ret += &format!("&variant={variant}");
                    }
                    if *program != ProgramQuery::Default {
                        ret += &format!("&program={program}");
                    }
                } else {
                    if *variant != VariantQuery::All {
                        ret += &format!("&variant={variant}");
                    }
                    if *program != ProgramQuery::All {
                        ret += &format!("&program={program}");
                    }
                }
            }
            CategoryQuery::Fmc { computer_assisted } => {
                ret += "&event=fmc";
                if *computer_assisted {
                    ret += "&computer_assisted=true";
                }
            }
        }
        ret
    }

    pub(super) fn sql_score_column(&self) -> &'static str {
        match self {
            CategoryQuery::Speed { .. } => "speed_cs",
            CategoryQuery::Fmc { .. } => "move_count",
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum Category {
    Speed {
        average: bool,
        blind: bool,
        filters: bool,
        macros: bool,
        one_handed: bool,
        variant: Option<Variant>,
        material: bool,
    },
    Fmc {
        computer_assisted: bool,
    },
}
impl Category {
    pub fn new_speed(flags: SolveFlags, variant: Option<Variant>, material: bool) -> Self {
        Self::Speed {
            average: flags.average,
            blind: flags.blind,
            filters: flags.filters,
            macros: flags.macros,
            one_handed: flags.one_handed,
            variant,
            material,
        }
    }

    pub fn new_fmc(flags: SolveFlags) -> Self {
        Self::Fmc {
            computer_assisted: flags.computer_assisted,
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainPageCategory {
    Speed {
        puzzle: PuzzleId,
        variant: Option<VariantId>,
        material: bool,
    },
    Fmc {
        puzzle: PuzzleId,
    },
}
// impl Linkable for MainPageCategory {
//     fn relative_url(&self) -> String {
//         self.event().relative_url()
//     }

//     fn md_text(&self) -> String {
//         self.name()
//     }
// }
// impl MainPageCategory {
//     pub fn name(&self) -> String {
//         self.event().name()
//     }

//     pub fn event(&self) -> Event {
//         match self {
//             MainPageCategory::StandardSpeed {
//                 puzzle,
//                 variant,
//                 average,
//                 blind,
//                 one_handed,
//             } => Event {
//                 puzzle: puzzle.clone(),
//                 category: Category::Speed {
//                     average: *average,
//                     blind: *blind,
//                     filters: puzzle.primary_filters,
//                     macros: puzzle.primary_macros,
//                     one_handed: *one_handed,
//                     variant: variant.clone(),
//                     material: match variant {
//                         Some(v) => v.material_by_default,
//                         None => false,
//                     },
//                 },
//             },
//             MainPageCategory::SpecialSpeed { puzzle, category } => todo!(),
//             MainPageCategory::StandardFmc {
//                 puzzle,
//                 computer_assisted,
//             } => Event {
//                 puzzle: puzzle.clone(),
//                 category: Category::Fmc {
//                     computer_assisted: *computer_assisted,
//                 },
//             },
//         }
//     }
// }
