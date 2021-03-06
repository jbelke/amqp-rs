// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::Class;

pub struct MethodEnumWriter<'a> {
    class: &'a Class,
}

impl<'a> WriteRust for MethodEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.class.has_method_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "\n#[derive(Debug)]"));
        try!(writeln!(writer, "pub enum ClassMethod{} {{", lifetimes));
        for method in self.class.methods() {
            let lifetimes = if method.has_lifetimes() { "<'a>" } else {""};
            try!(writeln!(writer, "{0}({0}{1}),", method.pascal_case(), lifetimes));
        }
        try!(writeln!(writer, "}} // enum ClassMethod\n"));

        try!(self.write_encodable_impl(writer));
        try!(self.write_payload_method_payload_impl(writer));

        Ok(())
    }
}

impl<'a> MethodEnumWriter<'a> {
    pub fn new(class: &'a Class) -> Self {
        MethodEnumWriter {
            class: class,
        }
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.class.has_method_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "\nimpl{0} ::Encodable for ClassMethod{0} {{", lifetimes));
        try!(writeln!(writer, "\nfn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        for method in self.class.methods() {
            try!(writeln!(writer, "ClassMethod::{}(ref method) => ::Encodable::encoded_size(method),", method.pascal_case()));
        }
        try!(writeln!(writer, "\n}} // match *self"));
        try!(writeln!(writer, "\n}} // fn encoded_size"));

        try!(writeln!(writer, "fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>"));
        try!(writeln!(writer, "where W: ::std::io::Write"));
        try!(writeln!(writer, "{{"));
        try!(writeln!(writer, "unimplemented!()"));
        try!(writeln!(writer, "}} // fn write_encoded_to()"));

        try!(writeln!(writer, "\n}} // impl ::Encodable for ClassMethod{0}", lifetimes));

        Ok(())
    }

    fn write_payload_method_payload_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let methods = [
            ("class", "::Class"),
            ("class_id", "u16"),
            ("class_name", "&'static str"),
            ("method_id", "u16"),
            ("method_name", "&'static str"),
        ];

        let lifetimes = if self.class.has_method_lifetimes() { "<'a>" } else { "" };

        // ProtocolMethod
        try!(writeln!(writer, "\nimpl<'a> ::ProtocolMethodPayload for ClassMethod{} {{", lifetimes));

        for &(method_name, method_return) in &methods {
            try!(writeln!(writer, "\nfn {}(&self) -> {} {{", method_name, method_return));
            try!(writeln!(writer, "match *self {{"));
            for method in self.class.methods() {
                try!(writeln!(
                    writer,
                    "ClassMethod::{}(ref method) => ::ProtocolMethodPayload::{}(method),",
                    method.pascal_case(),
                    method_name
                ));
            }
            try!(writeln!(writer, "\n}} // match *self"));
            try!(writeln!(writer, "\n}} // fn {}", method_name));
        }

        try!(writeln!(writer, "\n}} // impl ProtocolMethodPayload for ClassMethod"));

        Ok(())
    }
}