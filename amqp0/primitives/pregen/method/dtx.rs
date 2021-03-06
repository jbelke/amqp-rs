// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.
pub trait SelectMethod {
    type Payload: Default;
} // pub trait SelectMethod

pub struct SelectBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct SelectBuilder


impl<T> SelectBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> SelectBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> SelectBuilder<T>

impl<T> Default for SelectBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        SelectBuilder { payload: Default::default() }
    }
} // impl Default for SelectBuilder
pub trait SelectOkMethod {
    type Payload: Default;
} // pub trait SelectOkMethod

pub struct SelectOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct SelectOkBuilder


impl<T> SelectOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> SelectOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> SelectOkBuilder<T>

impl<T> Default for SelectOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        SelectOkBuilder { payload: Default::default() }
    }
} // impl Default for SelectOkBuilder
pub trait StartMethod<'a> {
    type Payload: Default + SetStartMethodFields<'a>;
} // pub trait StartMethod<'a>

pub trait SetStartMethodFields<'a> {
    fn set_dtx_identifier<V>(&mut self, _: V) where V: Into<::std::borrow::Cow<'a, str>> {}
} // pub trait SetStartMethodFields<'a>

pub struct StartBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct StartBuilder


impl<T> StartBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> StartBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> StartBuilder<T>

impl<T> Default for StartBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        StartBuilder { payload: Default::default() }
    }
} // impl Default for StartBuilder
impl<'a, T> StartBuilder<T>
    where T: ::Encodable + ::Content<'a> + SetStartMethodFields<'a>
{
    pub fn dtx_identifier<V>(mut self, dtx_identifier: V) -> Self
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        SetStartMethodFields::set_dtx_identifier(&mut self.payload, dtx_identifier.into());
        self
    } // set_dtx_identifier()
} // impl<'a, T> StartBuilder<T>
pub trait StartOkMethod {
    type Payload: Default;
} // pub trait StartOkMethod

pub struct StartOkBuilder<T>
    where T: ::Encodable
{
    payload: T,
} // struct StartOkBuilder


impl<T> StartOkBuilder<T>
    where T: Default + ::Encodable
{
    pub fn new() -> Self {
        Default::default()
    }
} // impl Builder (new)

impl<T> StartOkBuilder<T>
    where T: ::Encodable
{
    pub fn build(self) -> T {
        self.payload
    }
} // impl<T> StartOkBuilder<T>

impl<T> Default for StartOkBuilder<T>
    where T: ::Encodable + Default
{
    fn default() -> Self {
        StartOkBuilder { payload: Default::default() }
    }
} // impl Default for StartOkBuilder
