// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use common::{Domain, ClassMethod, Field};
use WriteRust;

impl<'a> WriteRust for EncodableMethodImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { ("<'a>") } else { ("") };

        try!(writeln!(writer, "\nimpl{1} ::Encodable for {0}{1} {{", self.method.pascal_case(), lifetimes));
        try!(self.write_encoded_size(writer));
        try!(self.write_encoded_writer(writer));
        try!(writeln!(writer, "}} // impl Encodable"));

        try!(writeln!(writer, "\n#[test]\n\
            fn test_{snake_case}_encodable_bytes_written_matches_len() {{\n\
                let payload: {struct_name} = Default::default();\n\
                let expected_len = ::Encodable::encoded_size(&payload);\n\
                let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));\n\
                ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();\n\
                let payload = writer.into_inner();\n\
                \n\
                if payload.len() != expected_len {{\n\
                    panic!(\n\
                        \"Expected payload len {{}}, got {{}}, {{:?}}\",\n\
                        expected_len,\n\
                        payload.len(),\n\
                        &payload[..]\n\
                    );\n\
                }}\n\
            }}\n\n",
            snake_case = self.method.snake_case(),
            struct_name = self.method.pascal_case()
        ));

        Ok(())
    }
}

pub struct EncodableMethodImplWriter<'a> {
    method: &'a ClassMethod,
}

impl<'a> EncodableMethodImplWriter<'a> {
    pub fn new(method: &'a ClassMethod) -> Self {
        EncodableMethodImplWriter {
            method: method,
        }
    }

    fn write_bit_fields<W>(&self, writer: &mut W, fields: &mut Vec<&Field>) -> io::Result<()>
        where W: io::Write
    {
        let has_usable_fields = fields.iter().any(|field| !field.is_reserved());
        let num_padding_bytes = if fields.len() % 8 != 0 { 1 } else { 0 };
        let num_bits = (fields.len() / 8 + num_padding_bytes) * 8;

        try!(write!(writer, "try!(::Encodable::write_encoded_to(&"));
        if has_usable_fields {
            let mut bit_num = num_bits - 1;
            try!(writeln!(writer, "{{\nlet mut bits = ::bit_vec::BitVec::from_elem({}, false);", num_bits));

            for field in fields.drain(..) {
                if field.is_reserved() {
                    try!(writeln!(writer, "// bit {} reserved ({})", bit_num, field.var_name()));
                } else {
                    try!(writeln!(writer, "bits.set({}, self.{});", bit_num, field.var_name()));
                }
                bit_num -= 1;
            }
            try!(writeln!(writer, "bits\n}},"));
        } else {
            try!(match num_bits {
                8 => write!(writer, "0u8, "),
                16 => write!(writer, "0u16, "),
                32 => write!(writer, "0u32, "),
                64 => write!(writer, "0u64, "),
                _ => write!(writer, "::bit_vec::BitVec::from_elem({}, false), ", num_bits),
            });
            fields.clear();
        }
        try!(writeln!(writer, "writer));"));

        Ok(())
    }

    pub fn write_encoded_size<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let static_size_bits = self.method.fields().iter()
            .filter(|field| field.is_reserved() || field.ty().dynamic_bit_method().is_none())
            .map(|field| field.ty().num_bits_static())
            .sum::<usize>();

        let static_size = static_size_bits / 8 + if static_size_bits % 8 > 0 { 1 } else { 0 };
        try!(write!(writer, "fn encoded_size(&self) -> usize {{\n{}", static_size));

        for field in self.method.fields() {
            if field.is_reserved() || field.ty().dynamic_bit_method().is_none() {
                continue;
            }

            try!(write!(writer, " + ::Encodable::encoded_size(&self.{})", field.var_name()));
        }
        try!(writeln!(writer, "\n}} // encoded_size"));

        Ok(())
    }

    fn write_encoded_writer<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.method.fields().is_empty() {
            try!(writeln!(
                writer,
                "fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()> \n\
                    where W: ::std::io::Write\n\
                {{\n\
                    ::std::result::Result::Ok(())\n\
                }}\n\
                "
            ));
            return Ok(())
        }

        try!(writeln!(writer, "fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>"));
        try!(writeln!(writer, "where W: ::std::io::Write"));
        try!(writeln!(writer, "{{"));

        let mut bit_fields = Vec::with_capacity(8);

        for field in self.method.fields() {

            if Domain::Bit == *field.ty() {
                bit_fields.push(field);
                continue;
            } else if !bit_fields.is_empty() {
                try!(self.write_bit_fields(writer, &mut bit_fields))
            }

            if field.is_reserved() {
                try!(writeln!(
                    writer,
                    "try!(::Encodable::write_encoded_to(&{}, writer)); // reserved: {}",
                    field.ty().empty_encoded_value(),
                    field.var_name()
                ));
            } else {
                try!(writeln!(
                    writer,
                    "try!(::Encodable::write_encoded_to(&self.{0}, writer)); // {0}",
                    field.var_name()
                ));
            }
        }

        if !bit_fields.is_empty() {
            try!(self.write_bit_fields(writer, &mut bit_fields));
        }

        try!(writeln!(writer, ""));

        try!(writeln!(writer, "::std::result::Result::Ok(())"));
        try!(writeln!(writer, "}} // fn write_encoded_to()"));

        Ok(())
    }
}
