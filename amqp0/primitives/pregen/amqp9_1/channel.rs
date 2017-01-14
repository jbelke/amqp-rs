// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl<'a> ::method::channel::CloseMethod<'a> for ::Amqp9_1 {
    type Payload = Close<'a>;
} // impl<'a> ::method::channel::CloseMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Close<'a> {
    reply_code: u16,
    reply_text: ::std::borrow::Cow<'a, str>,
    class_id: u16,
    method_id: u16,
} // struct Close<'a>

impl<'a> Close<'a> {
    pub fn new<R>(reply_code: u16, reply_text: R, class_id: u16, method_id: u16) -> Self
        where R: Into<::std::borrow::Cow<'a, str>>
    {
        Close {
            reply_code: reply_code,
            reply_text: reply_text.into(),
            class_id: class_id,
            method_id: method_id,
        } // Close
    } // fn new()
    impl_properties! {
(reply_code, set_reply_code) -> u16,
(reply_text, reply_text_mut, set_reply_text) -> Cow<str>,
(class_id, set_class_id) -> u16,
(method_id, set_method_id) -> u16,
} // impl_properties
} // impl<'a> Close<'a>
impl<'a> Default for Close<'a> {
    fn default() -> Self {
        Close::new(0, "", 0, 0)
    } // fn default()
} // impl Default for Close

impl<'a> ::Encodable for Close<'a> {
    fn encoded_size(&self) -> usize {
        [7, ::Encodable::encoded_size(&self.reply_text)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Close<'a> {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        40
    } // fn method_id()
} // impl ::Payload for Close
impl<'a> ::method::channel::SetCloseMethodFields<'a> for Close<'a> {
    fn set_reply_code(&mut self, reply_code: u16) {
        self.set_reply_code(reply_code)
    } // set_reply_code()
    fn set_reply_text<V>(&mut self, reply_text: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_reply_text(reply_text.into())
    } // set_reply_text()
    fn set_class_id(&mut self, class_id: u16) {
        self.set_class_id(class_id)
    } // set_class_id()
    fn set_method_id(&mut self, method_id: u16) {
        self.set_method_id(method_id)
    } // set_method_id()
} // impl<'a> ::method::channel::SetCloseMethodFields<'a> for Close<'a>
impl ::method::channel::CloseOkMethod for ::Amqp9_1 {
    type Payload = CloseOk;
} // impl ::method::channel::CloseOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct CloseOk;

impl CloseOk {
    pub fn new() -> Self {
        CloseOk
    } // fn new()
} // impl CloseOk
impl Default for CloseOk {
    fn default() -> Self {
        CloseOk::new()
    } // fn default()
} // impl Default for CloseOk

impl ::Encodable for CloseOk {
    fn encoded_size(&self) -> usize {
        0
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for CloseOk {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        41
    } // fn method_id()
} // impl ::Payload for CloseOk
impl ::method::channel::FlowMethod for ::Amqp9_1 {
    type Payload = Flow;
} // impl ::method::channel::FlowMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Flow {
    active: bool,
} // struct Flow

impl Flow {
    pub fn new(active: bool) -> Self {
        Flow { active: active } // Flow
    } // fn new()
    impl_properties! {
(active, set_active) -> bool,
} // impl_properties
} // impl Flow
impl Default for Flow {
    fn default() -> Self {
        Flow::new(false)
    } // fn default()
} // impl Default for Flow

impl ::Encodable for Flow {
    fn encoded_size(&self) -> usize {
        1
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Flow {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Flow
impl ::method::channel::SetFlowMethodFields for Flow {
    fn set_active(&mut self, active: bool) {
        self.set_active(active)
    } // set_active()
} // impl ::method::channel::SetFlowMethodFields for Flow
impl ::method::channel::FlowOkMethod for ::Amqp9_1 {
    type Payload = FlowOk;
} // impl ::method::channel::FlowOkMethod for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct FlowOk {
    active: bool,
} // struct FlowOk

impl FlowOk {
    pub fn new(active: bool) -> Self {
        FlowOk { active: active } // FlowOk
    } // fn new()
    impl_properties! {
(active, set_active) -> bool,
} // impl_properties
} // impl FlowOk
impl Default for FlowOk {
    fn default() -> Self {
        FlowOk::new(false)
    } // fn default()
} // impl Default for FlowOk

impl ::Encodable for FlowOk {
    fn encoded_size(&self) -> usize {
        1
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for FlowOk {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for FlowOk
impl ::method::channel::SetFlowOkMethodFields for FlowOk {
    fn set_active(&mut self, active: bool) {
        self.set_active(active)
    } // set_active()
} // impl ::method::channel::SetFlowOkMethodFields for FlowOk
impl<'a> ::method::channel::OpenMethod<'a> for ::Amqp9_1 {
    type Payload = Open;
} // impl<'a> ::method::channel::OpenMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Open;

impl Open {
    pub fn new() -> Self {
        Open
    } // fn new()
} // impl Open
impl Default for Open {
    fn default() -> Self {
        Open::new()
    } // fn default()
} // impl Default for Open

impl ::Encodable for Open {
    fn encoded_size(&self) -> usize {
        [1].iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Open {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Open
impl<'a> ::method::channel::SetOpenMethodFields<'a> for Open {} // impl<'a> ::method::channel::SetOpenMethodFields<'a> for Open
impl<'a> ::method::channel::OpenOkMethod<'a> for ::Amqp9_1 {
    type Payload = OpenOk;
} // impl<'a> ::method::channel::OpenOkMethod<'a> for ::Amqp9_1

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct OpenOk;

impl OpenOk {
    pub fn new() -> Self {
        OpenOk
    } // fn new()
} // impl OpenOk
impl Default for OpenOk {
    fn default() -> Self {
        OpenOk::new()
    } // fn default()
} // impl Default for OpenOk

impl ::Encodable for OpenOk {
    fn encoded_size(&self) -> usize {
        [2].iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for OpenOk {
    fn class_id(&self) -> u16 {
        20
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for OpenOk
impl<'a> ::method::channel::SetOpenOkMethodFields<'a> for OpenOk {} // impl<'a> ::method::channel::SetOpenOkMethodFields<'a> for OpenOk

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Close(Close<'a>),
    CloseOk(CloseOk),
    Flow(Flow),
    FlowOk(FlowOk),
    Open(Open),
    OpenOk(OpenOk),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Close(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CloseOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Flow(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::FlowOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Open(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::OpenOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Close(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CloseOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Flow(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::FlowOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
