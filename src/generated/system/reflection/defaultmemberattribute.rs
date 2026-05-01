
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/defaultmemberattribute/DefaultMemberAttribute.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "DefaultMemberAttribute")]
pub struct DefaultMemberAttribute {
    #[rename(name = "m_memberName")]
    pub m_member_name: ::unity2::Il2CppString,
}

#[cfg(feature = "system-reflection-defaultmemberattribute")]
#[::unity2::methods]
impl DefaultMemberAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, member_name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MemberName", args = 0)]
    pub fn get_member_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "system-reflection-defaultmemberattribute")]
impl DefaultMemberAttribute {
    pub fn new(member_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DefaultMemberAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IDefaultMemberAttributeMethods>::ctor(this, member_name);
        this
    }
}
