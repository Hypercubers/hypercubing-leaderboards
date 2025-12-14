macro_rules! id_struct {
    ($id_struct_name:ident, $struct_name:ident $(,)?) => {
        id_struct!(
            $id_struct_name,
            concat!("[`", stringify!($struct_name), "`]"),
        );
    };
    ($id_struct_name:ident, $noun:expr $(,)?) => {
        #[doc = concat!("Database ID for a ", $noun, ".")]
        #[derive(
            sqlx::Encode,
            sqlx::Decode,
            serde::Serialize,
            serde::Deserialize,
            derive_more::From,
            derive_more::Into,
            Debug,
            Copy,
            Clone,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $id_struct_name(pub i32);

        impl std::str::FromStr for $id_struct_name {
            type Err = <i32 as std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.strip_prefix('#').unwrap_or(s);
                i32::from_str(s).map(Self)
            }
        }

        impl std::fmt::Display for $id_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{}", self.0)
            }
        }
    };
}

macro_rules! impl_try_from_multipart_wrapper {
    ($outer:ident($inner:ty)) => {
        impl ::axum_typed_multipart::TryFromMultipart for $outer {
            fn try_from_multipart<'life0, 'async_trait>(
                multipart: &'life0 mut Multipart,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<Output = Result<Self, TypedMultipartError>>
                        + ::core::marker::Send
                        + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                ::std::boxed::Box::pin(
                    <$inner>::try_from_multipart(multipart).map(|result| result.map(Self)),
                )
            }
        }
    };
}

/// Usage:
///
/// - Created:
///     - `audit_log_msg!(Type, new_data, id, [field1, field2, field3])`
///     - `audit_log_msg!(new_data, [field1, field2, field3])`
/// - Updated:
///     - `audit_log_msg!(Type, old_data => new_data, [field1, field2, field3])`
///     - `audit_log_msg!(old_data => new_data, [field1, field2, field3])`
macro_rules! audit_log_msg {
    ($old:expr => $new:expr, $fields:tt $(,)?) => {{
        let old = &$old;
        let new = &$new;
        let mut log_message = String::new();
        audit_log_msg!(@append(log_message), (old => new), $fields);
        log_message
    }};

    ($new:expr, $fields:tt $(,)?) => {{
        let new = &$new;
        let mut log_message = "Created".to_string();
        audit_log_msg!(@append(log_message), (new), $fields);
        log_message
    }};

    ($type:ty, $old:expr => $new:expr, $fields:tt $(,)?) => {{
        let old = &$old;
        let new = &$new;
        let mut log_message = format!("Updated {} {} named {:?}", stringify!($type), old.id, old.name);
        audit_log_msg!(@append(log_message), (old => new), $fields);
        log_message
    }};

    ($type:ty, $new:expr, $id:expr, $fields:tt $(,)?) => {{
        let new = &$new;
        let mut log_message = format!("Created {} #{} named {:?}", stringify!($type), $id, new.name);
        audit_log_msg!(@append(log_message), (new), $fields);
        log_message
    }};

    (@append($log_message:ident), $data:tt, [$($field:ident),* $(,)?]) => {
        let indent = !$log_message.is_empty();
        $(
            audit_log_msg!(@append_line($log_message, indent), $data, $field);
        )*
    };

    (@append_whitespace($log_message:ident, $indent:expr)) => {
        if !$log_message.is_empty() {
            $log_message += "\n";
        }
        if $indent {
            $log_message += "\t";
        }
    };

    (@append_line($log_message:ident, $indent:expr), ($old:ident => $new:ident), $field:ident) => {
        if $old.$field != $new.$field {
            audit_log_msg!(@append_whitespace($log_message, $indent));
            $log_message += &format!(
                "Changed {} from {:?} to {:?}",
                stringify!($field),
                $old.$field,
                $new.$field,
            );
        }
    };

    (@append_line($log_message:ident, $indent:expr), ($new:ident), $field:ident) => {
        audit_log_msg!(@append_whitespace($log_message, $indent));
        $log_message += &format!("{} = {:?}", stringify!($field), $new.$field);
    };
}

macro_rules! fields_map {
    ($value:expr, [$($field:ident),* $(,)?] $(,)?) => {{
        let value = &$value;
        std::collections::BTreeMap::from_iter(
            [$((stringify!($field).to_string(), format!("{:?}", value.$field))),*]
        )
    }};
}

macro_rules! changed_fields_map {
    ($old:expr, $new:expr, [$($field:ident),* $(,)?] $(,)?) => {{
        let old = &$old;
        let new = &$new;
        let mut ret = std::collections::BTreeMap::new();
        $(
            if old.$field != new.$field {
                ret.insert(
                    stringify!($field).to_string(),
                    [format!("{:?}", old.$field), format!("{:?}", new.$field)],
                );
            }
        )*
        ret
    }};
}

macro_rules! updated_object {
    ($type:ty, $value:expr) => {{
        let value = &$value;
        $crate::db::UpdatedObject {
            ty: stringify!($type).to_string(),
            id: value.id.0,
            name: Some(value.name.to_string()),
        }
    }};
    ($type:ty, $id:expr, $value:expr) => {{
        $crate::db::UpdatedObject {
            ty: stringify!($type).to_string(),
            id: $id,
            name: Some($value.name.to_string()),
        }
    }};
}
