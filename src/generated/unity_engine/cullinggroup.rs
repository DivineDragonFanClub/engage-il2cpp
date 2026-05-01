
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cullinggroup/CullingGroup_StateChanged.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CullingGroup.StateChanged")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct CullingGroup_StateChanged {}

#[cfg(feature = "unity_engine-cullinggroup")]
#[::unity2::methods]
impl CullingGroup_StateChanged {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, sphere: crate::unity_engine::cullinggroupevent::CullingGroupEvent) -> ();
}

#[cfg(feature = "unity_engine-cullinggroup")]
impl CullingGroup_StateChanged {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CullingGroup_StateChanged),
                ::core::stringify!(new),
            )
        });
        <Self as ICullingGroup_StateChangedMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cullinggroup/CullingGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CullingGroup")]
#[parent(crate::system::object::Object)]
pub struct CullingGroup {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_OnStateChanged")]
    pub m_on_state_changed: crate::unity_engine::cullinggroup::CullingGroup_StateChanged,
}

#[cfg(feature = "unity_engine-cullinggroup")]
#[::unity2::methods]
impl CullingGroup {
    #[method(name = "SendEvents", args = 3)]
    pub fn send_events(
        culling_group: crate::unity_engine::cullinggroup::CullingGroup,
        events_ptr: ::unity2::IntPtr,
        count: i32,
    ) -> ();
}
