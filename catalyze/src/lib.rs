#![doc = include_str!("../../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::missing_panics_doc, // TODO: remove after todo!()s are removed
    clippy::missing_errors_doc, // TODO: remove when I get around to documenting
	clippy::must_use_candidate, // TODO: remove once the API is settled
    clippy::wildcard_imports,
    clippy::module_inception,
	clippy::struct_excessive_bools,
    clippy::missing_const_for_fn
)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

use std::fmt::Display;

pub mod ast;
pub mod error;
pub mod generator;

type HashMap<K, V> = ahash::HashMap<K, V>;
type HashSet<V> = ahash::HashSet<V>;

fn to_i32<T>(value: T) -> i32
where
    T: TryInto<i32>,
    T::Error: Display,
{
    value
        .try_into()
        .unwrap_or_else(|err| panic!("value cannot be converted to i32: {err}"))
}
