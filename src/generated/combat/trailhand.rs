
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/trailhand/TrailHand.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TrailHand {
    pub supplier: crate::combat::trailvertexsupplier::TrailVertexSupplier,
    pub mesh: crate::combat::trailmesh::TrailMesh,
    pub root_node: crate::unity_engine::transform::Transform,
    pub tip_node: crate::unity_engine::transform::Transform,
}

impl ::unity2::ClassIdentity for TrailHand {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "TrailHand";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TrailHand {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "combat-trailhand")]
#[::unity2::methods(value)]
impl TrailHand {
    #[method(name = "get_IsAlive", args = 0)]
    pub fn get_is_alive(self) -> bool;

    #[method(name = "SetRootAndTipTransform", args = 3)]
    pub fn set_root_and_tip_transform(
        self,
        t: crate::unity_engine::transform::Transform,
        root_name: ::unity2::Il2CppString,
        tip_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, dt: f32) -> ();
}
