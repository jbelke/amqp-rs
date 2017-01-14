// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::test::ContentMethod for ::Amqp8_0 {
    type Payload = Content;
} // impl ::method::test::ContentMethod for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Content;

impl Content {
    pub fn new() -> Self {
        Content
    } // fn new()
} // impl Content
impl Default for Content {
    fn default() -> Self {
        Content::new()
    } // fn default()
} // impl Default for Content

impl ::Encodable for Content {
    fn encoded_size(&self) -> usize {
        0
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Content {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        40
    } // fn method_id()
} // impl ::Payload for Content
impl ::method::test::ContentOkMethod for ::Amqp8_0 {
    type Payload = ContentOk;
} // impl ::method::test::ContentOkMethod for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct ContentOk {
    content_checksum: u32,
} // struct ContentOk

impl ContentOk {
    pub fn new(content_checksum: u32) -> Self {
        ContentOk { content_checksum: content_checksum } // ContentOk
    } // fn new()
    impl_properties! {
(content_checksum, set_content_checksum) -> u32,
} // impl_properties
} // impl ContentOk
impl Default for ContentOk {
    fn default() -> Self {
        ContentOk::new(0)
    } // fn default()
} // impl Default for ContentOk

impl ::Encodable for ContentOk {
    fn encoded_size(&self) -> usize {
        4
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for ContentOk {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        41
    } // fn method_id()
} // impl ::Payload for ContentOk
impl ::method::test::SetContentOkMethodFields for ContentOk {
    fn set_content_checksum(&mut self, content_checksum: u32) {
        self.set_content_checksum(content_checksum)
    } // set_content_checksum()
} // impl ::method::test::SetContentOkMethodFields for ContentOk
impl ::method::test::IntegerMethod for ::Amqp8_0 {
    type Payload = Integer;
} // impl ::method::test::IntegerMethod for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Integer {
    integer_1: u8,
    integer_2: u16,
    integer_3: u32,
    integer_4: u64,
    operation: u8,
} // struct Integer

impl Integer {
    pub fn new(integer_1: u8,
               integer_2: u16,
               integer_3: u32,
               integer_4: u64,
               operation: u8)
               -> Self {
        Integer {
            integer_1: integer_1,
            integer_2: integer_2,
            integer_3: integer_3,
            integer_4: integer_4,
            operation: operation,
        } // Integer
    } // fn new()
    impl_properties! {
(integer_1, set_integer_1) -> u8,
(integer_2, set_integer_2) -> u16,
(integer_3, set_integer_3) -> u32,
(integer_4, set_integer_4) -> u64,
(operation, set_operation) -> u8,
} // impl_properties
} // impl Integer
impl Default for Integer {
    fn default() -> Self {
        Integer::new(0, 0, 0, 0, 0)
    } // fn default()
} // impl Default for Integer

impl ::Encodable for Integer {
    fn encoded_size(&self) -> usize {
        16
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Integer {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Integer
impl ::method::test::SetIntegerMethodFields for Integer {
    fn set_integer_1(&mut self, integer_1: u8) {
        self.set_integer_1(integer_1)
    } // set_integer_1()
    fn set_integer_2(&mut self, integer_2: u16) {
        self.set_integer_2(integer_2)
    } // set_integer_2()
    fn set_integer_3(&mut self, integer_3: u32) {
        self.set_integer_3(integer_3)
    } // set_integer_3()
    fn set_integer_4(&mut self, integer_4: u64) {
        self.set_integer_4(integer_4)
    } // set_integer_4()
    fn set_operation(&mut self, operation: u8) {
        self.set_operation(operation)
    } // set_operation()
} // impl ::method::test::SetIntegerMethodFields for Integer
impl ::method::test::IntegerOkMethod for ::Amqp8_0 {
    type Payload = IntegerOk;
} // impl ::method::test::IntegerOkMethod for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct IntegerOk {
    result: u64,
} // struct IntegerOk

impl IntegerOk {
    pub fn new(result: u64) -> Self {
        IntegerOk { result: result } // IntegerOk
    } // fn new()
    impl_properties! {
(result, set_result) -> u64,
} // impl_properties
} // impl IntegerOk
impl Default for IntegerOk {
    fn default() -> Self {
        IntegerOk::new(0)
    } // fn default()
} // impl Default for IntegerOk

impl ::Encodable for IntegerOk {
    fn encoded_size(&self) -> usize {
        8
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for IntegerOk {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for IntegerOk
impl ::method::test::SetIntegerOkMethodFields for IntegerOk {
    fn set_result(&mut self, result: u64) {
        self.set_result(result)
    } // set_result()
} // impl ::method::test::SetIntegerOkMethodFields for IntegerOk
impl<'a> ::method::test::StringMethod<'a> for ::Amqp8_0 {
    type Payload = String<'a>;
} // impl<'a> ::method::test::StringMethod<'a> for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct String<'a> {
    string_1: ::std::borrow::Cow<'a, str>,
    string_2: ::std::borrow::Cow<'a, [u8]>,
    operation: u8,
} // struct String<'a>

impl<'a> String<'a> {
    pub fn new<S, S0>(string_1: S, string_2: S0, operation: u8) -> Self
        where S: Into<::std::borrow::Cow<'a, str>>,
              S0: Into<::std::borrow::Cow<'a, [u8]>>
    {
        String {
            string_1: string_1.into(),
            string_2: string_2.into(),
            operation: operation,
        } // String
    } // fn new()
    impl_properties! {
(string_1, string_1_mut, set_string_1) -> Cow<str>,
(string_2, string_2_mut, set_string_2) -> Cow<[u8]>,
(operation, set_operation) -> u8,
} // impl_properties
} // impl<'a> String<'a>
impl<'a> Default for String<'a> {
    fn default() -> Self {
        String::new("", &[][..], 0)
    } // fn default()
} // impl Default for String

impl<'a> ::Encodable for String<'a> {
    fn encoded_size(&self) -> usize {
        [4, ::Encodable::encoded_size(&self.string_1), ::Encodable::encoded_size(&self.string_2)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for String<'a> {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for String
impl<'a> ::method::test::SetStringMethodFields<'a> for String<'a> {
    fn set_string_1<V>(&mut self, string_1: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_string_1(string_1.into())
    } // set_string_1()
    fn set_string_2<V>(&mut self, string_2: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_string_2(string_2.into())
    } // set_string_2()
    fn set_operation(&mut self, operation: u8) {
        self.set_operation(operation)
    } // set_operation()
} // impl<'a> ::method::test::SetStringMethodFields<'a> for String<'a>
impl<'a> ::method::test::StringOkMethod<'a> for ::Amqp8_0 {
    type Payload = StringOk<'a>;
} // impl<'a> ::method::test::StringOkMethod<'a> for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct StringOk<'a> {
    result: ::std::borrow::Cow<'a, [u8]>,
} // struct StringOk<'a>

impl<'a> StringOk<'a> {
    pub fn new<R>(result: R) -> Self
        where R: Into<::std::borrow::Cow<'a, [u8]>>
    {
        StringOk { result: result.into() } // StringOk
    } // fn new()
    impl_properties! {
(result, result_mut, set_result) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> StringOk<'a>
impl<'a> Default for StringOk<'a> {
    fn default() -> Self {
        StringOk::new(&[][..])
    } // fn default()
} // impl Default for StringOk

impl<'a> ::Encodable for StringOk<'a> {
    fn encoded_size(&self) -> usize {
        [2, ::Encodable::encoded_size(&self.result)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for StringOk<'a> {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for StringOk
impl<'a> ::method::test::SetStringOkMethodFields<'a> for StringOk<'a> {
    fn set_result<V>(&mut self, result: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_result(result.into())
    } // set_result()
} // impl<'a> ::method::test::SetStringOkMethodFields<'a> for StringOk<'a>
impl<'a> ::method::test::TableMethod<'a> for ::Amqp8_0 {
    type Payload = Table<'a>;
} // impl<'a> ::method::test::TableMethod<'a> for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Table<'a> {
    table: ::field::TableEntries<'a>,
    integer_op: u8,
    string_op: u8,
} // struct Table<'a>

impl<'a> Table<'a> {
    pub fn new<T>(table: T, integer_op: u8, string_op: u8) -> Self
        where T: Into<::field::TableEntries<'a>>
    {
        Table {
            table: table.into(),
            integer_op: integer_op,
            string_op: string_op,
        } // Table
    } // fn new()
    impl_properties! {
(table, table_mut, set_table) -> &::field::TableEntries<'a>,
(integer_op, set_integer_op) -> u8,
(string_op, set_string_op) -> u8,
} // impl_properties
} // impl<'a> Table<'a>
impl<'a> Default for Table<'a> {
    fn default() -> Self {
        Table::new(::field::TableEntries::new(), 0, 0)
    } // fn default()
} // impl Default for Table

impl<'a> ::Encodable for Table<'a> {
    fn encoded_size(&self) -> usize {
        [2, ::Encodable::encoded_size(&self.table)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Table<'a> {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        30
    } // fn method_id()
} // impl ::Payload for Table
impl<'a> ::method::test::SetTableMethodFields<'a> for Table<'a> {
    fn set_table<V>(&mut self, table: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_table(table.into())
    } // set_table()
    fn set_integer_op(&mut self, integer_op: u8) {
        self.set_integer_op(integer_op)
    } // set_integer_op()
    fn set_string_op(&mut self, string_op: u8) {
        self.set_string_op(string_op)
    } // set_string_op()
} // impl<'a> ::method::test::SetTableMethodFields<'a> for Table<'a>
impl<'a> ::method::test::TableOkMethod<'a> for ::Amqp8_0 {
    type Payload = TableOk<'a>;
} // impl<'a> ::method::test::TableOkMethod<'a> for ::Amqp8_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct TableOk<'a> {
    integer_result: u64,
    string_result: ::std::borrow::Cow<'a, [u8]>,
} // struct TableOk<'a>

impl<'a> TableOk<'a> {
    pub fn new<S>(integer_result: u64, string_result: S) -> Self
        where S: Into<::std::borrow::Cow<'a, [u8]>>
    {
        TableOk {
            integer_result: integer_result,
            string_result: string_result.into(),
        } // TableOk
    } // fn new()
    impl_properties! {
(integer_result, set_integer_result) -> u64,
(string_result, string_result_mut, set_string_result) -> Cow<[u8]>,
} // impl_properties
} // impl<'a> TableOk<'a>
impl<'a> Default for TableOk<'a> {
    fn default() -> Self {
        TableOk::new(0, &[][..])
    } // fn default()
} // impl Default for TableOk

impl<'a> ::Encodable for TableOk<'a> {
    fn encoded_size(&self) -> usize {
        [10, ::Encodable::encoded_size(&self.string_result)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for TableOk<'a> {
    fn class_id(&self) -> u16 {
        120
    } // fn class_id()
    fn method_id(&self) -> u16 {
        31
    } // fn method_id()
} // impl ::Payload for TableOk
impl<'a> ::method::test::SetTableOkMethodFields<'a> for TableOk<'a> {
    fn set_integer_result(&mut self, integer_result: u64) {
        self.set_integer_result(integer_result)
    } // set_integer_result()
    fn set_string_result<V>(&mut self, string_result: V)
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self.set_string_result(string_result.into())
    } // set_string_result()
} // impl<'a> ::method::test::SetTableOkMethodFields<'a> for TableOk<'a>

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Content(Content),
    ContentOk(ContentOk),
    Integer(Integer),
    IntegerOk(IntegerOk),
    String(String<'a>),
    StringOk(StringOk<'a>),
    Table(Table<'a>),
    TableOk(TableOk<'a>),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Content(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::ContentOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Integer(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::IntegerOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::String(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::StringOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Table(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::TableOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Content(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::ContentOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Integer(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::IntegerOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::String(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::StringOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Table(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::TableOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
