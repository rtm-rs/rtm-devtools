//! Assorted testing utilities.
//!
//! Most notable:
//!
//! * `assert_eq_text!`: Rich text comparison, which outputs a diff then panics.

pub use dissimilar::diff as __diff;

/// Asserts two strings are equal, otherwise sends a diff to stderr then panics.
///
/// The rich diff shows changes from the "original" left string to the "actual"
/// right string.
///
/// All arguments starting from and including the 3rd one are passed to
/// `eprintln!()` macro in case of text inequality.
///
/// # Panics
///
/// The macro will panic in case of text inequality.
///
/// # License
///
/// SPDX-License-Identifier: Apache-2.0 OR MIT
/// Copyright 2022 rust-analyzer project authors
///
#[macro_export]
macro_rules! assert_eq_text {
    ($left:expr, $right:expr) => {
        assert_eq_text!($left, $right,)
    };
    ($left:expr, $right:expr, $($tt:tt)*) => {{
        let left = $left;
        let right = $right;
        if left != right {
            if left.trim() == right.trim() {
                std::eprintln!("Left:\n{:?}\n\nRight:\n{:?}\n\nWhitespace difference\n", left, right);
            } else {
                let diff = $crate::__diff(left, right);
                std::eprintln!("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n", left, right, $crate::format_diff(diff));
            }
            std::eprintln!($($tt)*);
            panic!("text differs");
        }
    }};
}

pub fn format_diff(chunks: Vec<dissimilar::Chunk>) -> String {
    let mut buf = String::new();
    for chunk in chunks {
        let formatted = match chunk {
            dissimilar::Chunk::Equal(text) => text.into(),
            dissimilar::Chunk::Delete(text) => format!("\x1b[41m{}\x1b[0m", text),
            dissimilar::Chunk::Insert(text) => format!("\x1b[42m{}\x1b[0m", text),
        };
        buf.push_str(&formatted);
    }
    buf
}

pub fn normalize_spans<R, S>(records: R) -> std::string::String
where
    S: Sized,
    R: AsRef<[S]> + std::fmt::Debug,
{
    let pre = format!("{records:#?}");
    //let pre = records.to_string();
    let re1 = regex::Regex::new(r"begin_unix_time_ns: \d+,").unwrap();
    let re2 = regex::Regex::new(r"duration_ns: \d+,").unwrap();
    let int: std::string::String = re1.replace_all(&pre, r"begin_unix_time_ns: \d+,").into();
    let actual: std::string::String = re2.replace_all(&int, r"duration_ns: \d+,").into();
    actual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_text() {
        assert_eq_text!("abc", "abc");
    }

    #[test]
    #[should_panic]
    fn test_mismatching_text() {
        assert_eq_text!("a", "abc");
    }
}
