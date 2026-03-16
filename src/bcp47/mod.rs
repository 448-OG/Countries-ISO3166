mod language_info;
pub use language_info::*;

mod multi_lang_parser;
pub use multi_lang_parser::*;

mod single_lang_parser;
pub use single_lang_parser::*;

#[cfg(all(feature = "small_keys", feature = "large_keys"))]
compile_error!(
    "The crate features `small_keys` and `large_keys` cannot be enabled at the same time"
);
