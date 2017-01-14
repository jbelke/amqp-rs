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
    content_type: Option<::std::borrow::Cow<'a, str>>,
    content_encoding: Option<::std::borrow::Cow<'a, str>>,
    headers: Option<::field::TableEntries<'a>>,
    priority: Option<u8>,
    reply_to: Option<::std::borrow::Cow<'a, str>>,
    message_id: Option<::std::borrow::Cow<'a, str>>,
    filename: Option<::std::borrow::Cow<'a, str>>,
    timestamp: Option<u64>,
    cluster_id: Option<::std::borrow::Cow<'a, str>>,
} // struct Header

impl<'a> Header<'a> {
    impl_properties! {
(content_type, content_type_mut, set_content_type, take_content_type) -> Option< Cow<str> >,
(content_encoding, content_encoding_mut, set_content_encoding, take_content_encoding) -> Option< Cow<str> >,
(headers, headers_mut, set_headers, take_headers) -> Option< &::field::TableEntries<'a> >,
(priority, priority_mut, set_priority, take_priority) -> Option< u8 >,
(reply_to, reply_to_mut, set_reply_to, take_reply_to) -> Option< Cow<str> >,
(message_id, message_id_mut, set_message_id, take_message_id) -> Option< Cow<str> >,
(filename, filename_mut, set_filename, take_filename) -> Option< Cow<str> >,
(timestamp, timestamp_mut, set_timestamp, take_timestamp) -> Option< u64 >,
(cluster_id, cluster_id_mut, set_cluster_id, take_cluster_id) -> Option< Cow<str> >,
} // impl_properties
} // impl Headers

impl<'a> ::Encodable for Header<'a> {
    fn encoded_size(&self) -> usize {
        unimplemented!()
    } // fn encoded_size
} // impl ::Encodable for Header<'a>
impl ::method::file::AckMethod for ::Qpid9_0 {
    type Payload = Ack;
} // impl ::method::file::AckMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Ack {
    delivery_tag: u64,
    multiple: bool,
} // struct Ack

impl Ack {
    pub fn new(delivery_tag: u64, multiple: bool) -> Self {
        Ack {
            delivery_tag: delivery_tag,
            multiple: multiple,
        } // Ack
    } // fn new()
    impl_properties! {
(delivery_tag, set_delivery_tag) -> u64,
(multiple, set_multiple) -> bool,
} // impl_properties
} // impl Ack
impl Default for Ack {
    fn default() -> Self {
        Ack::new(0, false)
    } // fn default()
} // impl Default for Ack

impl ::Encodable for Ack {
    fn encoded_size(&self) -> usize {
        9
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Ack {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        90
    } // fn method_id()
} // impl ::Payload for Ack
impl ::method::file::SetAckMethodFields for Ack {
    fn set_delivery_tag(&mut self, delivery_tag: u64) {
        self.set_delivery_tag(delivery_tag)
    } // set_delivery_tag()
    fn set_multiple(&mut self, multiple: bool) {
        self.set_multiple(multiple)
    } // set_multiple()
} // impl ::method::file::SetAckMethodFields for Ack
impl<'a> ::method::file::CancelMethod<'a> for ::Qpid9_0 {
    type Payload = Cancel<'a>;
} // impl<'a> ::method::file::CancelMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Cancel<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
    no_wait: bool,
} // struct Cancel<'a>

impl<'a> Cancel<'a> {
    pub fn new<C>(consumer_tag: C, no_wait: bool) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        Cancel {
            consumer_tag: consumer_tag.into(),
            no_wait: no_wait,
        } // Cancel
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(no_wait, set_no_wait) -> bool,
} // impl_properties
} // impl<'a> Cancel<'a>
impl<'a> Default for Cancel<'a> {
    fn default() -> Self {
        Cancel::new("", false)
    } // fn default()
} // impl Default for Cancel

impl<'a> ::Encodable for Cancel<'a> {
    fn encoded_size(&self) -> usize {
        [2, ::Encodable::encoded_size(&self.consumer_tag)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Cancel<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        30
    } // fn method_id()
} // impl ::Payload for Cancel
impl<'a> ::method::file::SetCancelMethodFields<'a> for Cancel<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
} // impl<'a> ::method::file::SetCancelMethodFields<'a> for Cancel<'a>
impl<'a> ::method::file::CancelOkMethod<'a> for ::Qpid9_0 {
    type Payload = CancelOk<'a>;
} // impl<'a> ::method::file::CancelOkMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct CancelOk<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
} // struct CancelOk<'a>

impl<'a> CancelOk<'a> {
    pub fn new<C>(consumer_tag: C) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        CancelOk { consumer_tag: consumer_tag.into() } // CancelOk
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
} // impl_properties
} // impl<'a> CancelOk<'a>
impl<'a> Default for CancelOk<'a> {
    fn default() -> Self {
        CancelOk::new("")
    } // fn default()
} // impl Default for CancelOk

impl<'a> ::Encodable for CancelOk<'a> {
    fn encoded_size(&self) -> usize {
        [1, ::Encodable::encoded_size(&self.consumer_tag)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for CancelOk<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        31
    } // fn method_id()
} // impl ::Payload for CancelOk
impl<'a> ::method::file::SetCancelOkMethodFields<'a> for CancelOk<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
} // impl<'a> ::method::file::SetCancelOkMethodFields<'a> for CancelOk<'a>
impl<'a> ::method::file::ConsumeMethod<'a> for ::Qpid9_0 {
    type Payload = Consume<'a>;
} // impl<'a> ::method::file::ConsumeMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Consume<'a> {
    ticket: u16,
    queue: ::std::borrow::Cow<'a, str>,
    consumer_tag: ::std::borrow::Cow<'a, str>,
    no_local: bool,
    no_ack: bool,
    exclusive: bool,
    no_wait: bool,
    filter: ::field::TableEntries<'a>,
} // struct Consume<'a>

impl<'a> Consume<'a> {
    pub fn new<Q, C, F>(ticket: u16,
                        queue: Q,
                        consumer_tag: C,
                        no_local: bool,
                        no_ack: bool,
                        exclusive: bool,
                        no_wait: bool,
                        filter: F)
                        -> Self
        where Q: Into<::std::borrow::Cow<'a, str>>,
              C: Into<::std::borrow::Cow<'a, str>>,
              F: Into<::field::TableEntries<'a>>
    {
        Consume {
            ticket: ticket,
            queue: queue.into(),
            consumer_tag: consumer_tag.into(),
            no_local: no_local,
            no_ack: no_ack,
            exclusive: exclusive,
            no_wait: no_wait,
            filter: filter.into(),
        } // Consume
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(queue, queue_mut, set_queue) -> Cow<str>,
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(no_local, set_no_local) -> bool,
(no_ack, set_no_ack) -> bool,
(exclusive, set_exclusive) -> bool,
(no_wait, set_no_wait) -> bool,
(filter, filter_mut, set_filter) -> &::field::TableEntries<'a>,
} // impl_properties
} // impl<'a> Consume<'a>
impl<'a> Default for Consume<'a> {
    fn default() -> Self {
        Consume::new(0,
                     "",
                     "",
                     false,
                     false,
                     false,
                     false,
                     ::field::TableEntries::new())
    } // fn default()
} // impl Default for Consume

impl<'a> ::Encodable for Consume<'a> {
    fn encoded_size(&self) -> usize {
        [5,
         ::Encodable::encoded_size(&self.queue),
         ::Encodable::encoded_size(&self.consumer_tag),
         ::Encodable::encoded_size(&self.filter)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Consume<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Consume
impl<'a> ::method::file::SetConsumeMethodFields<'a> for Consume<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_queue<V>(&mut self, queue: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_queue(queue.into())
    } // set_queue()
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_no_local(&mut self, no_local: bool) {
        self.set_no_local(no_local)
    } // set_no_local()
    fn set_no_ack(&mut self, no_ack: bool) {
        self.set_no_ack(no_ack)
    } // set_no_ack()
    fn set_exclusive(&mut self, exclusive: bool) {
        self.set_exclusive(exclusive)
    } // set_exclusive()
    fn set_no_wait(&mut self, no_wait: bool) {
        self.set_no_wait(no_wait)
    } // set_no_wait()
    fn set_filter<V>(&mut self, filter: V)
        where V: Into<::field::TableEntries<'a>>
    {
        self.set_filter(filter.into())
    } // set_filter()
} // impl<'a> ::method::file::SetConsumeMethodFields<'a> for Consume<'a>
impl<'a> ::method::file::ConsumeOkMethod<'a> for ::Qpid9_0 {
    type Payload = ConsumeOk<'a>;
} // impl<'a> ::method::file::ConsumeOkMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct ConsumeOk<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
} // struct ConsumeOk<'a>

impl<'a> ConsumeOk<'a> {
    pub fn new<C>(consumer_tag: C) -> Self
        where C: Into<::std::borrow::Cow<'a, str>>
    {
        ConsumeOk { consumer_tag: consumer_tag.into() } // ConsumeOk
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
} // impl_properties
} // impl<'a> ConsumeOk<'a>
impl<'a> Default for ConsumeOk<'a> {
    fn default() -> Self {
        ConsumeOk::new("")
    } // fn default()
} // impl Default for ConsumeOk

impl<'a> ::Encodable for ConsumeOk<'a> {
    fn encoded_size(&self) -> usize {
        [1, ::Encodable::encoded_size(&self.consumer_tag)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for ConsumeOk<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for ConsumeOk
impl<'a> ::method::file::SetConsumeOkMethodFields<'a> for ConsumeOk<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
} // impl<'a> ::method::file::SetConsumeOkMethodFields<'a> for ConsumeOk<'a>
impl<'a> ::method::file::DeliverMethod<'a> for ::Qpid9_0 {
    type Payload = Deliver<'a>;
} // impl<'a> ::method::file::DeliverMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Deliver<'a> {
    consumer_tag: ::std::borrow::Cow<'a, str>,
    delivery_tag: u64,
    redelivered: bool,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
    identifier: ::std::borrow::Cow<'a, str>,
} // struct Deliver<'a>

impl<'a> Deliver<'a> {
    pub fn new<C, E, R, I>(consumer_tag: C,
                           delivery_tag: u64,
                           redelivered: bool,
                           exchange: E,
                           routing_key: R,
                           identifier: I)
                           -> Self
        where C: Into<::std::borrow::Cow<'a, str>>,
              E: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, str>>,
              I: Into<::std::borrow::Cow<'a, str>>
    {
        Deliver {
            consumer_tag: consumer_tag.into(),
            delivery_tag: delivery_tag,
            redelivered: redelivered,
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            identifier: identifier.into(),
        } // Deliver
    } // fn new()
    impl_properties! {
(consumer_tag, consumer_tag_mut, set_consumer_tag) -> Cow<str>,
(delivery_tag, set_delivery_tag) -> u64,
(redelivered, set_redelivered) -> bool,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
(identifier, identifier_mut, set_identifier) -> Cow<str>,
} // impl_properties
} // impl<'a> Deliver<'a>
impl<'a> Default for Deliver<'a> {
    fn default() -> Self {
        Deliver::new("", 0, false, "", "", "")
    } // fn default()
} // impl Default for Deliver

impl<'a> ::Encodable for Deliver<'a> {
    fn encoded_size(&self) -> usize {
        [13,
         ::Encodable::encoded_size(&self.consumer_tag),
         ::Encodable::encoded_size(&self.exchange),
         ::Encodable::encoded_size(&self.routing_key),
         ::Encodable::encoded_size(&self.identifier)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Deliver<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        80
    } // fn method_id()
} // impl ::Payload for Deliver
impl<'a> ::method::file::SetDeliverMethodFields<'a> for Deliver<'a> {
    fn set_consumer_tag<V>(&mut self, consumer_tag: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_consumer_tag(consumer_tag.into())
    } // set_consumer_tag()
    fn set_delivery_tag(&mut self, delivery_tag: u64) {
        self.set_delivery_tag(delivery_tag)
    } // set_delivery_tag()
    fn set_redelivered(&mut self, redelivered: bool) {
        self.set_redelivered(redelivered)
    } // set_redelivered()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_routing_key<V>(&mut self, routing_key: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_routing_key(routing_key.into())
    } // set_routing_key()
    fn set_identifier<V>(&mut self, identifier: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_identifier(identifier.into())
    } // set_identifier()
} // impl<'a> ::method::file::SetDeliverMethodFields<'a> for Deliver<'a>
impl<'a> ::method::file::OpenMethod<'a> for ::Qpid9_0 {
    type Payload = Open<'a>;
} // impl<'a> ::method::file::OpenMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Open<'a> {
    identifier: ::std::borrow::Cow<'a, str>,
    content_size: u64,
} // struct Open<'a>

impl<'a> Open<'a> {
    pub fn new<I>(identifier: I, content_size: u64) -> Self
        where I: Into<::std::borrow::Cow<'a, str>>
    {
        Open {
            identifier: identifier.into(),
            content_size: content_size,
        } // Open
    } // fn new()
    impl_properties! {
(identifier, identifier_mut, set_identifier) -> Cow<str>,
(content_size, set_content_size) -> u64,
} // impl_properties
} // impl<'a> Open<'a>
impl<'a> Default for Open<'a> {
    fn default() -> Self {
        Open::new("", 0)
    } // fn default()
} // impl Default for Open

impl<'a> ::Encodable for Open<'a> {
    fn encoded_size(&self) -> usize {
        [9, ::Encodable::encoded_size(&self.identifier)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Open<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        40
    } // fn method_id()
} // impl ::Payload for Open
impl<'a> ::method::file::SetOpenMethodFields<'a> for Open<'a> {
    fn set_identifier<V>(&mut self, identifier: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_identifier(identifier.into())
    } // set_identifier()
    fn set_content_size(&mut self, content_size: u64) {
        self.set_content_size(content_size)
    } // set_content_size()
} // impl<'a> ::method::file::SetOpenMethodFields<'a> for Open<'a>
impl ::method::file::OpenOkMethod for ::Qpid9_0 {
    type Payload = OpenOk;
} // impl ::method::file::OpenOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct OpenOk {
    staged_size: u64,
} // struct OpenOk

impl OpenOk {
    pub fn new(staged_size: u64) -> Self {
        OpenOk { staged_size: staged_size } // OpenOk
    } // fn new()
    impl_properties! {
(staged_size, set_staged_size) -> u64,
} // impl_properties
} // impl OpenOk
impl Default for OpenOk {
    fn default() -> Self {
        OpenOk::new(0)
    } // fn default()
} // impl Default for OpenOk

impl ::Encodable for OpenOk {
    fn encoded_size(&self) -> usize {
        8
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for OpenOk {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        41
    } // fn method_id()
} // impl ::Payload for OpenOk
impl ::method::file::SetOpenOkMethodFields for OpenOk {
    fn set_staged_size(&mut self, staged_size: u64) {
        self.set_staged_size(staged_size)
    } // set_staged_size()
} // impl ::method::file::SetOpenOkMethodFields for OpenOk
impl<'a> ::method::file::PublishMethod<'a> for ::Qpid9_0 {
    type Payload = Publish<'a>;
} // impl<'a> ::method::file::PublishMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Publish<'a> {
    ticket: u16,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
    mandatory: bool,
    immediate: bool,
    identifier: ::std::borrow::Cow<'a, str>,
} // struct Publish<'a>

impl<'a> Publish<'a> {
    pub fn new<E, R, I>(ticket: u16,
                        exchange: E,
                        routing_key: R,
                        mandatory: bool,
                        immediate: bool,
                        identifier: I)
                        -> Self
        where E: Into<::std::borrow::Cow<'a, str>>,
              R: Into<::std::borrow::Cow<'a, str>>,
              I: Into<::std::borrow::Cow<'a, str>>
    {
        Publish {
            ticket: ticket,
            exchange: exchange.into(),
            routing_key: routing_key.into(),
            mandatory: mandatory,
            immediate: immediate,
            identifier: identifier.into(),
        } // Publish
    } // fn new()
    impl_properties! {
(ticket, set_ticket) -> u16,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
(mandatory, set_mandatory) -> bool,
(immediate, set_immediate) -> bool,
(identifier, identifier_mut, set_identifier) -> Cow<str>,
} // impl_properties
} // impl<'a> Publish<'a>
impl<'a> Default for Publish<'a> {
    fn default() -> Self {
        Publish::new(0, "", "", false, false, "")
    } // fn default()
} // impl Default for Publish

impl<'a> ::Encodable for Publish<'a> {
    fn encoded_size(&self) -> usize {
        [6,
         ::Encodable::encoded_size(&self.exchange),
         ::Encodable::encoded_size(&self.routing_key),
         ::Encodable::encoded_size(&self.identifier)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Publish<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        60
    } // fn method_id()
} // impl ::Payload for Publish
impl<'a> ::method::file::SetPublishMethodFields<'a> for Publish<'a> {
    fn set_ticket(&mut self, ticket: u16) {
        self.set_ticket(ticket)
    } // set_ticket()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_routing_key<V>(&mut self, routing_key: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_routing_key(routing_key.into())
    } // set_routing_key()
    fn set_mandatory(&mut self, mandatory: bool) {
        self.set_mandatory(mandatory)
    } // set_mandatory()
    fn set_immediate(&mut self, immediate: bool) {
        self.set_immediate(immediate)
    } // set_immediate()
    fn set_identifier<V>(&mut self, identifier: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_identifier(identifier.into())
    } // set_identifier()
} // impl<'a> ::method::file::SetPublishMethodFields<'a> for Publish<'a>
impl ::method::file::QosMethod for ::Qpid9_0 {
    type Payload = Qos;
} // impl ::method::file::QosMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Qos {
    prefetch_size: u32,
    prefetch_count: u16,
    global: bool,
} // struct Qos

impl Qos {
    pub fn new(prefetch_size: u32, prefetch_count: u16, global: bool) -> Self {
        Qos {
            prefetch_size: prefetch_size,
            prefetch_count: prefetch_count,
            global: global,
        } // Qos
    } // fn new()
    impl_properties! {
(prefetch_size, set_prefetch_size) -> u32,
(prefetch_count, set_prefetch_count) -> u16,
(global, set_global) -> bool,
} // impl_properties
} // impl Qos
impl Default for Qos {
    fn default() -> Self {
        Qos::new(0, 0, false)
    } // fn default()
} // impl Default for Qos

impl ::Encodable for Qos {
    fn encoded_size(&self) -> usize {
        7
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Qos {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Qos
impl ::method::file::SetQosMethodFields for Qos {
    fn set_prefetch_size(&mut self, prefetch_size: u32) {
        self.set_prefetch_size(prefetch_size)
    } // set_prefetch_size()
    fn set_prefetch_count(&mut self, prefetch_count: u16) {
        self.set_prefetch_count(prefetch_count)
    } // set_prefetch_count()
    fn set_global(&mut self, global: bool) {
        self.set_global(global)
    } // set_global()
} // impl ::method::file::SetQosMethodFields for Qos
impl ::method::file::QosOkMethod for ::Qpid9_0 {
    type Payload = QosOk;
} // impl ::method::file::QosOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct QosOk;

impl QosOk {
    pub fn new() -> Self {
        QosOk
    } // fn new()
} // impl QosOk
impl Default for QosOk {
    fn default() -> Self {
        QosOk::new()
    } // fn default()
} // impl Default for QosOk

impl ::Encodable for QosOk {
    fn encoded_size(&self) -> usize {
        0
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for QosOk {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for QosOk
impl ::method::file::RejectMethod for ::Qpid9_0 {
    type Payload = Reject;
} // impl ::method::file::RejectMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Reject {
    delivery_tag: u64,
    requeue: bool,
} // struct Reject

impl Reject {
    pub fn new(delivery_tag: u64, requeue: bool) -> Self {
        Reject {
            delivery_tag: delivery_tag,
            requeue: requeue,
        } // Reject
    } // fn new()
    impl_properties! {
(delivery_tag, set_delivery_tag) -> u64,
(requeue, set_requeue) -> bool,
} // impl_properties
} // impl Reject
impl Default for Reject {
    fn default() -> Self {
        Reject::new(0, false)
    } // fn default()
} // impl Default for Reject

impl ::Encodable for Reject {
    fn encoded_size(&self) -> usize {
        9
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Reject {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        100
    } // fn method_id()
} // impl ::Payload for Reject
impl ::method::file::SetRejectMethodFields for Reject {
    fn set_delivery_tag(&mut self, delivery_tag: u64) {
        self.set_delivery_tag(delivery_tag)
    } // set_delivery_tag()
    fn set_requeue(&mut self, requeue: bool) {
        self.set_requeue(requeue)
    } // set_requeue()
} // impl ::method::file::SetRejectMethodFields for Reject
impl<'a> ::method::file::ReturnMethod<'a> for ::Qpid9_0 {
    type Payload = Return<'a>;
} // impl<'a> ::method::file::ReturnMethod<'a> for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Return<'a> {
    reply_code: u16,
    reply_text: ::std::borrow::Cow<'a, str>,
    exchange: ::std::borrow::Cow<'a, str>,
    routing_key: ::std::borrow::Cow<'a, str>,
} // struct Return<'a>

impl<'a> Return<'a> {
    pub fn new<R, E, R0>(reply_code: u16, reply_text: R, exchange: E, routing_key: R0) -> Self
        where R: Into<::std::borrow::Cow<'a, str>>,
              E: Into<::std::borrow::Cow<'a, str>>,
              R0: Into<::std::borrow::Cow<'a, str>>
    {
        Return {
            reply_code: reply_code,
            reply_text: reply_text.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
        } // Return
    } // fn new()
    impl_properties! {
(reply_code, set_reply_code) -> u16,
(reply_text, reply_text_mut, set_reply_text) -> Cow<str>,
(exchange, exchange_mut, set_exchange) -> Cow<str>,
(routing_key, routing_key_mut, set_routing_key) -> Cow<str>,
} // impl_properties
} // impl<'a> Return<'a>
impl<'a> Default for Return<'a> {
    fn default() -> Self {
        Return::new(0, "", "", "")
    } // fn default()
} // impl Default for Return

impl<'a> ::Encodable for Return<'a> {
    fn encoded_size(&self) -> usize {
        [5,
         ::Encodable::encoded_size(&self.reply_text),
         ::Encodable::encoded_size(&self.exchange),
         ::Encodable::encoded_size(&self.routing_key)]
            .iter()
            .sum()
    } // fn encoded_size()
} // impl Encodable

impl<'a> ::ProtocolMethodPayload for Return<'a> {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        70
    } // fn method_id()
} // impl ::Payload for Return
impl<'a> ::method::file::SetReturnMethodFields<'a> for Return<'a> {
    fn set_reply_code(&mut self, reply_code: u16) {
        self.set_reply_code(reply_code)
    } // set_reply_code()
    fn set_reply_text<V>(&mut self, reply_text: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_reply_text(reply_text.into())
    } // set_reply_text()
    fn set_exchange<V>(&mut self, exchange: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_exchange(exchange.into())
    } // set_exchange()
    fn set_routing_key<V>(&mut self, routing_key: V)
        where V: Into<::std::borrow::Cow<'a, str>>
    {
        self.set_routing_key(routing_key.into())
    } // set_routing_key()
} // impl<'a> ::method::file::SetReturnMethodFields<'a> for Return<'a>
impl ::method::file::StageMethod for ::Qpid9_0 {
    type Payload = Stage;
} // impl ::method::file::StageMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Stage;

impl Stage {
    pub fn new() -> Self {
        Stage
    } // fn new()
} // impl Stage
impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    } // fn default()
} // impl Default for Stage

impl ::Encodable for Stage {
    fn encoded_size(&self) -> usize {
        0
    } // fn encoded_size()
} // impl Encodable

impl ::ProtocolMethodPayload for Stage {
    fn class_id(&self) -> u16 {
        70
    } // fn class_id()
    fn method_id(&self) -> u16 {
        50
    } // fn method_id()
} // impl ::Payload for Stage

#[derive(Debug)]
pub enum ClassMethod<'a> {
    Ack(Ack),
    Cancel(Cancel<'a>),
    CancelOk(CancelOk<'a>),
    Consume(Consume<'a>),
    ConsumeOk(ConsumeOk<'a>),
    Deliver(Deliver<'a>),
    Open(Open<'a>),
    OpenOk(OpenOk),
    Publish(Publish<'a>),
    Qos(Qos),
    QosOk(QosOk),
    Reject(Reject),
    Return(Return<'a>),
    Stage(Stage),
} // enum ClassMethod


impl<'a> ::Encodable for ClassMethod<'a> {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Ack(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Cancel(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CancelOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Consume(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::ConsumeOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Deliver(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Open(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::OpenOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Publish(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Qos(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::QosOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Reject(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Return(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Stage(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
} // impl ::Encodable for ClassMethod<'a>

impl<'a> ::ProtocolMethodPayload for ClassMethod<'a> {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Ack(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Cancel(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CancelOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Consume(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::ConsumeOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Deliver(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Publish(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Qos(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::QosOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Reject(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Return(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Stage(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Ack(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Cancel(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CancelOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Consume(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::ConsumeOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Deliver(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Open(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::OpenOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Publish(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Qos(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::QosOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Reject(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Return(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Stage(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod