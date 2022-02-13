// Copyright (c) 2022 Milen Dzhumerov

#[derive(Debug)]

/// A headermap entry consists of three strings: `key`, `prefix` and `suffix`.
/// Conceptually, each entry represents a map entry from `key` -> `prefix` + `suffix`.
///
/// They `key` is what appears in `#include` directives and gets mapped to an
/// actual path (i.e., `prefix` + `suffix`).
pub struct Entry<'a> {
    pub key: &'a str,
    pub prefix: &'a str,
    pub suffix: &'a str,
}
