
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/monobehaviour/MonoBehaviour.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "MonoBehaviour")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct MonoBehaviour {}

#[cfg(feature = "unity_engine-monobehaviour")]
#[::unity2::methods]
impl MonoBehaviour {
    #[method(name = "IsInvoking", args = 0)]
    pub fn is_invoking(self) -> bool;

    #[method(name = "CancelInvoke", args = 0)]
    pub fn cancel_invoke(self) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, method_name: ::unity2::Il2CppString, time: f32) -> ();

    #[method(name = "InvokeRepeating", args = 3)]
    pub fn invoke_repeating(
        self,
        method_name: ::unity2::Il2CppString,
        time: f32,
        repeat_rate: f32,
    ) -> ();

    #[method(name = "CancelInvoke", args = 1)]
    pub fn cancel_invoke_2(self, method_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsInvoking", args = 1)]
    pub fn is_invoking_2(self, method_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "StartCoroutine", args = 1)]
    pub fn start_coroutine(
        self,
        method_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StartCoroutine", args = 2)]
    pub fn start_coroutine_2(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StartCoroutine", args = 1)]
    pub fn start_coroutine_3(
        self,
        routine: crate::system::collections::ienumerator::IEnumerator,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StartCoroutine_Auto", args = 1)]
    pub fn start_coroutine_auto(
        self,
        routine: crate::system::collections::ienumerator::IEnumerator,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StopCoroutine", args = 1)]
    pub fn stop_coroutine(
        self,
        routine: crate::system::collections::ienumerator::IEnumerator,
    ) -> ();

    #[method(name = "StopCoroutine", args = 1)]
    pub fn stop_coroutine_2(self, routine: crate::unity_engine::coroutine::Coroutine) -> ();

    #[method(name = "StopCoroutine", args = 1)]
    pub fn stop_coroutine_3(self, method_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopAllCoroutines", args = 0)]
    pub fn stop_all_coroutines(self) -> ();

    #[method(name = "get_useGUILayout", args = 0)]
    pub fn get_use_gui_layout(self) -> bool;

    #[method(name = "set_useGUILayout", args = 1)]
    pub fn set_use_gui_layout(self, value: bool) -> ();

    #[method(name = "print", args = 1)]
    pub fn print(message: crate::system::object::Object) -> ();

    #[method(name = "Internal_CancelInvokeAll", args = 1)]
    pub fn internal_cancel_invoke_all(
        self_: crate::unity_engine::monobehaviour::MonoBehaviour,
    ) -> ();

    #[method(name = "Internal_IsInvokingAll", args = 1)]
    pub fn internal_is_invoking_all(
        self_: crate::unity_engine::monobehaviour::MonoBehaviour,
    ) -> bool;

    #[method(name = "InvokeDelayed", args = 4)]
    pub fn invoke_delayed(
        self_: crate::unity_engine::monobehaviour::MonoBehaviour,
        method_name: ::unity2::Il2CppString,
        time: f32,
        repeat_rate: f32,
    ) -> ();

    #[method(name = "CancelInvoke", args = 2)]
    pub fn cancel_invoke_3(
        self_: crate::unity_engine::monobehaviour::MonoBehaviour,
        method_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "IsInvoking", args = 2)]
    pub fn is_invoking_3(
        self_: crate::unity_engine::monobehaviour::MonoBehaviour,
        method_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "IsObjectMonoBehaviour", args = 1)]
    pub fn is_object_mono_behaviour(obj: crate::unity_engine::object_2::Object_2) -> bool;

    #[method(name = "StartCoroutineManaged", args = 2)]
    pub fn start_coroutine_managed(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StartCoroutineManaged2", args = 1)]
    pub fn start_coroutine_managed2(
        self,
        enumerator: crate::system::collections::ienumerator::IEnumerator,
    ) -> crate::unity_engine::coroutine::Coroutine;

    #[method(name = "StopCoroutineManaged", args = 1)]
    pub fn stop_coroutine_managed(self, routine: crate::unity_engine::coroutine::Coroutine) -> ();

    #[method(name = "StopCoroutineFromEnumeratorManaged", args = 1)]
    pub fn stop_coroutine_from_enumerator_managed(
        self,
        routine: crate::system::collections::ienumerator::IEnumerator,
    ) -> ();

    #[method(name = "GetScriptClassName", args = 0)]
    pub fn get_script_class_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-monobehaviour")]
impl MonoBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoBehaviourMethods>::ctor(this);
        this
    }
}
