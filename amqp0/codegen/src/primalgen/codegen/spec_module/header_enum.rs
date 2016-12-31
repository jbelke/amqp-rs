// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use CodeGenerator;
use primalgen::Spec;

pub struct HeaderEnumWriter<'a> {
    spec: &'a Spec<'a>,
}

impl<'a> CodeGenerator for HeaderEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n// Generated by primalgen::codegen::header_enum::HeaderEnumWriter"));
        try!(writeln!(writer, "#[derive(Debug)]"));
        try!(writeln!(writer, "pub enum SpecHeader<'a> {{"));
        for class in self.spec.classes() {
            let pascal_case = class.pascal_case();
            if class.fields().is_empty() {
                try!(writeln!(writer, "{},", pascal_case));
            }
            else {
                let snake_case = class.snake_case();
                try!(writeln!(writer, "{}({}::Header<'a>),", pascal_case, snake_case));
            }
        }
        try!(writeln!(writer, "}} // enum SpecHeader"));

        try!(self.write_encodable_impl(writer));

        Ok(())
    }
}

impl<'a> HeaderEnumWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let has_lifetimes = spec.classes().iter()
            .any(|class| class.has_method_lifetimes());

        HeaderEnumWriter {
            spec: spec,
        }
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\nimpl<'a> ::Encodable for SpecHeader<'a> {{"));
        try!(writeln!(writer, "\nfn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        for class in self.spec.classes() {
            let matcher = if class.fields().is_empty() {
                ("", "0")
            }
            else {
                ("(ref header)", "::Encodable::encoded_size(header)")
            };
            try!(writeln!(writer, "SpecHeader::{}{} => {},", class.pascal_case(), matcher.0, matcher.1));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn encoded_size"));
        try!(writeln!(writer, "\n}} // F::Encodable for SpecMethod"));

        Ok(())
    }
}