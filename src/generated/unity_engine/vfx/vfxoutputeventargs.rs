
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vfx/vfxoutputeventargs/VFXOutputEventArgs.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct VFXOutputEventArgs {}

impl ::unity2::ClassIdentity for VFXOutputEventArgs {
    const NAMESPACE: &'static str = "UnityEngine.VFX";

    const NAME: &'static str = "VFXOutputEventArgs";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VFXOutputEventArgs {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-vfx-vfxoutputeventargs")]
#[::unity2::methods(value)]
impl VFXOutputEventArgs {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name_id: i32,
        event_attribute: crate::unity_engine::vfx::vfxeventattribute::VFXEventAttribute,
    ) -> ();
}
