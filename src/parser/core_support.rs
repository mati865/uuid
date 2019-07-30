// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::parser;
use core::fmt;

impl From<parser::ParseError> for crate::Error {
    fn from(err: parser::ParseError) -> Self {
        crate::Error::Parse(err)
    }
}

impl<'a> fmt::Display for parser::ExpectedLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            parser::ExpectedLength::Any(crits) => {
                write!(f, "one of {:?}", crits)
            }
            parser::ExpectedLength::Exact(crit) => write!(f, "{}", crit),
            parser::ExpectedLength::Range { min, max } => {
                write!(f, "{}..{} inclusive", min, max)
            }
        }
    }
}

impl fmt::Display for parser::ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self._description())?;

        match *self {
            parser::ParseError::InvalidCharacter {
                expected,
                found,
                index,
                urn,
            } => {
                let urn_str = match urn {
                    parser::UrnPrefix::None => "",
                    parser::UrnPrefix::Optional => {
                        " an optional prefix of `urn:uuid:` followed by"
                    }
                    parser::UrnPrefix::Required => {
                        " a prefix of `urn:uuid` followed by"
                    }
                };

                write!(
                    f,
                    "expected{} {}, found {} at {}",
                    urn_str, expected, found, index
                )
            }
            parser::ParseError::InvalidGroupCount {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
            parser::ParseError::InvalidGroupLength {
                ref expected,
                found,
                group,
            } => write!(
                f,
                "expected {}, found {} in group {}",
                expected, found, group,
            ),
            parser::ParseError::InvalidLength {
                ref expected,
                found,
            } => write!(f, "expected {}, found {}", expected, found),
        }
    }
}
