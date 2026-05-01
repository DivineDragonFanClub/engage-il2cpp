
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/kerningpairkey/KerningPairKey.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct KerningPairKey {
    pub ascii_left: u32,
    pub ascii_right: u32,
    pub key: u32,
}

impl ::unity2::ClassIdentity for KerningPairKey {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "KerningPairKey";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for KerningPairKey {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-kerningpairkey")]
#[::unity2::methods(value)]
impl KerningPairKey {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, ascii_left: u32, ascii_right: u32) -> ();
}
