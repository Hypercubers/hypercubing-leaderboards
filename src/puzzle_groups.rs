use std::collections::HashMap;

pub const PUZZLE_GROUPS_SPEC: &str = include_str!("../puzzle_groups.txt");

lazy_static! {
    pub static ref PUZZLE_GROUPS: PuzzleGroups = PuzzleGroups::from_spec(PUZZLE_GROUPS_SPEC);
}

pub struct PuzzleGroups {
    pub group_names_in_order: Vec<&'static str>,
    pub hsc_id_to_group_name: HashMap<&'static str, &'static str>,
    pub default_group_name: &'static str,
}

impl PuzzleGroups {
    fn from_spec(spec: &'static str) -> Self {
        let mut group_names_in_order = vec![];
        let mut hsc_id_to_group_name = HashMap::new();
        let mut default_group_name = "unknown";
        for line in spec.lines() {
            if let Some((group_name, hsc_ids_list)) = line.split_once('=') {
                let group_name = group_name.trim();
                group_names_in_order.push(group_name);
                for id in hsc_ids_list.split(',') {
                    let hsc_id = id.trim();
                    if hsc_ids_list.trim() == "*" {
                        default_group_name = group_name;
                    } else {
                        hsc_id_to_group_name.entry(hsc_id).or_insert(group_name);
                    }
                }
            }
        }

        Self {
            group_names_in_order,
            hsc_id_to_group_name,
            default_group_name,
        }
    }
}
