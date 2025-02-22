use super::{ProgramQuery, SolveFlags, Variant, VariantQuery};

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
    pub fn url_query_params(&self) -> String {
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
                    ret += &format!("&average=true");
                }
                if *blind {
                    ret += &format!("&blind=true");
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
                if *variant != VariantQuery::Default {
                    ret += &format!("&variant={variant}");
                }
                if *program != ProgramQuery::Default {
                    ret += &format!("&program={program}");
                }
            }
            CategoryQuery::Fmc { computer_assisted } => {
                ret += "&event=fmc";
                if *computer_assisted {
                    ret += &format!("&computer_assisted=true");
                }
            }
        }
        ret
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
