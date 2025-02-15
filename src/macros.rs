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
            sqlx::Type,
            Serialize,
            Deserialize,
            From,
            Into,
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
        pub struct $id_struct_name(pub i32);
    };
}
