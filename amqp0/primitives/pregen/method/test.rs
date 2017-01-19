// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.
pub trait ContentMethod<'a> {
    type Payload: Default;
} // pub trait ContentMethod<'a>

pub struct ContentBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct ContentBuilder


impl<T> ContentBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> ContentBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> ContentBuilder<T>

impl<T> Default for ContentBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        ContentBuilder { payload: Default::default() }
    }
} // impl Default for ContentBuilder
pub trait ContentOkMethod<'a> {
    type Payload: Default + SetContentOkMethodFields<'a>;
} // pub trait ContentOkMethod<'a>

pub trait SetContentOkMethodFields<'a> {
    fn set_content_checksum(&mut self, _: u32) {}
} // pub trait SetContentOkMethodFields<'a>

pub struct ContentOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct ContentOkBuilder


impl<T> ContentOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> ContentOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> ContentOkBuilder<T>

impl<T> Default for ContentOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        ContentOkBuilder { payload: Default::default() }
    }
} // impl Default for ContentOkBuilder
impl<'a, T> ContentOkBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetContentOkMethodFields<'a>
{
    pub fn content_checksum(mut self, content_checksum: u32) -> Self {
        SetContentOkMethodFields::set_content_checksum(&mut self.payload, content_checksum);
        self
    } // set_content_checksum()
    pub fn set_headers<V>(self, _: V) -> Self
        where V: Into<<T as ::Content<'a>>::Headers>
    {
        self
    }

    pub fn set_body<V>(self, _: V) -> Self
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        self
    }
} // impl<'a, T> ContentOkBuilder<T>
pub trait IntegerMethod {
    type Payload: Default + SetIntegerMethodFields;
} // pub trait IntegerMethod

pub trait SetIntegerMethodFields {
    fn set_integer_1(&mut self, _: u8) {}
    fn set_integer_2(&mut self, _: u16) {}
    fn set_integer_3(&mut self, _: u32) {}
    fn set_integer_4(&mut self, _: u64) {}
    fn set_operation(&mut self, _: u8) {}
} // pub trait SetIntegerMethodFields

pub struct IntegerBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct IntegerBuilder


impl<T> IntegerBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> IntegerBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> IntegerBuilder<T>

impl<T> Default for IntegerBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        IntegerBuilder { payload: Default::default() }
    }
} // impl Default for IntegerBuilder
impl<T> IntegerBuilder<T>
    where T: ::Encodable + SetIntegerMethodFields
{
    pub fn integer_1(mut self, integer_1: u8) -> Self {
        SetIntegerMethodFields::set_integer_1(&mut self.payload, integer_1);
        self
    } // set_integer_1()
    pub fn integer_2(mut self, integer_2: u16) -> Self {
        SetIntegerMethodFields::set_integer_2(&mut self.payload, integer_2);
        self
    } // set_integer_2()
    pub fn integer_3(mut self, integer_3: u32) -> Self {
        SetIntegerMethodFields::set_integer_3(&mut self.payload, integer_3);
        self
    } // set_integer_3()
    pub fn integer_4(mut self, integer_4: u64) -> Self {
        SetIntegerMethodFields::set_integer_4(&mut self.payload, integer_4);
        self
    } // set_integer_4()
    pub fn operation(mut self, operation: u8) -> Self {
        SetIntegerMethodFields::set_operation(&mut self.payload, operation);
        self
    } // set_operation()
} // impl<T> IntegerBuilder<T>
pub trait IntegerOkMethod {
    type Payload: Default + SetIntegerOkMethodFields;
} // pub trait IntegerOkMethod

pub trait SetIntegerOkMethodFields {
    fn set_result(&mut self, _: u64) {}
} // pub trait SetIntegerOkMethodFields

pub struct IntegerOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct IntegerOkBuilder


impl<T> IntegerOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> IntegerOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> IntegerOkBuilder<T>

impl<T> Default for IntegerOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        IntegerOkBuilder { payload: Default::default() }
    }
} // impl Default for IntegerOkBuilder
impl<T> IntegerOkBuilder<T>
    where T: ::Encodable + SetIntegerOkMethodFields
{
    pub fn result(mut self, result: u64) -> Self {
        SetIntegerOkMethodFields::set_result(&mut self.payload, result);
        self
    } // set_result()
} // impl<T> IntegerOkBuilder<T>
pub trait StringMethod<'a> {
    type Payload: Default + SetStringMethodFields<'a>;
} // pub trait StringMethod<'a>

pub trait SetStringMethodFields<'a> {
    fn set_operation(&mut self, _: u8) {}
    fn set_string_1<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
    fn set_string_2<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, [u8]>> {}
} // pub trait SetStringMethodFields<'a>

pub struct StringBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct StringBuilder


impl<T> StringBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> StringBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> StringBuilder<T>

impl<T> Default for StringBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        StringBuilder { payload: Default::default() }
    }
} // impl Default for StringBuilder
impl<'a, T> StringBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetStringMethodFields<'a>
{
    pub fn operation(mut self, operation: u8) -> Self {
        SetStringMethodFields::set_operation(&mut self.payload, operation);
        self
    } // set_operation()
    pub fn string_1<V>(mut self, string_1: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetStringMethodFields::set_string_1(&mut self.payload, string_1.into());
        self
    } // set_string_1()
    pub fn string_2<V>(mut self, string_2: V) -> Self
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        SetStringMethodFields::set_string_2(&mut self.payload, string_2.into());
        self
    } // set_string_2()
} // impl<'a, T> StringBuilder<T>
pub trait StringOkMethod<'a> {
    type Payload: Default + SetStringOkMethodFields<'a>;
} // pub trait StringOkMethod<'a>

pub trait SetStringOkMethodFields<'a> {
    fn set_result<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, [u8]>> {}
} // pub trait SetStringOkMethodFields<'a>

pub struct StringOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct StringOkBuilder


impl<T> StringOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> StringOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> StringOkBuilder<T>

impl<T> Default for StringOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        StringOkBuilder { payload: Default::default() }
    }
} // impl Default for StringOkBuilder
impl<'a, T> StringOkBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetStringOkMethodFields<'a>
{
    pub fn result<V>(mut self, result: V) -> Self
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        SetStringOkMethodFields::set_result(&mut self.payload, result.into());
        self
    } // set_result()
} // impl<'a, T> StringOkBuilder<T>
pub trait TableMethod<'a> {
    type Payload: Default + SetTableMethodFields<'a>;
} // pub trait TableMethod<'a>

pub trait SetTableMethodFields<'a> {
    fn set_integer_op(&mut self, _: u8) {}
    fn set_string_op(&mut self, _: u8) {}
    fn set_table<V>(&mut self, _: V) where V: Into<::field::TableEntries<'a>> {}
} // pub trait SetTableMethodFields<'a>

pub struct TableBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct TableBuilder


impl<T> TableBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> TableBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> TableBuilder<T>

impl<T> Default for TableBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        TableBuilder { payload: Default::default() }
    }
} // impl Default for TableBuilder
impl<'a, T> TableBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetTableMethodFields<'a>
{
    pub fn integer_op(mut self, integer_op: u8) -> Self {
        SetTableMethodFields::set_integer_op(&mut self.payload, integer_op);
        self
    } // set_integer_op()
    pub fn string_op(mut self, string_op: u8) -> Self {
        SetTableMethodFields::set_string_op(&mut self.payload, string_op);
        self
    } // set_string_op()
    pub fn table<V>(mut self, table: V) -> Self
        where V: Into<::field::TableEntries<'a>>
    {
        SetTableMethodFields::set_table(&mut self.payload, table.into());
        self
    } // set_table()
} // impl<'a, T> TableBuilder<T>
pub trait TableOkMethod<'a> {
    type Payload: Default + SetTableOkMethodFields<'a>;
} // pub trait TableOkMethod<'a>

pub trait SetTableOkMethodFields<'a> {
    fn set_integer_result(&mut self, _: u64) {}
    fn set_string_result<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, [u8]>> {}
} // pub trait SetTableOkMethodFields<'a>

pub struct TableOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct TableOkBuilder


impl<T> TableOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> TableOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> TableOkBuilder<T>

impl<T> Default for TableOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        TableOkBuilder { payload: Default::default() }
    }
} // impl Default for TableOkBuilder
impl<'a, T> TableOkBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetTableOkMethodFields<'a>
{
    pub fn integer_result(mut self, integer_result: u64) -> Self {
        SetTableOkMethodFields::set_integer_result(&mut self.payload, integer_result);
        self
    } // set_integer_result()
    pub fn string_result<V>(mut self, string_result: V) -> Self
        where V: Into<::std::borrow::Cow<'a, [u8]>>
    {
        SetTableOkMethodFields::set_string_result(&mut self.payload, string_result.into());
        self
    } // set_string_result()
} // impl<'a, T> TableOkBuilder<T>
