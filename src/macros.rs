macro_rules! id_struct {
    ($id_struct_name:ident, $struct_name:ident) => {
        #[doc = concat!("Database ID for a [`", stringify!($struct_name), "`].")]
        #[derive(
            Serialize,
            Deserialize,
            Encode,
            Decode,
            From,
            Into,
            Debug,
            Copy,
            Clone,
            PartialEq,
            Eq,
            Hash,
        )]
        #[repr(transparent)]
        pub struct $id_struct_name(pub i32);
    };
}
