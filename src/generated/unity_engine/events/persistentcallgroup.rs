
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/persistentcallgroup/PersistentCallGroup.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "PersistentCallGroup")]
#[parent(crate::system::object::Object)]
pub struct PersistentCallGroup {
    #[rename(name = "m_Calls")]
    pub m_calls: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::persistentcall::PersistentCall,
    >,
}

#[cfg(feature = "unity_engine-events-persistentcallgroup")]
#[::unity2::methods]
impl PersistentCallGroup {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Initialize", args = 2)]
    pub fn initialize(
        self,
        invokable_list: crate::unity_engine::events::invokablecalllist::InvokableCallList,
        unity_event_base: crate::unity_engine::events::unityeventbase::UnityEventBase,
    ) -> ();
}

#[cfg(feature = "unity_engine-events-persistentcallgroup")]
impl PersistentCallGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersistentCallGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IPersistentCallGroupMethods>::ctor(this);
        this
    }
}
