
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/renderqueuerange/RenderQueueRange.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderQueueRange {
    pub m_lower_bound: i32,
    pub m_upper_bound: i32,
}

impl ::unity2::ClassIdentity for RenderQueueRange {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "RenderQueueRange";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderQueueRange {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-renderqueuerange")]
#[::unity2::methods(value)]
impl RenderQueueRange {
    #[method(name = "get_all", args = 0)]
    pub fn get_all() -> crate::unity_engine::rendering::renderqueuerange::RenderQueueRange;

    #[method(name = "get_opaque", args = 0)]
    pub fn get_opaque() -> crate::unity_engine::rendering::renderqueuerange::RenderQueueRange;

    #[method(name = "get_transparent", args = 0)]
    pub fn get_transparent() -> crate::unity_engine::rendering::renderqueuerange::RenderQueueRange;

    #[method(name = "set_lowerBound", args = 1)]
    pub fn set_lower_bound(self, value: i32) -> ();

    #[method(name = "set_upperBound", args = 1)]
    pub fn set_upper_bound(self, value: i32) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
