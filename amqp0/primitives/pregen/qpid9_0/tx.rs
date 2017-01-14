// Generated by build.rs script in amqp0-primitives
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate
//
// To regenerate, and not use pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::tx::CommitMethod for ::Qpid9_0 {
    type Payload = Commit;
} // impl ::method::tx::CommitMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Commit;

impl Commit {
    pub fn new() -> Self {
        Commit
    } // fn new()
} // impl Commit
impl Default for Commit {
    fn default() -> Self {
        Commit::new()
    } // fn default()
} // impl Default for Commit

impl ::Encodable for Commit {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for Commit {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        20
    } // fn method_id()
} // impl ::Payload for Commit
impl ::method::tx::CommitOkMethod for ::Qpid9_0 {
    type Payload = CommitOk;
} // impl ::method::tx::CommitOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct CommitOk;

impl CommitOk {
    pub fn new() -> Self {
        CommitOk
    } // fn new()
} // impl CommitOk
impl Default for CommitOk {
    fn default() -> Self {
        CommitOk::new()
    } // fn default()
} // impl Default for CommitOk

impl ::Encodable for CommitOk {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for CommitOk {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        21
    } // fn method_id()
} // impl ::Payload for CommitOk
impl ::method::tx::RollbackMethod for ::Qpid9_0 {
    type Payload = Rollback;
} // impl ::method::tx::RollbackMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Rollback;

impl Rollback {
    pub fn new() -> Self {
        Rollback
    } // fn new()
} // impl Rollback
impl Default for Rollback {
    fn default() -> Self {
        Rollback::new()
    } // fn default()
} // impl Default for Rollback

impl ::Encodable for Rollback {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for Rollback {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        30
    } // fn method_id()
} // impl ::Payload for Rollback
impl ::method::tx::RollbackOkMethod for ::Qpid9_0 {
    type Payload = RollbackOk;
} // impl ::method::tx::RollbackOkMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct RollbackOk;

impl RollbackOk {
    pub fn new() -> Self {
        RollbackOk
    } // fn new()
} // impl RollbackOk
impl Default for RollbackOk {
    fn default() -> Self {
        RollbackOk::new()
    } // fn default()
} // impl Default for RollbackOk

impl ::Encodable for RollbackOk {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for RollbackOk {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        31
    } // fn method_id()
} // impl ::Payload for RollbackOk
impl ::method::tx::SelectMethod for ::Qpid9_0 {
    type Payload = Select;
} // impl ::method::tx::SelectMethod for ::Qpid9_0

// generated by primalgen::codegen::spec-module::class_mod::method_struct
#[derive(Debug)]
pub struct Select;

impl Select {
    pub fn new() -> Self {
        Select
    } // fn new()
} // impl Select
impl Default for Select {
    fn default() -> Self {
        Select::new()
    } // fn default()
} // impl Default for Select

impl ::Encodable for Select {
    fn encoded_size(&self) -> usize {
        0
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for Select {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        10
    } // fn method_id()
} // impl ::Payload for Select
impl ::method::tx::SelectOkMethod for ::Qpid9_0 {
    type Payload = SelectOk;
} // impl ::method::tx::SelectOkMethod for ::Qpid9_0

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
    } // encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

impl ::ProtocolMethodPayload for SelectOk {
    fn class_id(&self) -> u16 {
        90
    } // fn class_id()
    fn method_id(&self) -> u16 {
        11
    } // fn method_id()
} // impl ::Payload for SelectOk

#[derive(Debug)]
pub enum ClassMethod {
    Commit(Commit),
    CommitOk(CommitOk),
    Rollback(Rollback),
    RollbackOk(RollbackOk),
    Select(Select),
    SelectOk(SelectOk),
} // enum ClassMethod


impl ::Encodable for ClassMethod {
    fn encoded_size(&self) -> usize {
        match *self {
            ClassMethod::Commit(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::CommitOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Rollback(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::RollbackOk(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::Select(ref method) => ::Encodable::encoded_size(method),
            ClassMethod::SelectOk(ref method) => ::Encodable::encoded_size(method),

        } // match *self

    } // fn encoded_size
    fn write_encoded_to<W>(&self, _: &mut W) -> ::io::Result<()>
        where W: ::io::Write
    {
        unimplemented!()
    } // fn write_encoded_to()
} // impl ::Encodable for ClassMethod

impl<'a> ::ProtocolMethodPayload for ClassMethod {
    fn class_id(&self) -> u16 {
        match *self {
            ClassMethod::Commit(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::CommitOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Rollback(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::RollbackOk(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_id(method),

        } // match *self

    } // fn class_id

    fn method_id(&self) -> u16 {
        match *self {
            ClassMethod::Commit(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::CommitOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Rollback(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::RollbackOk(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_id(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_id(method),

        } // match *self

    } // fn method_id
} // impl ProtocolMethodPayload for ClassMethod
