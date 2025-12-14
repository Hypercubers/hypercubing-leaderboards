use std::{collections::BTreeMap, fmt};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::db::UserId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatedObject {
    pub ty: String,
    pub id: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl fmt::Display for UpdatedObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { ty, id, name } = self;
        write!(f, "{ty} #{id}")?;
        if let Some(name) = name {
            write!(f, " ({name:?})")?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditLogEvent {
    Started,
    Migrated {
        speed_verified: Option<(bool, UserId, String)>,
        fmc_verified: Option<(bool, UserId, String)>,
    },
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        object: Option<UpdatedObject>,
        fields: BTreeMap<String, String>,
    },
    Submitted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        object: Option<UpdatedObject>,
        fields: BTreeMap<String, String>,
    },
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        object: Option<UpdatedObject>,
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        fields: BTreeMap<String, [String; 2]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    SpeedVerified {
        old: Option<bool>,
        new: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    FmcVerified {
        old: Option<bool>,
        new: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    /// Error deserializing
    Unknown {
        json: serde_json::Value,
    },
}

impl From<serde_json::Value> for AuditLogEvent {
    fn from(json: serde_json::Value) -> Self {
        serde_json::from_value(json.clone()).unwrap_or(Self::Unknown { json })
    }
}

impl AuditLogEvent {
    pub fn display_public(&self) -> Option<String> {
        let mut ret = match self {
            AuditLogEvent::Started => return None,
            AuditLogEvent::Migrated {
                speed_verified,
                fmc_verified,
            } => {
                let mut msg = "Migrated from old schema".to_string();
                if let Some((verified, _user_id, user_name)) = speed_verified {
                    let verbed = if *verified { "accepted" } else { "rejected" };
                    msg += &format!("\nSpeed solve previously {verbed} by {user_name}");
                }
                if let Some((verified, _user_id, user_name)) = fmc_verified {
                    let verbed = if *verified { "accepted" } else { "rejected" };
                    msg += &format!("\nFMC solve previously {verbed} by {user_name}");
                }
                msg
            }
            AuditLogEvent::Added { object, fields: _ } => match object {
                Some(obj) => format!("Added {obj}"),
                None => "Added".to_string(),
            },
            AuditLogEvent::Submitted { object, fields: _ } => match object {
                Some(obj) => format!("Submitted {obj}"),
                None => "Submitted".to_string(),
            },
            AuditLogEvent::Updated {
                object,
                fields,
                comment: _,
            } => {
                if let Some(obj) = object {
                    format!("Updated {obj}")
                } else if fields.is_empty() {
                    "Updated".to_string()
                } else {
                    format!(
                        "Updated {}",
                        fields
                            .keys()
                            .map(|s| human_friendly_field_name(s))
                            .join(", ")
                    )
                }
            }
            AuditLogEvent::SpeedVerified {
                old: _,
                new,
                comment: _,
            } => match new {
                Some(true) => "Accepted speed solve",
                Some(false) => "Rejected speed solve",
                None => "Unverified speed solve",
            }
            .to_string(),
            AuditLogEvent::FmcVerified {
                old: _,
                new,
                comment: _,
            } => match new {
                Some(true) => "Accepted FMC solve",
                Some(false) => "Rejected FMC solve",
                None => "Unverified FMC solve",
            }
            .to_string(),
            AuditLogEvent::Unknown { .. } => return None,
        };

        if let Some(comment) = self.comment() {
            if ret.is_empty() {
                ret += "Added comment";
            }
            ret += ": ";
            ret += comment;
        }

        Some(ret)
    }

    pub fn display_full(&self) -> String {
        let mut ret = match self {
            AuditLogEvent::Started => "Began audit logs".to_string(),
            AuditLogEvent::Migrated {
                speed_verified,
                fmc_verified,
            } => {
                let mut ret = "Migrated from old schema".to_string();
                if let Some(speed_verified) = speed_verified {
                    ret += &format!("\nspeed_verified = {speed_verified:?}");
                }
                if let Some(fmc_verified) = fmc_verified {
                    ret += &format!("\nfmc_verified = {fmc_verified:?}");
                }
                ret
            }
            AuditLogEvent::Added { object, fields } => {
                (match object {
                    Some(obj) => format!("Added {obj}\n"),
                    None => "Added\n".to_string(),
                }) + &display_new_fields(fields)
            }
            AuditLogEvent::Submitted { object, fields } => {
                (match object {
                    Some(obj) => format!("Submitted {obj}\n"),
                    None => "Submitted\n".to_string(),
                }) + &display_new_fields(fields)
            }
            AuditLogEvent::Updated {
                object,
                fields,
                comment: _,
            } => match object {
                Some(obj) => format!("Updated {obj}\n{}", display_changed_fields(fields, true)),
                None => display_changed_fields(fields, false),
            },
            AuditLogEvent::SpeedVerified {
                old,
                new,
                comment: _,
            } => {
                format!("Changed speed_verified from {old:?} to {new:?}")
            }
            AuditLogEvent::FmcVerified {
                old,
                new,
                comment: _,
            } => {
                format!("Changed fmc_verified from {old:?} to {new:?}")
            }
            AuditLogEvent::Unknown { json } => {
                format!("unknown: {json:?}")
            }
        };

        if let Some(comment) = self.comment() {
            if !ret.is_empty() {
                ret += "\n";
            }
            ret += "Comment: ";
            ret += comment;
        }

        ret
    }

    fn comment(&self) -> &Option<String> {
        match self {
            AuditLogEvent::Started
            | AuditLogEvent::Migrated { .. }
            | AuditLogEvent::Added { .. }
            | AuditLogEvent::Submitted { .. }
            | AuditLogEvent::Unknown { .. } => &None,
            AuditLogEvent::Updated { comment, .. }
            | AuditLogEvent::SpeedVerified { comment, .. }
            | AuditLogEvent::FmcVerified { comment, .. } => comment,
        }
    }
}

fn display_new_fields(fields: &BTreeMap<String, String>) -> String {
    fields
        .iter()
        .map(|(k, v)| format!("    {k} = {v}"))
        .join("\n")
}

fn display_changed_fields(fields: &BTreeMap<String, [String; 2]>, indent: bool) -> String {
    let indent = if indent { "\t" } else { "" };
    fields
        .iter()
        .map(|(k, [old, new])| format!("{indent}Changed {k} from {old} to {new}"))
        .join("\n")
}

fn human_friendly_field_name(field_name: &str) -> String {
    let without_id_suffix = field_name.strip_suffix("_id").unwrap_or(field_name);
    match without_id_suffix {
        "average" | "blind" | "filters" | "macros" | "one_handed" | "computer_assisted" => {
            without_id_suffix.replace('_', "-") + " flag"
        }
        "speed cs" => "time".to_string(),
        "memo cs" => "memo time".to_string(),
        "video url" => "video URL".to_string(),
        _ => without_id_suffix.replace('_', " "),
    }
}
