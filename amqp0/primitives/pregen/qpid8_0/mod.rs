// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden
mod access;
mod basic;
mod channel;
mod connection;
mod dtx;
mod exchange;
mod file;
mod queue;
mod stream;
mod test;
mod tunnel;
mod tx;
// Class Constants
// Generated by primalgen::codegen::spec_module::SpecModuleWriter
pub const CLASS_CONNECTION: u16 = 10;
pub const CLASS_CHANNEL: u16 = 20;
pub const CLASS_ACCESS: u16 = 30;
pub const CLASS_EXCHANGE: u16 = 40;
pub const CLASS_QUEUE: u16 = 50;
pub const CLASS_BASIC: u16 = 60;
pub const CLASS_FILE: u16 = 70;
pub const CLASS_STREAM: u16 = 80;
pub const CLASS_TX: u16 = 90;
pub const CLASS_DTX: u16 = 100;
pub const CLASS_TUNNEL: u16 = 110;
pub const CLASS_TEST: u16 = 120;

// Class Methods
// Generated by codegen::spec_module::SpecModuleWriter
pub const METHOD_ACCESS_REQUEST: u16 = 10;
pub const METHOD_ACCESS_REQUEST_OK: u16 = 11;

pub const METHOD_BASIC_QOS: u16 = 10;
pub const METHOD_BASIC_QOS_OK: u16 = 11;
pub const METHOD_BASIC_CONSUME: u16 = 20;
pub const METHOD_BASIC_CONSUME_OK: u16 = 21;
pub const METHOD_BASIC_CANCEL: u16 = 30;
pub const METHOD_BASIC_CANCEL_OK: u16 = 31;
pub const METHOD_BASIC_PUBLISH: u16 = 40;
pub const METHOD_BASIC_RETURN: u16 = 50;
pub const METHOD_BASIC_DELIVER: u16 = 60;
pub const METHOD_BASIC_GET: u16 = 70;
pub const METHOD_BASIC_GET_OK: u16 = 71;
pub const METHOD_BASIC_GET_EMPTY: u16 = 72;
pub const METHOD_BASIC_ACK: u16 = 80;
pub const METHOD_BASIC_REJECT: u16 = 90;
pub const METHOD_BASIC_RECOVER: u16 = 100;
pub const METHOD_BASIC_RECOVER_OK: u16 = 101;

pub const METHOD_CHANNEL_OPEN: u16 = 10;
pub const METHOD_CHANNEL_OPEN_OK: u16 = 11;
pub const METHOD_CHANNEL_FLOW: u16 = 20;
pub const METHOD_CHANNEL_FLOW_OK: u16 = 21;
pub const METHOD_CHANNEL_ALERT: u16 = 30;
pub const METHOD_CHANNEL_CLOSE: u16 = 40;
pub const METHOD_CHANNEL_CLOSE_OK: u16 = 41;

pub const METHOD_CONNECTION_START: u16 = 10;
pub const METHOD_CONNECTION_START_OK: u16 = 11;
pub const METHOD_CONNECTION_SECURE: u16 = 20;
pub const METHOD_CONNECTION_SECURE_OK: u16 = 21;
pub const METHOD_CONNECTION_TUNE: u16 = 30;
pub const METHOD_CONNECTION_TUNE_OK: u16 = 31;
pub const METHOD_CONNECTION_OPEN: u16 = 40;
pub const METHOD_CONNECTION_OPEN_OK: u16 = 41;
pub const METHOD_CONNECTION_REDIRECT: u16 = 50;
pub const METHOD_CONNECTION_CLOSE: u16 = 60;
pub const METHOD_CONNECTION_CLOSE_OK: u16 = 61;

pub const METHOD_DTX_SELECT: u16 = 10;
pub const METHOD_DTX_SELECT_OK: u16 = 11;
pub const METHOD_DTX_START: u16 = 20;
pub const METHOD_DTX_START_OK: u16 = 21;

pub const METHOD_EXCHANGE_DECLARE: u16 = 10;
pub const METHOD_EXCHANGE_DECLARE_OK: u16 = 11;
pub const METHOD_EXCHANGE_DELETE: u16 = 20;
pub const METHOD_EXCHANGE_DELETE_OK: u16 = 21;
pub const METHOD_EXCHANGE_BOUND: u16 = 22;
pub const METHOD_EXCHANGE_BOUND_OK: u16 = 23;

pub const METHOD_FILE_QOS: u16 = 10;
pub const METHOD_FILE_QOS_OK: u16 = 11;
pub const METHOD_FILE_CONSUME: u16 = 20;
pub const METHOD_FILE_CONSUME_OK: u16 = 21;
pub const METHOD_FILE_CANCEL: u16 = 30;
pub const METHOD_FILE_CANCEL_OK: u16 = 31;
pub const METHOD_FILE_OPEN: u16 = 40;
pub const METHOD_FILE_OPEN_OK: u16 = 41;
pub const METHOD_FILE_STAGE: u16 = 50;
pub const METHOD_FILE_PUBLISH: u16 = 60;
pub const METHOD_FILE_RETURN: u16 = 70;
pub const METHOD_FILE_DELIVER: u16 = 80;
pub const METHOD_FILE_ACK: u16 = 90;
pub const METHOD_FILE_REJECT: u16 = 100;

pub const METHOD_QUEUE_DECLARE: u16 = 10;
pub const METHOD_QUEUE_DECLARE_OK: u16 = 11;
pub const METHOD_QUEUE_BIND: u16 = 20;
pub const METHOD_QUEUE_BIND_OK: u16 = 21;
pub const METHOD_QUEUE_PURGE: u16 = 30;
pub const METHOD_QUEUE_PURGE_OK: u16 = 31;
pub const METHOD_QUEUE_DELETE: u16 = 40;
pub const METHOD_QUEUE_DELETE_OK: u16 = 41;

pub const METHOD_STREAM_QOS: u16 = 10;
pub const METHOD_STREAM_QOS_OK: u16 = 11;
pub const METHOD_STREAM_CONSUME: u16 = 20;
pub const METHOD_STREAM_CONSUME_OK: u16 = 21;
pub const METHOD_STREAM_CANCEL: u16 = 30;
pub const METHOD_STREAM_CANCEL_OK: u16 = 31;
pub const METHOD_STREAM_PUBLISH: u16 = 40;
pub const METHOD_STREAM_RETURN: u16 = 50;
pub const METHOD_STREAM_DELIVER: u16 = 60;

pub const METHOD_TEST_INTEGER: u16 = 10;
pub const METHOD_TEST_INTEGER_OK: u16 = 11;
pub const METHOD_TEST_STRING: u16 = 20;
pub const METHOD_TEST_STRING_OK: u16 = 21;
pub const METHOD_TEST_TABLE: u16 = 30;
pub const METHOD_TEST_TABLE_OK: u16 = 31;
pub const METHOD_TEST_CONTENT: u16 = 40;
pub const METHOD_TEST_CONTENT_OK: u16 = 41;

pub const METHOD_TUNNEL_REQUEST: u16 = 10;

pub const METHOD_TX_SELECT: u16 = 10;
pub const METHOD_TX_SELECT_OK: u16 = 11;
pub const METHOD_TX_COMMIT: u16 = 20;
pub const METHOD_TX_COMMIT_OK: u16 = 21;
pub const METHOD_TX_ROLLBACK: u16 = 30;
pub const METHOD_TX_ROLLBACK_OK: u16 = 31;

// Generated by primalgen::codegen::header_enum::HeaderEnumWriter
#[derive(Debug)]
pub enum SpecHeader<'a> {
    Access,
    Basic(basic::Header<'a>),
    Channel,
    Connection,
    Dtx,
    Exchange,
    File(file::Header<'a>),
    Queue,
    Stream(stream::Header<'a>),
    Test,
    Tunnel(tunnel::Header<'a>),
    Tx,
} // enum SpecHeader

impl<'a> ::Encodable for SpecHeader<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            SpecHeader::Access => 0,
            SpecHeader::Basic(ref header) => ::Encodable::encoded_size(header),
            SpecHeader::Channel => 0,
            SpecHeader::Connection => 0,
            SpecHeader::Dtx => 0,
            SpecHeader::Exchange => 0,
            SpecHeader::File(ref header) => ::Encodable::encoded_size(header),
            SpecHeader::Queue => 0,
            SpecHeader::Stream(ref header) => ::Encodable::encoded_size(header),
            SpecHeader::Test => 0,
            SpecHeader::Tunnel(ref header) => ::Encodable::encoded_size(header),
            SpecHeader::Tx => 0,

        } // match *self

    } // fn encoded_size
} // F::Encodable for SpecMethod

// Generated by primalgen::codegen::spec_module::SpecModuleWriter
#[derive(Debug)]
pub struct Frame<'a> {
    channel: u16,
    cycle: u8,
    payload: FramePayload<'a>,
} // struct Frame

impl<'a> Frame<'a> {
    pub fn new<P>(channel: u16, cycle: u8, payload: P) -> Self
        where P: Into<FramePayload<'a>>
    {
        Frame {
            channel: channel,
            cycle: cycle,
            payload: payload.into(),
        } // Frame
    } // fn new()

    pub fn channel(&self) -> u16 {
        self.channel
    } // fn channel()

    pub fn payload(&self) -> &FramePayload<'a> {
        &self.payload
    } // fn payload()

    pub fn cycle(&self) -> u8 {
        {
            self.cycle
        }
    } // fn cycle()
} // impl Frame<'a>


// Generated by primalgen::codegen::spec_module::frame_payload_enum::FramePayloadEnumWriter
#[derive(Debug)]
pub enum FramePayload<'a> {
    Body(&'a [u8]),
    Header(SpecHeader<'a>),
    Heartbeat,
    Method(SpecMethod<'a>),
    OobBody(&'a [u8]),
    OobHeader(SpecHeader<'a>),
    OobMethod(SpecMethod<'a>),
    Trace(&'a [u8]),
} // enum FramePayload

// Generated by primalgen::codegen::spec_module::frame_payload_enum::FramePayloadEnumWriter
impl<'a> ::Encodable for FramePayload<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            FramePayload::Method(ref method) |
            FramePayload::OobMethod(ref method) => ::Encodable::encoded_size(method),
            FramePayload::Header(ref header) |
            FramePayload::OobHeader(ref header) => ::Encodable::encoded_size(header),
            FramePayload::Body(ref body) |
            FramePayload::OobBody(ref body) |
            FramePayload::Trace(ref body) => body.len(),
            _ => 0,
        } // match
    } // fn encoded_size()
} // impl Encodable

// Class methods
pub use self::access::ClassMethod as AccessMethod;
pub use self::basic::ClassMethod as BasicMethod;
pub use self::channel::ClassMethod as ChannelMethod;
pub use self::connection::ClassMethod as ConnectionMethod;
pub use self::dtx::ClassMethod as DtxMethod;
pub use self::exchange::ClassMethod as ExchangeMethod;
pub use self::file::ClassMethod as FileMethod;
pub use self::queue::ClassMethod as QueueMethod;
pub use self::stream::ClassMethod as StreamMethod;
pub use self::test::ClassMethod as TestMethod;
pub use self::tunnel::ClassMethod as TunnelMethod;
pub use self::tx::ClassMethod as TxMethod;

// Class headers
pub use self::basic::Header as BasicHeader;
pub use self::file::Header as FileHeader;
pub use self::stream::Header as StreamHeader;
pub use self::tunnel::Header as TunnelHeader;
#[derive(Debug)]
pub enum SpecMethod<'a> {
    Access(AccessMethod<'a>),
    Basic(BasicMethod<'a>),
    Channel(ChannelMethod<'a>),
    Connection(ConnectionMethod<'a>),
    Dtx(DtxMethod<'a>),
    Exchange(ExchangeMethod<'a>),
    File(FileMethod<'a>),
    Queue(QueueMethod<'a>),
    Stream(StreamMethod<'a>),
    Test(TestMethod<'a>),
    Tunnel(TunnelMethod<'a>),
    Tx(TxMethod),
} // enum SpecMethod

impl<'a> ::Encodable for SpecMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            SpecMethod::Access(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Basic(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Channel(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Connection(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Dtx(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Exchange(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::File(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Queue(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Stream(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Test(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Tunnel(ref method) => ::Encodable::encoded_size(method),
            SpecMethod::Tx(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for SpecMethod<'a>

impl<'a> ::ProtocolMethodPayload for SpecMethod<'a> {
    fn class_id(&self) -> u16 {
        match *self {
            SpecMethod::Access(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Basic(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Channel(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Connection(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Dtx(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Exchange(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::File(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Queue(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Stream(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Test(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Tunnel(ref method) => ::ProtocolMethodPayload::class_id(method),
            SpecMethod::Tx(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            SpecMethod::Access(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Basic(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Channel(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Connection(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Dtx(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Exchange(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::File(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Queue(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Stream(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Test(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Tunnel(ref method) => ::ProtocolMethodPayload::method_id(method),
            SpecMethod::Tx(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for SpecMethod