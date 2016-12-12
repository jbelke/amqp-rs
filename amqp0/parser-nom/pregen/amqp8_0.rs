// Generated by build.rs script in amqp0-parser-nom
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-parser"
// To format and replace the pre-generated files, use: cargo --features="amqp0-build-parser"
//
// EDITORS BEWARE: Your modifications may be overridden

use nom::{IResult, be_u8, be_u16, be_u32, be_u64};

// access Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::access::Request<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
realm: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::access::Request::new(realm, flag1.0, flag1.1, flag1.2, flag1.3, flag1.4))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::access::RequestOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
(::primitives::amqp8_0::access::RequestOk::new(ticket))
)
}
}

// basic Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Qos {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
prefetch_size: be_u32 >>
prefetch_count: be_u16 >>
global: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Qos::new(prefetch_size, prefetch_count, global))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::QosOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::basic::QosOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Consume<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
consumer_tag: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::basic::Consume::new(ticket, queue, consumer_tag, flag1.0, flag1.1, flag1.2, flag1.3))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::ConsumeOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::basic::ConsumeOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Cancel<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
nowait: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Cancel::new(consumer_tag, nowait))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::CancelOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::basic::CancelOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Publish<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::basic::Publish::new(ticket, exchange, routing_key, flag1.0, flag1.1))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Return<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
(::primitives::amqp8_0::basic::Return::new(reply_code, reply_text, exchange, routing_key))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Deliver<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
delivery_tag: be_u64 >>
redelivered: bits!(call!(::common::bool_bit)) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
(::primitives::amqp8_0::basic::Deliver::new(consumer_tag, delivery_tag, redelivered, exchange, routing_key))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Get<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
no_ack: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Get::new(ticket, queue, no_ack))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::GetOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
delivery_tag: be_u64 >>
redelivered: bits!(call!(::common::bool_bit)) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
message_count: be_u32 >>
(::primitives::amqp8_0::basic::GetOk::new(delivery_tag, redelivered, exchange, routing_key, message_count))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::GetEmpty<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
cluster_id: call!(::common::shortstr) >>
(::primitives::amqp8_0::basic::GetEmpty::new(cluster_id))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Ack {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
delivery_tag: be_u64 >>
multiple: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Ack::new(delivery_tag, multiple))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Reject {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
delivery_tag: be_u64 >>
requeue: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Reject::new(delivery_tag, requeue))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::basic::Recover {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
requeue: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::basic::Recover::new(requeue))
)
}
}

// channel Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::Open<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
out_of_band: call!(::common::shortstr) >>
(::primitives::amqp8_0::channel::Open::new(out_of_band))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::OpenOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::channel::OpenOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::Flow {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
active: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::channel::Flow::new(active))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::FlowOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
active: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::channel::FlowOk::new(active))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::Alert<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
details: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
(::primitives::amqp8_0::channel::Alert::new(reply_code, reply_text, details))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::Close<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
class_id: be_u16 >>
method_id: be_u16 >>
(::primitives::amqp8_0::channel::Close::new(reply_code, reply_text, class_id, method_id))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::channel::CloseOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::channel::CloseOk::new())
)
}
}

// connection Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Start<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
version_major: be_u8 >>
version_minor: be_u8 >>
server_properties: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
mechanisms: call!(::common::longstr) >>
locales: call!(::common::longstr) >>
(::primitives::amqp8_0::connection::Start::new(version_major, version_minor, server_properties, mechanisms, locales))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::StartOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
client_properties: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
mechanism: call!(::common::shortstr) >>
response: call!(::common::longstr) >>
locale: call!(::common::shortstr) >>
(::primitives::amqp8_0::connection::StartOk::new(client_properties, mechanism, response, locale))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Secure<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
challenge: call!(::common::longstr) >>
(::primitives::amqp8_0::connection::Secure::new(challenge))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::SecureOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
response: call!(::common::longstr) >>
(::primitives::amqp8_0::connection::SecureOk::new(response))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Tune {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
channel_max: be_u16 >>
frame_max: be_u32 >>
heartbeat: be_u16 >>
(::primitives::amqp8_0::connection::Tune::new(channel_max, frame_max, heartbeat))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::TuneOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
channel_max: be_u16 >>
frame_max: be_u32 >>
heartbeat: be_u16 >>
(::primitives::amqp8_0::connection::TuneOk::new(channel_max, frame_max, heartbeat))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Open<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
virtual_host: call!(::common::shortstr) >>
capabilities: call!(::common::shortstr) >>
insist: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::connection::Open::new(virtual_host, capabilities, insist))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::OpenOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
known_hosts: call!(::common::shortstr) >>
(::primitives::amqp8_0::connection::OpenOk::new(known_hosts))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Redirect<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
host: call!(::common::shortstr) >>
known_hosts: call!(::common::shortstr) >>
(::primitives::amqp8_0::connection::Redirect::new(host, known_hosts))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::Close<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
class_id: be_u16 >>
method_id: be_u16 >>
(::primitives::amqp8_0::connection::Close::new(reply_code, reply_text, class_id, method_id))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::connection::CloseOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::connection::CloseOk::new())
)
}
}

// dtx Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::dtx::Select {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::dtx::Select::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::dtx::SelectOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::dtx::SelectOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::dtx::Start<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
dtx_identifier: call!(::common::shortstr) >>
(::primitives::amqp8_0::dtx::Start::new(dtx_identifier))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::dtx::StartOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::dtx::StartOk::new())
)
}
}

// exchange Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::exchange::Declare<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
exchange: call!(::common::shortstr) >>
ty: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
arguments: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
(::primitives::amqp8_0::exchange::Declare::new(ticket, exchange, ty, flag1.0, flag1.1, flag1.2, flag1.3, flag1.4, arguments))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::exchange::DeclareOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::exchange::DeclareOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::exchange::Delete<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
exchange: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::exchange::Delete::new(ticket, exchange, flag1.0, flag1.1))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::exchange::DeleteOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::exchange::DeleteOk::new())
)
}
}

// file Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Qos {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
prefetch_size: be_u32 >>
prefetch_count: be_u16 >>
global: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::file::Qos::new(prefetch_size, prefetch_count, global))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::QosOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::file::QosOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Consume<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
consumer_tag: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::file::Consume::new(ticket, queue, consumer_tag, flag1.0, flag1.1, flag1.2, flag1.3))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::ConsumeOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::file::ConsumeOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Cancel<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
nowait: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::file::Cancel::new(consumer_tag, nowait))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::CancelOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::file::CancelOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Open<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
identifier: call!(::common::shortstr) >>
content_size: be_u64 >>
(::primitives::amqp8_0::file::Open::new(identifier, content_size))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::OpenOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
staged_size: be_u64 >>
(::primitives::amqp8_0::file::OpenOk::new(staged_size))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Stage {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::file::Stage::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Publish<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
identifier: call!(::common::shortstr) >>
(::primitives::amqp8_0::file::Publish::new(ticket, exchange, routing_key, flag1.0, flag1.1, identifier))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Return<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
(::primitives::amqp8_0::file::Return::new(reply_code, reply_text, exchange, routing_key))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Deliver<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
delivery_tag: be_u64 >>
redelivered: bits!(call!(::common::bool_bit)) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
identifier: call!(::common::shortstr) >>
(::primitives::amqp8_0::file::Deliver::new(consumer_tag, delivery_tag, redelivered, exchange, routing_key, identifier))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Ack {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
delivery_tag: be_u64 >>
multiple: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::file::Ack::new(delivery_tag, multiple))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::file::Reject {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
delivery_tag: be_u64 >>
requeue: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::file::Reject::new(delivery_tag, requeue))
)
}
}

// queue Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::Declare<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
arguments: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
(::primitives::amqp8_0::queue::Declare::new(ticket, queue, flag1.0, flag1.1, flag1.2, flag1.3, flag1.4, arguments))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::DeclareOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
queue: call!(::common::shortstr) >>
message_count: be_u32 >>
consumer_count: be_u32 >>
(::primitives::amqp8_0::queue::DeclareOk::new(queue, message_count, consumer_count))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::Bind<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
nowait: bits!(call!(::common::bool_bit)) >>
arguments: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
(::primitives::amqp8_0::queue::Bind::new(ticket, queue, exchange, routing_key, nowait, arguments))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::BindOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::queue::BindOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::Purge<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
nowait: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::queue::Purge::new(ticket, queue, nowait))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::PurgeOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
message_count: be_u32 >>
(::primitives::amqp8_0::queue::PurgeOk::new(message_count))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::Delete<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::queue::Delete::new(ticket, queue, flag1.0, flag1.1, flag1.2))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::queue::DeleteOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
message_count: be_u32 >>
(::primitives::amqp8_0::queue::DeleteOk::new(message_count))
)
}
}

// stream Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Qos {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
prefetch_size: be_u32 >>
prefetch_count: be_u16 >>
consume_rate: be_u32 >>
global: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::stream::Qos::new(prefetch_size, prefetch_count, consume_rate, global))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::QosOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::stream::QosOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Consume<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
queue: call!(::common::shortstr) >>
consumer_tag: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::stream::Consume::new(ticket, queue, consumer_tag, flag1.0, flag1.1, flag1.2))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::ConsumeOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::stream::ConsumeOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Cancel<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
nowait: bits!(call!(::common::bool_bit)) >>
(::primitives::amqp8_0::stream::Cancel::new(consumer_tag, nowait))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::CancelOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
(::primitives::amqp8_0::stream::CancelOk::new(consumer_tag))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Publish<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
ticket: be_u16 >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
flag1: bits!(tuple!(
call!(::common::bool_bit),
call!(::common::bool_bit)
)) >>
(::primitives::amqp8_0::stream::Publish::new(ticket, exchange, routing_key, flag1.0, flag1.1))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Return<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
reply_code: be_u16 >>
reply_text: call!(::common::shortstr) >>
exchange: call!(::common::shortstr) >>
routing_key: call!(::common::shortstr) >>
(::primitives::amqp8_0::stream::Return::new(reply_code, reply_text, exchange, routing_key))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::stream::Deliver<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
consumer_tag: call!(::common::shortstr) >>
delivery_tag: be_u64 >>
exchange: call!(::common::shortstr) >>
queue: call!(::common::shortstr) >>
(::primitives::amqp8_0::stream::Deliver::new(consumer_tag, delivery_tag, exchange, queue))
)
}
}

// test Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::Integer {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
integer_1: be_u8 >>
integer_2: be_u16 >>
integer_3: be_u32 >>
integer_4: be_u64 >>
operation: be_u8 >>
(::primitives::amqp8_0::test::Integer::new(integer_1, integer_2, integer_3, integer_4, operation))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::IntegerOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
result: be_u64 >>
(::primitives::amqp8_0::test::IntegerOk::new(result))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::String<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
string_1: call!(::common::shortstr) >>
string_2: call!(::common::longstr) >>
operation: be_u8 >>
(::primitives::amqp8_0::test::String::new(string_1, string_2, operation))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::StringOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
result: call!(::common::longstr) >>
(::primitives::amqp8_0::test::StringOk::new(result))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::Table<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
table: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
integer_op: be_u8 >>
string_op: be_u8 >>
(::primitives::amqp8_0::test::Table::new(table, integer_op, string_op))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::TableOk<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
integer_result: be_u64 >>
string_result: call!(::common::longstr) >>
(::primitives::amqp8_0::test::TableOk::new(integer_result, string_result))
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::Content {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::test::Content::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::test::ContentOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
content_checksum: be_u32 >>
(::primitives::amqp8_0::test::ContentOk::new(content_checksum))
)
}
}

// tunnel Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tunnel::Request<'a> {
fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
meta_data: apply!(<::primitives::field::Table as ::NomBytes>::nom_bytes, pool) >>
(::primitives::amqp8_0::tunnel::Request::new(meta_data))
)
}
}

// tx Class
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::Select {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::Select::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::SelectOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::SelectOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::Commit {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::Commit::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::CommitOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::CommitOk::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::Rollback {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::Rollback::new())
)
}
}
impl<'a> ::NomBytes<'a> for ::primitives::amqp8_0::tx::RollbackOk {
fn nom_bytes<'b, P>(input: &'a [u8], _: &'b mut P) -> IResult<&'a [u8], Self>
where P: ::pool::ParserPool
{
do_parse!(input,
(::primitives::amqp8_0::tx::RollbackOk::new())
)
}
}
