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

macro_rules! format_log_message {
    ($type:ty, $old:expr => $new:expr, [$($field:ident),* $(,)?] $(,)?) => {{
        let old = &$old;
        let new = &$new;
        let mut log_message = format!("Updated {} {} named {:?}", stringify!($type), old.id, old.name);
        $(
            if old.$field != new.$field {
                log_message += &format!(
                    "\nChanged {} from {:?} to {:?}",
                    stringify!($field),
                    old.$field,
                    new.$field,
                );
            }
        )*
        log_message
    }};

    ($type:ty, $new:expr, $id:expr, [$($field:ident),* $(,)?] $(,)?) => {{
        let new = &$new;
        let mut log_message = format!("Created {} #{} named {:?}", stringify!($type), $id, new.name);
        $(
            log_message += &format!("\n    {} = {:?}", stringify!($field), new.$field);
        )*
        log_message
    }};
}
