
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/contactfilter2d/ContactFilter2D.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ContactFilter2D {
    pub use_triggers: bool,
    pub use_layer_mask: bool,
    pub use_depth: bool,
    pub use_outside_depth: bool,
    pub use_normal_angle: bool,
    pub use_outside_normal_angle: bool,
    pub layer_mask: crate::unity_engine::layermask::LayerMask,
    pub min_depth: f32,
    pub max_depth: f32,
    pub min_normal_angle: f32,
    pub max_normal_angle: f32,
}

impl ::unity2::ClassIdentity for ContactFilter2D {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ContactFilter2D";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ContactFilter2D {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-contactfilter2d")]
#[::unity2::methods(value)]
impl ContactFilter2D {
    #[method(name = "CheckConsistency", args = 0)]
    pub fn check_consistency(self) -> ();

    #[method(name = "SetLayerMask", args = 1)]
    pub fn set_layer_mask(self, layer_mask: crate::unity_engine::layermask::LayerMask) -> ();

    #[method(name = "SetDepth", args = 2)]
    pub fn set_depth(self, min_depth: f32, max_depth: f32) -> ();

    #[method(name = "CreateLegacyFilter", args = 3)]
    pub fn create_legacy_filter(
        layer_mask: i32,
        min_depth: f32,
        max_depth: f32,
    ) -> crate::unity_engine::contactfilter2d::ContactFilter2D;

    #[method(name = "CheckConsistency_Injected", args = 1)]
    pub fn check_consistency_injected(
        unity_self: crate::unity_engine::contactfilter2d::ContactFilter2D,
    ) -> ();
}
