
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/memberinfoserializationholder/MemberInfoSerializationHolder.md")))]
#[::unity2::class(
    namespace = "System.Reflection",
    name = "MemberInfoSerializationHolder"
)]
#[parent(crate::system::object::Object)]
pub struct MemberInfoSerializationHolder {
    #[rename(name = "m_memberName")]
    pub m_member_name: ::unity2::Il2CppString,
    #[rename(name = "m_reflectedType")]
    pub m_reflected_type: crate::system::runtimetype::RuntimeType,
    #[rename(name = "m_signature")]
    pub m_signature: ::unity2::Il2CppString,
    #[rename(name = "m_signature2")]
    pub m_signature2: ::unity2::Il2CppString,
    #[rename(name = "m_memberType")]
    pub m_member_type: crate::system::reflection::membertypes::MemberTypes,
}
