// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]


// Generated by primalgen::spec::frame_payload_enum::ClassEnumWriter
#[derive(Debug)]
pub struct Header<'a> {
    headers: Option<::field::TableEntries<'a>>,
    proxy_name: Option<::std::borrow::Cow<'a, str>>,
    data_name: Option<::std::borrow::Cow<'a, str>>,
    durable: Option<u8>,
    broadcast: Option<u8>,
} // struct Header

impl<'a> Header<'a> {
    impl_properties! {
(headers, headers_mut, set_headers, take_headers) -> Option< &::field::TableEntries<'a> >,
(proxy_name, proxy_name_mut, set_proxy_name, take_proxy_name) -> Option< Cow<str> >,
(data_name, data_name_mut, set_data_name, take_data_name) -> Option< Cow<str> >,
(durable, durable_mut, set_durable, take_durable) -> Option< u8 >,
(broadcast, broadcast_mut, set_broadcast, take_broadcast) -> Option< u8 >,
} // impl_properties
} // impl Headers

impl<'a> ::Encodable for Header<'a> {
    fn encoded_size(&self) -> usize {
        unimplemented!()
    } // fn encoded_size
} // impl ::Encodable for Header<'a>
impl<'a> ::method::tunnel::RequestMethod<'a> for ::Qpid8_0 {
    type Payload = Request<'a>;
} // impl<'a> ::method::tunnel::RequestMethod<'a> for ::Qpid8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Request<'a> {
    meta_data: ::field::TableEntries<'a>,
} // struct Request<'a>

impl<'a> Request<'a> {
    pub fn new<M>(meta_data: M) -> Self
        where M: Into<::field::TableEntries<'a>>
    {
        Request { meta_data: meta_data.into() } // Request
    } // fn new()
    impl_properties! {
(meta_data, meta_data_mut, set_meta_data) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Request<'a>
impl<'a> Default for Request<'a> {
    fn default() -> Self {
        Request::new(::field::TableEntries::new())
    } // fn default()
} // impl Default for Request

impl<'a> ::Encodable for Request<'a> {
    fn encoded_size(&self) -> usize {
        [0, ::Encodable::encoded_size(&self.meta_data)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Request<'a> {
    fn class_id(&self) -> u16 {
        110
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Request
impl<'a> ::method::tunnel::SetRequestMethodFields<'a> for Request<'a> {
    fn set_meta_data<V>(&mut self, meta_data: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_meta_data(meta_data.into())
    } // set_meta_data()
} // impl<'a> ::method::tunnel::SetRequestMethodFields<'a> for Request<'a>

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Request(Request<'a>),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Request(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Request(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Request(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
