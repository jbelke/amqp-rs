// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod header_struct;
mod header_encodable_impl;
mod common_impl;
mod method_enum;
mod method_impl;
mod method_content_impl;
mod method_default_impl;
mod method_encodable_impl;
mod method_from_impls;
mod method_payload_impl;
mod method_setter_impl;
mod method_struct;

use std::io;
use common::{Specs, Spec, Class};
use WriteRust;

use self::common_impl::CommonImplWriter;
use self::header_struct::HeaderStructWriter;
use self::header_encodable_impl::EncodableHeaderImplWriter;
use self::method_enum::MethodEnumWriter;
use self::method_impl::MethodImplWriter;
use self::method_content_impl::MethodContentImplWriter;
use self::method_default_impl::DefaultImplWriter;
use self::method_encodable_impl::EncodableMethodImplWriter;
use self::method_payload_impl::MethodPayloadImplWriter;
use self::method_setter_impl::MethodSetterImplWriter;
use self::method_struct::MethodStructWriter;
use self::method_from_impls::MethodFromImplsWriter;

pub struct SpecClassModuleWriter<'a> {
    specs: &'a Specs<'a>,
    spec: &'a Spec,
    class: &'a Class,
}

impl<'a> SpecClassModuleWriter<'a> {
    pub fn new(specs: &'a Specs<'a>, spec: &'a Spec, class: &'a Class) -> Self {
        SpecClassModuleWriter {
            specs: specs,
            spec: spec,
            class: class,
        }
    }
}

impl<'a> WriteRust for SpecClassModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter"));
        try!(writeln!(writer, "#![allow(too_many_arguments)]\n"));

        let header = HeaderStructWriter::new(self.class);
        try!(header.write_to(writer));

        let encodable_impl = EncodableHeaderImplWriter::new(self.class);
        try!(encodable_impl.write_rust_to(writer));

        for method in self.class.methods() {
            let common_impl= CommonImplWriter::new(self.specs, self.spec, self.class, method);
            try!(common_impl.write_rust_to(writer));

            let struct_writer = MethodStructWriter::new(method);
            try!(struct_writer.write_rust_to(writer));

            let inherit_impl = MethodImplWriter::new(method);
            try!(inherit_impl.write_rust_to(writer));

            let default_impl = DefaultImplWriter::new(method);
            try!(default_impl.write_rust_to(writer));

            let content_impl = MethodContentImplWriter::new(self.class, method);
            try!(content_impl.write_rust_to(writer));

            let encodable_impl = EncodableMethodImplWriter::new(method);
            try!(encodable_impl.write_rust_to(writer));

            let payload_impl = MethodPayloadImplWriter::new(self.class, method);
            try!(payload_impl.write_rust_to(writer));

            let setter_impl = MethodSetterImplWriter::new(self.specs, self.class, method);
            try!(setter_impl.write_rust_to(writer));

            let from_impls = MethodFromImplsWriter::new(self.class, method);
            try!(from_impls.write_rust_to(writer));
        }

        let method_enum = MethodEnumWriter::new(self.class);
        try!(method_enum.write_rust_to(writer));

        Ok(())
    }
}