// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//!
//! Basic representation of AMQP dynamic types, tables, and lists
//!
//! AMQP has several dynamic-typed arguments:
//!
//!  * A no-vale type (Void)
//!  * 11 primitive types (i8-64, u8-64, f32-64)
//!  * A numeric decimal type
//!  * 2 "string" types (short and long)
//!  * A "timestamp" type
//!  * 2 container types (`List` and `Table`)
//!
//! The main types are `Value`, `List`, and `Table`
//!

const MAX_SHORTSTR_LEN: usize = 255;

mod value;
mod list;
mod table;

pub use self::list::List;
pub use self::table::{Table, TableEntry, TableEntries};
pub use self::value::Value;
