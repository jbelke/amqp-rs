// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::exchange::DeclareMethod<'a> for ::Amqp9_1 {
    type Payload = Declare<'a>;
} // impl<'a> ::method::exchange::DeclareMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Declare<'a> {
    exchange: ::std::borrow::Cow<'a, str>,
    ty: ::std::borrow::Cow<'a, str>,
    passive: bool,
    durable: bool,
    no_wait: bool,
    arguments: ::field::TableEntries<'a>,
} // struct Declare<'a>

impl<'a> Declare<'a> {
    pub fn new<E, T, A>(exchange: E,
                        ty: T,
                        passive: bool,
                        durable: bool,
                        no_wait: bool,
                        arguments: A)
                        -> Self
        where E: Into<::std::borrow::Cow<'a, str>>,
              T: Into<::std::borrow::Cow<'a, str>>,
              A: Into<::field::TableEntries<'a>>
    {
        Declare {
            exchange: exchange.into(),
            ty: ty.into(),
            passive: passive,
            durable: durable,
            no_wait: no_wait,
            arguments: arguments.into(),
        } // Declare
    } // fn new()
    impl_properties! {
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(ty, ty_mut, set_ty) -> Cow<str>,
(passive, set_passive) -> bool,
(durable, set_durable) -> bool,
(no_wait, set_no_wait) -> bool,
(arguments, arguments_mut, set_arguments) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Declare<'a>
impl<'a> Default for Declare<'a> {
    fn default() -> Self {
        Declare::new("", "", false, false, false, ::field::TableEntries::new())
    } // fn default()
} // impl Default for Declare

impl<'a> ::Encodable for Declare<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.exchange) + ::Encodable::encoded_size(&self.ty) +
        ::Encodable::encoded_size(&self.arguments)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&0u16, writer)); // reserved: reserved_1
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&self.ty, writer)); // ty
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.passive);
                                               bits.set(6, self.durable);
                                               // bit 5 reserved (reserved_2)
                                               // bit 4 reserved (reserved_3)
                                               bits.set(3, self.no_wait);
                                               bits
                                           },
                                           writer));
        try!(::Encodable::write_encoded_to(&self.arguments, writer)); // arguments

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_declare_encodable_bytes_written_matches_len() {
    let payload: Declare = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for Declare<'a> {
    fn class(&self) -> ::Class {
        ::Class::Exchange
    }
    fn class_id(&self) -> u16 {
        40
    }
    fn class_name(&self) -> &'static str {
        "exchange"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "declare"
    }
} // impl ::ProtocolMethodPayload for Declare<'a>
impl<'a> ::method::exchange::SetDeclareMethodFields<'a> for Declare<'a> {
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_ty<V>(&mut self, ty: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_ty(ty.into())
    } // set_ty()
    fn set_passive(&mut self, passive: bool) {
        self.set_passive(passive)
    } // set_passive()
    fn set_durable(&mut self, durable: bool) {
        self.set_durable(durable)
    } // set_durable()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_arguments<V>(&mut self, arguments: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_arguments(arguments.into())
    } // set_arguments()
} // impl<'a> ::method::exchange::SetDeclareMethodFields<'a> for Declare<'a>
impl<'a> From<Declare<'a>> for ClassMethod<'a> {
    fn from(from: Declare<'a>) -> Self {
        ClassMethod::Declare(from)
    } // fn from()
} // impl From<Declare<'a>> for ClassMethod

impl<'a> From<Declare<'a>> for super::SpecMethod<'a> {
    fn from(from: Declare<'a>) -> Self {
        super::SpecMethod::Exchange(from.into())
    } // fn default()
} // impl From<Declare<'a>> for ::super::SpecMethod
impl ::method::exchange::DeclareOkMethod for ::Amqp9_1 {
    type Payload = DeclareOk;
} // impl ::method::exchange::DeclareOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeclareOk;

impl DeclareOk {
    pub fn new() -> Self {
        DeclareOk
    } // fn new()
} // impl DeclareOk
impl Default for DeclareOk {
    fn default() -> Self {
        DeclareOk::new()
    } // fn default()
} // impl Default for DeclareOk

impl ::Encodable for DeclareOk {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_declare_ok_encodable_bytes_written_matches_len() {
    let payload: DeclareOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for DeclareOk {
    fn class(&self) -> ::Class {
        ::Class::Exchange
    }
    fn class_id(&self) -> u16 {
        40
    }
    fn class_name(&self) -> &'static str {
        "exchange"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "declare-ok"
    }
} // impl ::ProtocolMethodPayload for DeclareOk
impl<'a> From<DeclareOk> for ClassMethod<'a> {
    fn from(from: DeclareOk) -> Self {
        ClassMethod::DeclareOk(from)
    } // fn from()
} // impl From<DeclareOk> for ClassMethod

impl From<DeclareOk> for super::SpecMethod<'static> {
    fn from(from: DeclareOk) -> Self {
        super::SpecMethod::Exchange(from.into())
    } // fn default()
} // impl From<DeclareOk> for ::super::SpecMethod
impl<'a> ::method::exchange::DeleteMethod<'a> for ::Amqp9_1 {
    type Payload = Delete<'a>;
} // impl<'a> ::method::exchange::DeleteMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Delete<'a> {
    exchange: ::std::borrow::Cow<'a, str>,
    if_unused: bool,
    no_wait: bool,
} // struct Delete<'a>

impl<'a> Delete<'a> {
    pub fn new<E>(exchange: E, if_unused: bool, no_wait: bool) -> Self
        where E: Into<::std::borrow::Cow<'a, str>>
    {
        Delete {
            exchange: exchange.into(),
            if_unused: if_unused,
            no_wait: no_wait,
        } // Delete
    } // fn new()
    impl_properties! {
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(if_unused, set_if_unused) -> bool,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Delete<'a>
impl<'a> Default for Delete<'a> {
    fn default() -> Self {
        Delete::new("", false, false)
    } // fn default()
} // impl Default for Delete

impl<'a> ::Encodable for Delete<'a> {
    fn encoded_size(&self) -> usize {
        3 + ::Encodable::encoded_size(&self.exchange)
    } // encoded_size
    fn write_encoded_to<W>(&self, writer: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        try!(::Encodable::write_encoded_to(&0u16, writer)); // reserved: reserved_1
        try!(::Encodable::write_encoded_to(&self.exchange, writer)); // exchange
        try!(::Encodable::write_encoded_to(&{
                                               let mut bits = ::bit_vec::BitVec::from_elem(8,
                                                                                           false);
                                               bits.set(7, self.if_unused);
                                               bits.set(6, self.no_wait);
                                               bits
                                           },
                                           writer));

        ::std::result::Result::Ok(())
    } // fn write_encoded_to()
} // impl Encodable

#[test]
fn test_delete_encodable_bytes_written_matches_len() {
    let payload: Delete = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl<'a> ::ProtocolMethodPayload for Delete<'a> {
    fn class(&self) -> ::Class {
        ::Class::Exchange
    }
    fn class_id(&self) -> u16 {
        40
    }
    fn class_name(&self) -> &'static str {
        "exchange"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "delete"
    }
} // impl ::ProtocolMethodPayload for Delete<'a>
impl<'a> ::method::exchange::SetDeleteMethodFields<'a> for Delete<'a> {
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_if_unused(&mut self, if_unused: bool) {
        self.set_if_unused(if_unused)
    } // set_if_unused()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::exchange::SetDeleteMethodFields<'a> for Delete<'a>
impl<'a> From<Delete<'a>> for ClassMethod<'a> {
    fn from(from: Delete<'a>) -> Self {
        ClassMethod::Delete(from)
    } // fn from()
} // impl From<Delete<'a>> for ClassMethod

impl<'a> From<Delete<'a>> for super::SpecMethod<'a> {
    fn from(from: Delete<'a>) -> Self {
        super::SpecMethod::Exchange(from.into())
    } // fn default()
} // impl From<Delete<'a>> for ::super::SpecMethod
impl ::method::exchange::DeleteOkMethod for ::Amqp9_1 {
    type Payload = DeleteOk;
} // impl ::method::exchange::DeleteOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct DeleteOk;

impl DeleteOk {
    pub fn new() -> Self {
        DeleteOk
    } // fn new()
} // impl DeleteOk
impl Default for DeleteOk {
    fn default() -> Self {
        DeleteOk::new()
    } // fn default()
} // impl Default for DeleteOk

impl ::Encodable for DeleteOk {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_delete_ok_encodable_bytes_written_matches_len() {
    let payload: DeleteOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for DeleteOk {
    fn class(&self) -> ::Class {
        ::Class::Exchange
    }
    fn class_id(&self) -> u16 {
        40
    }
    fn class_name(&self) -> &'static str {
        "exchange"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "delete-ok"
    }
} // impl ::ProtocolMethodPayload for DeleteOk
impl<'a> From<DeleteOk> for ClassMethod<'a> {
    fn from(from: DeleteOk) -> Self {
        ClassMethod::DeleteOk(from)
    } // fn from()
} // impl From<DeleteOk> for ClassMethod

impl From<DeleteOk> for super::SpecMethod<'static> {
    fn from(from: DeleteOk) -> Self {
        super::SpecMethod::Exchange(from.into())
    } // fn default()
} // impl From<DeleteOk> for ::super::SpecMethod

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Declare(Declare<'a>),
    DeclareOk(DeclareOk),
    Delete(Delete<'a>),
    DeleteOk(DeleteOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Declare(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeclareOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Delete(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::DeleteOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        unimplemented!()
    } // fn write_encoded_to()
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class(&self) -> ::Class {
        match *self {
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Declare(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::DeclareOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Delete(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::DeleteOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
