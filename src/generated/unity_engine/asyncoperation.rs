
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/asyncoperation/AsyncOperation.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AsyncOperation")]
#[parent(crate::unity_engine::yieldinstruction::YieldInstruction)]
pub struct AsyncOperation {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_completeCallback")]
    pub m_complete_callback:
        crate::system::action_1::Action_1<crate::unity_engine::asyncoperation::AsyncOperation>,
}

#[cfg(feature = "unity_engine-asyncoperation")]
#[::unity2::methods]
impl AsyncOperation {
    #[method(name = "InternalDestroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "get_isDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "set_priority", args = 1)]
    pub fn set_priority(self, value: i32) -> ();

    #[method(name = "get_allowSceneActivation", args = 0)]
    pub fn get_allow_scene_activation(self) -> bool;

    #[method(name = "set_allowSceneActivation", args = 1)]
    pub fn set_allow_scene_activation(self, value: bool) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "InvokeCompletionEvent", args = 0)]
    pub fn invoke_completion_event(self) -> ();

    #[method(name = "add_completed", args = 1)]
    pub fn add_completed(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::asyncoperation::AsyncOperation,
        >,
    ) -> ();

    #[method(name = "remove_completed", args = 1)]
    pub fn remove_completed(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::asyncoperation::AsyncOperation,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-asyncoperation")]
impl AsyncOperation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AsyncOperation),
                ::core::stringify!(new),
            )
        });
        <Self as IAsyncOperationMethods>::ctor(this);
        this
    }
}
