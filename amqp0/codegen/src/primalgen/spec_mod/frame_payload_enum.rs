// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;
use specs::Spec;

use CodeGenerator;

pub struct FramePayloadEnumWriter {
    frame_types: Vec<ClassModFrameType>,
}

enum ClassModFrameType {
    Method(String),
    Header(String),
    Bytes(String),
    Empty(String),
}

impl CodeGenerator for FramePayloadEnumWriter {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(self.write_header(writer));
        try!(writeln!(writer, "#[derive(Debug)]"));
        try!(writeln!(writer, "pub enum FramePayload<'a> {{"));
        for frame_type in &self.frame_types {
            try!(frame_type.write_enum_definition(writer));
        }
        try!(writeln!(writer, "}} // enum FramePayload"));

        try!(self.write_encodable_impl(writer));

        Ok(())
    }
}

impl FramePayloadEnumWriter {
    pub fn new(spec: &Spec) -> Self {
        let frame_types = spec.frame_types().keys()
            .map(|name| {
                let name = if name.starts_with("frame") { &name[6..] } else { name };
                ClassModFrameType::new(name.to_pascal_case())
            })
            .collect::<Vec<_>>();

        FramePayloadEnumWriter {
            frame_types: frame_types,
        }
    }

    fn write_header<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "\n// Generated by primalgen::codegen::spec_module::frame_payload_enum::FramePayloadEnumWriter"));

        Ok(())
    }

    fn write_encodable_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let (method_types, others) = self.frame_types.iter()
            .filter(|c| c.has_payload())
            .partition::<Vec<_>, _>(|frame_type| frame_type.is_method());

        let (header_types, body_types) = others.into_iter()
            .partition::<Vec<_>, _>(|frame_type| frame_type.is_header());

        let (method_variants, header_variants, body_variants) = (
            method_types.iter().map(|f| f.format_match("ref method")).collect::<Vec<_>>().join(" | "),
            header_types.iter().map(|f| f.format_match("ref header")).collect::<Vec<_>>().join(" | "),
            body_types.iter().map(|f| f.format_match("ref body")).collect::<Vec<_>>().join(" | "),
        );

        try!(self.write_header(writer));
        try!(writeln!(writer, "impl<'a> ::Encodable for FramePayload<'a> {{"));
        try!(writeln!(writer, "fn encoded_size(&self) -> usize {{"));
        try!(writeln!(writer, "match *self {{"));
        try!(writeln!(writer, "{} => ::Encodable::encoded_size(method),", method_variants));
        try!(writeln!(writer, "{} => ::Encodable::encoded_size(header),", header_variants));
        try!(writeln!(writer, "{} => body.len(),", body_variants));
        try!(writeln!(writer, "_ => 0"));
        try!(writeln!(writer, "}} // match "));
        try!(writeln!(writer, "}} // fn encoded_size()"));
        try!(writeln!(writer, "}} // impl Encodable"));

        Ok(())
    }

    fn write_protocol_frame_payload_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        // impl ProtocolFramePayload
        try!(self.write_header(writer));
        try!(writeln!(writer, "impl<'a> ::ProtocolFramePayload<'a> for FramePayload<'a> {{"));
        try!(writeln!(writer, "type Method = SpecMethod<'a>;"));

        // fn as_method
        try!(writeln!(writer, "fn as_method(&self) -> Option<&SpecMethod<'a>> {{"));
        try!(writeln!(writer, "if let FramePayload::Method(ref method) = *self {{"));
        try!(writeln!(writer, "Some(method)"));
        try!(writeln!(writer, "}} else {{"));
        try!(writeln!(writer, "None"));
        try!(writeln!(writer, "}} // if "));
        try!(writeln!(writer, "}} // fn as_method()"));

        try!(writeln!(writer, "}} // impl ::ProtocolFramePayload for FramePayload"));

        Ok(())
    }
}

impl ClassModFrameType {
    fn new(frame_type: String) -> Self {
        if frame_type.ends_with("Method") {
            ClassModFrameType::Method(frame_type)
        }
        else if frame_type.ends_with("Header") {
            ClassModFrameType::Header(frame_type)
        }
        else if frame_type.ends_with("Body") || frame_type.as_str() == "Trace" {
            ClassModFrameType::Bytes(frame_type)
        }
        else {
            ClassModFrameType::Empty(frame_type)
        }
    }

    fn write_enum_definition<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        match *self {
            ClassModFrameType::Method(ref name) => writeln!(writer, "{}(SpecMethod<'a>),", name),
            ClassModFrameType::Header(ref name) => writeln!(writer, "{}(SpecHeader<'a>),", name),
            ClassModFrameType::Bytes(ref name) => writeln!(writer, "{}(&'a [u8]),", name),
            ClassModFrameType::Empty(ref name) => writeln!(writer, "{},", name),
        }
    }

    fn format_match(&self, captured_name: &str) -> String {
        match *self {
            ClassModFrameType::Method(ref name)
            | ClassModFrameType::Header(ref name)
            | ClassModFrameType::Bytes(ref name)   => format!("FramePayload::{}({})", name, captured_name),

            ClassModFrameType::Empty(ref name)    => format!("FramePayload::{}", name),
        }
    }

    fn has_payload(&self) -> bool {
        match *self {
            ClassModFrameType::Method(_) | ClassModFrameType::Header(_) | ClassModFrameType::Bytes(_) => true,
            _ => false,
        }
    }

    fn is_method(&self) -> bool {
        match * self {
            ClassModFrameType::Method(_) => true,
            _ => false,
        }
    }

    fn is_header(&self) -> bool {
        match *self {
            ClassModFrameType::Header(_) => true,
            _ => false,
        }
    }
}