
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/inputtracking/InputTracking.md")))]
#[::unity2::class(namespace = "UnityEngine.XR", name = "InputTracking")]
#[parent(crate::system::object::Object)]
pub struct InputTracking {
    #[static_field]
    #[rename(name = "trackingAcquired")]
    pub tracking_acquired:
        crate::system::action_1::Action_1<crate::unity_engine::xr::xrnodestate::XRNodeState>,
    #[static_field]
    #[rename(name = "trackingLost")]
    pub tracking_lost:
        crate::system::action_1::Action_1<crate::unity_engine::xr::xrnodestate::XRNodeState>,
    #[static_field]
    #[rename(name = "nodeAdded")]
    pub node_added:
        crate::system::action_1::Action_1<crate::unity_engine::xr::xrnodestate::XRNodeState>,
    #[static_field]
    #[rename(name = "nodeRemoved")]
    pub node_removed:
        crate::system::action_1::Action_1<crate::unity_engine::xr::xrnodestate::XRNodeState>,
}

#[cfg(feature = "unity_engine-xr-inputtracking")]
#[::unity2::methods]
impl InputTracking {
    #[method(name = "InvokeTrackingEvent", args = 4)]
    pub fn invoke_tracking_event(
        event_type: crate::unity_engine::xr::inputtracking::InputTracking_TrackingStateEventType,
        node_type: crate::unity_engine::xr::xrnode::XRNode,
        unique_id: i64,
        tracked: bool,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/inputtracking/InputTracking_TrackingStateEventType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct InputTracking_TrackingStateEventType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for InputTracking_TrackingStateEventType {
    const NAMESPACE: &'static str = "UnityEngine.XR";

    const NAME: &'static str = "InputTracking.TrackingStateEventType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InputTracking_TrackingStateEventType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl InputTracking_TrackingStateEventType {
    pub fn node_added() -> Self {
        Self { value: 0 }
    }

    pub fn node_removed() -> Self {
        Self { value: 1 }
    }

    pub fn tracking_acquired() -> Self {
        Self { value: 2 }
    }

    pub fn tracking_lost() -> Self {
        Self { value: 3 }
    }
}
