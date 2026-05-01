
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/identifier/Identifier.md")))]
#[::unity2::class(namespace = "App", name = "Identifier")]
#[parent(crate::system::object::Object)]
pub struct Identifier {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_UserId0")]
    pub m_user_id0: u64,
    #[rename(name = "m_UserId1")]
    pub m_user_id1: u64,
    #[rename(name = "m_MakeTime")]
    pub m_make_time: u64,
}

#[cfg(feature = "app-identifier")]
#[::unity2::methods]
impl Identifier {
    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create_2(self, random: crate::app::random_2::Random_2) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, src: crate::app::identifier::Identifier) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DummyDeserialize", args = 1)]
    pub fn dummy_deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::app::identifier::Identifier,
        rhs: crate::app::identifier::Identifier,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::app::identifier::Identifier,
        rhs: crate::app::identifier::Identifier,
    ) -> bool;

    #[method(name = "MakeUserId", args = 2)]
    pub fn make_user_id(self, v0: u32, v1: u32) -> u64;

    #[method(name = "MakeUserId", args = 1)]
    pub fn make_user_id_2(self, random: crate::app::random_2::Random_2) -> u64;

    #[method(name = "MakeMakeTime", args = 1)]
    pub fn make_make_time(self, random: crate::app::random_2::Random_2) -> u64;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-identifier")]
impl Identifier {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Identifier),
                ::core::stringify!(new),
            )
        });
        <Self as IIdentifierMethods>::ctor(this);
        this
    }
}
