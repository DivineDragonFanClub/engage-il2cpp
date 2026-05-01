
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/component/Component.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Component")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Component {}

#[cfg(feature = "unity_engine-component")]
#[::unity2::methods]
impl Component {
    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_gameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetComponent", args = 1)]
    pub fn get_component(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentFastPath", args = 2)]
    pub fn get_component_fast_path(
        self,
        r#type: ::unity2::SystemType,
        one_further_than_result_value: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "GetComponentInChildren", args = 2)]
    pub fn get_component_in_children(
        self,
        t: ::unity2::SystemType,
        include_inactive: bool,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentInParent", args = 1)]
    pub fn get_component_in_parent(
        self,
        t: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentsForListInternal", args = 2)]
    pub fn get_components_for_list_internal(
        self,
        search_type: ::unity2::SystemType,
        result_list: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetComponents", args = 2)]
    pub fn get_components(
        self,
        r#type: ::unity2::SystemType,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::component::Component,
        >,
    ) -> ();

    #[method(name = "get_tag", args = 0)]
    pub fn get_tag(self) -> ::unity2::Il2CppString;

    #[method(name = "set_tag", args = 1)]
    pub fn set_tag(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "CompareTag", args = 1)]
    pub fn compare_tag(self, tag: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-component")]
impl Component {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Component),
                ::core::stringify!(new),
            )
        });
        <Self as IComponentMethods>::ctor(this);
        this
    }
}
