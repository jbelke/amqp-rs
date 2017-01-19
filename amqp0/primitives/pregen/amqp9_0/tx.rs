// Generated by build.rs script in the amqp0-primitives crate.
// Pre-generated files are used by default. Generation is done with the amqp0-codegen crate.
//
// To regenerate, ignoring the pre-generated files, use: cargo --features="amqp0-build-primitives"
// To format and replace the pre-generated files, use: cargo --features="amqp0-pregen-primitives"
//
// EDITORS BEWARE: Your modifications may be overridden or removed.

// generated by primalgen::codegen::spec_module::class_mod::ClassModuleWriter
#![allow(too_many_arguments)]

impl ::method::tx::CommitMethod for ::Amqp9_0 {
    type Payload = Commit;
} // impl ::method::tx::CommitMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_commit_encodable_bytes_written_matches_len() {
    let payload: Commit = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for Commit {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        20
    }
    fn method_name(&self) -> &'static str {
        "commit"
    }
} // impl ::ProtocolMethodPayload for Commit
impl From<Commit> for ClassMethod {
    fn from(from: Commit) -> Self {
        ClassMethod::Commit(from)
    } // fn from()
} // impl From<Commit> for ClassMethod

impl From<Commit> for super::SpecMethod<'static> {
    fn from(from: Commit) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<Commit> for ::super::SpecMethod
impl ::method::tx::CommitOkMethod for ::Amqp9_0 {
    type Payload = CommitOk;
} // impl ::method::tx::CommitOkMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_commit_ok_encodable_bytes_written_matches_len() {
    let payload: CommitOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for CommitOk {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        21
    }
    fn method_name(&self) -> &'static str {
        "commit-ok"
    }
} // impl ::ProtocolMethodPayload for CommitOk
impl From<CommitOk> for ClassMethod {
    fn from(from: CommitOk) -> Self {
        ClassMethod::CommitOk(from)
    } // fn from()
} // impl From<CommitOk> for ClassMethod

impl From<CommitOk> for super::SpecMethod<'static> {
    fn from(from: CommitOk) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<CommitOk> for ::super::SpecMethod
impl ::method::tx::RollbackMethod for ::Amqp9_0 {
    type Payload = Rollback;
} // impl ::method::tx::RollbackMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_rollback_encodable_bytes_written_matches_len() {
    let payload: Rollback = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for Rollback {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        30
    }
    fn method_name(&self) -> &'static str {
        "rollback"
    }
} // impl ::ProtocolMethodPayload for Rollback
impl From<Rollback> for ClassMethod {
    fn from(from: Rollback) -> Self {
        ClassMethod::Rollback(from)
    } // fn from()
} // impl From<Rollback> for ClassMethod

impl From<Rollback> for super::SpecMethod<'static> {
    fn from(from: Rollback) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<Rollback> for ::super::SpecMethod
impl ::method::tx::RollbackOkMethod for ::Amqp9_0 {
    type Payload = RollbackOk;
} // impl ::method::tx::RollbackOkMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_rollback_ok_encodable_bytes_written_matches_len() {
    let payload: RollbackOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for RollbackOk {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        31
    }
    fn method_name(&self) -> &'static str {
        "rollback-ok"
    }
} // impl ::ProtocolMethodPayload for RollbackOk
impl From<RollbackOk> for ClassMethod {
    fn from(from: RollbackOk) -> Self {
        ClassMethod::RollbackOk(from)
    } // fn from()
} // impl From<RollbackOk> for ClassMethod

impl From<RollbackOk> for super::SpecMethod<'static> {
    fn from(from: RollbackOk) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<RollbackOk> for ::super::SpecMethod
impl ::method::tx::SelectMethod for ::Amqp9_0 {
    type Payload = Select;
} // impl ::method::tx::SelectMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_select_encodable_bytes_written_matches_len() {
    let payload: Select = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for Select {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        10
    }
    fn method_name(&self) -> &'static str {
        "select"
    }
} // impl ::ProtocolMethodPayload for Select
impl From<Select> for ClassMethod {
    fn from(from: Select) -> Self {
        ClassMethod::Select(from)
    } // fn from()
} // impl From<Select> for ClassMethod

impl From<Select> for super::SpecMethod<'static> {
    fn from(from: Select) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<Select> for ::super::SpecMethod
impl ::method::tx::SelectOkMethod for ::Amqp9_0 {
    type Payload = SelectOk;
} // impl ::method::tx::SelectOkMethod for ::Amqp9_0

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        ::std::result::Result::Ok(())
    }
} // impl Encodable

#[test]
fn test_select_ok_encodable_bytes_written_matches_len() {
    let payload: SelectOk = Default::default();
    let expected_len = ::Encodable::encoded_size(&payload);
    let mut writer = ::std::io::Cursor::new(Vec::with_capacity(expected_len));
    ::Encodable::write_encoded_to(&payload, &mut writer).unwrap();
    let payload = writer.into_inner();

    if payload.len() != expected_len {
        panic!("Expected payload len {}, got {}, {:?}",
               expected_len,
               payload.len(),
               &payload[..]);
    }
}



impl ::ProtocolMethodPayload for SelectOk {
    fn class(&self) -> ::Class {
        ::Class::Tx
    }
    fn class_id(&self) -> u16 {
        90
    }
    fn class_name(&self) -> &'static str {
        "tx"
    }
    fn method_id(&self) -> u16 {
        11
    }
    fn method_name(&self) -> &'static str {
        "select-ok"
    }
} // impl ::ProtocolMethodPayload for SelectOk
impl From<SelectOk> for ClassMethod {
    fn from(from: SelectOk) -> Self {
        ClassMethod::SelectOk(from)
    } // fn from()
} // impl From<SelectOk> for ClassMethod

impl From<SelectOk> for super::SpecMethod<'static> {
    fn from(from: SelectOk) -> Self {
        super::SpecMethod::Tx(from.into())
    } // fn default()
} // impl From<SelectOk> for ::super::SpecMethod

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
    fn write_encoded_to<W>(&self, _: &mut W) -> ::std::io::Result<()>
        where W: ::std::io::Write
    {
        unimplemented!()
    } // fn write_encoded_to()
} // impl ::Encodable for ClassMethod

impl<'a> ::ProtocolMethodPayload for ClassMethod {
    fn class(&self) -> ::Class {
        match *self {
            ClassMethod::Commit(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::CommitOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Rollback(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::RollbackOk(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class(method),

        } // match *self

    } // fn class

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

    fn class_name(&self) -> &'static str {
        match *self {
            ClassMethod::Commit(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::CommitOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Rollback(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::RollbackOk(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::class_name(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::class_name(method),

        } // match *self

    } // fn class_name

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

    fn method_name(&self) -> &'static str {
        match *self {
            ClassMethod::Commit(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::CommitOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Rollback(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::RollbackOk(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::Select(ref method) => ::ProtocolMethodPayload::method_name(method),
            ClassMethod::SelectOk(ref method) => ::ProtocolMethodPayload::method_name(method),

        } // match *self

    } // fn method_name
} // impl ProtocolMethodPayload for ClassMethod
