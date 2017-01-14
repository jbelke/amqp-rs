// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::confirm::SelectMethod for ::Rabbitmq9_1 {
    type Payload = Select;
} // impl ::method::confirm::SelectMethod for ::Rabbitmq9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Select {
    no_wait: bool,
} // struct Select

impl Select {
    pub fn new(no_wait: bool) -> Self {
        Select { no_wait: no_wait } // Select
    } // fn new()
    impl_properties! {
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl Select
impl Default for Select {
    fn default() -> Self {
        Select::new(false)
    } // fn default()
} // impl Default for Select

impl ::Encodable for Select {
    fn encoded_size(&self) -> usize {
        1
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Select {
    fn class_id(&self) -> u16 {
        85
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Select
impl ::method::confirm::SetSelectMethodFields for Select {
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl ::method::confirm::SetSelectMethodFields for Select
impl ::method::confirm::SelectOkMethod for ::Rabbitmq9_1 {
    type Payload = SelectOk;
} // impl ::method::confirm::SelectOkMethod for ::Rabbitmq9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct SelectOk;

impl SelectOk {
    pub fn new() -> Self {
        SelectOk
    } // fn new()
} // impl SelectOk
impl Default for SelectOk {
    fn default() -> Self {
        SelectOk::new()
    } // fn default()
} // impl Default for SelectOk

impl ::Encodable for SelectOk {
    fn encoded_size(&self) -> usize {
        0
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for SelectOk {
    fn class_id(&self) -> u16 {
        85
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for SelectOk

#[derive(Debug)]
pub enum ClassMethod {
    Select(Select),
    SelectOk(SelectOk),
} // enum ClassMethod


impl ::Encodable for ClassMethod {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Select(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::SelectOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for ClassMethod

impl<'a> ::ProtocolMethodPayload for ClassMethod {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod