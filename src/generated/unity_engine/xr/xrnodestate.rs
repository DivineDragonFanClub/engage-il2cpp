
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/xrnodestate/XRNodeState.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct XRNodeState {
    pub m_type: crate::unity_engine::xr::xrnode::XRNode,
    pub m_available_fields: crate::unity_engine::xr::availabletrackingdata::AvailableTrackingData,
    pub m_position: crate::unity_engine::vector3::Vector3,
    pub m_rotation: crate::unity_engine::quaternion::Quaternion,
    pub m_velocity: crate::unity_engine::vector3::Vector3,
    pub m_angular_velocity: crate::unity_engine::vector3::Vector3,
    pub m_acceleration: crate::unity_engine::vector3::Vector3,
    pub m_angular_acceleration: crate::unity_engine::vector3::Vector3,
    pub m_tracked: i32,
    pub m_unique_id: u64,
}

impl ::unity2::ClassIdentity for XRNodeState {
    const NAMESPACE: &'static str = "UnityEngine.XR";

    const NAME: &'static str = "XRNodeState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for XRNodeState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-xr-xrnodestate")]
#[::unity2::methods(value)]
impl XRNodeState {
    #[method(name = "set_uniqueID", args = 1)]
    pub fn set_unique_id(self, value: u64) -> ();

    #[method(name = "set_nodeType", args = 1)]
    pub fn set_node_type(self, value: crate::unity_engine::xr::xrnode::XRNode) -> ();

    #[method(name = "set_tracked", args = 1)]
    pub fn set_tracked(self, value: bool) -> ();
}
