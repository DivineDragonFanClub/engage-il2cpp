
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/extents/Extents.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Extents {
    pub min: crate::unity_engine::vector2::Vector2,
    pub max: crate::unity_engine::vector2::Vector2,
}

impl ::unity2::ClassIdentity for Extents {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "Extents";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Extents {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-extents")]
#[::unity2::methods(value)]
impl Extents {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        min: crate::unity_engine::vector2::Vector2,
        max: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
