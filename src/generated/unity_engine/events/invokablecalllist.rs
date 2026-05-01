
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/invokablecalllist/InvokableCallList.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "InvokableCallList")]
#[parent(crate::system::object::Object)]
pub struct InvokableCallList {
    #[rename(name = "m_PersistentCalls")]
    pub m_persistent_calls: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    >,
    #[rename(name = "m_RuntimeCalls")]
    pub m_runtime_calls: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    >,
    #[rename(name = "m_ExecutingCalls")]
    pub m_executing_calls: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    >,
    #[rename(name = "m_NeedsUpdate")]
    pub m_needs_update: bool,
}

#[cfg(feature = "unity_engine-events-invokablecalllist")]
#[::unity2::methods]
impl InvokableCallList {
    #[method(name = "AddPersistentInvokableCall", args = 1)]
    pub fn add_persistent_invokable_call(
        self,
        call: crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    ) -> ();

    #[method(name = "AddListener", args = 1)]
    pub fn add_listener(
        self,
        call: crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    ) -> ();

    #[method(name = "RemoveListener", args = 2)]
    pub fn remove_listener(
        self,
        target_obj: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> ();

    #[method(name = "ClearPersistent", args = 0)]
    pub fn clear_persistent(self) -> ();

    #[method(name = "PrepareInvoke", args = 0)]
    pub fn prepare_invoke(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-events-invokablecalllist")]
impl InvokableCallList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvokableCallList),
                ::core::stringify!(new),
            )
        });
        <Self as IInvokableCallListMethods>::ctor(this);
        this
    }
}
