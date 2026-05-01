
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gameobject/GameObject.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GameObject")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct GameObject {}

#[cfg(feature = "unity_engine-gameobject")]
#[::unity2::methods]
impl GameObject {
    #[method(name = "CreatePrimitive", args = 1)]
    pub fn create_primitive(
        r#type: crate::unity_engine::primitivetype::PrimitiveType,
    ) -> crate::unity_engine::gameobject::GameObject;

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

    #[method(name = "GetComponentByName", args = 1)]
    pub fn get_component_by_name(
        self,
        r#type: ::unity2::Il2CppString,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponent", args = 1)]
    pub fn get_component_2(
        self,
        r#type: ::unity2::Il2CppString,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentInChildren", args = 2)]
    pub fn get_component_in_children(
        self,
        r#type: ::unity2::SystemType,
        include_inactive: bool,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentInChildren", args = 1)]
    pub fn get_component_in_children_2(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentInParent", args = 2)]
    pub fn get_component_in_parent(
        self,
        r#type: ::unity2::SystemType,
        include_inactive: bool,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentInParent", args = 1)]
    pub fn get_component_in_parent_2(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "GetComponentsInternal", args = 6)]
    pub fn get_components_internal(
        self,
        r#type: ::unity2::SystemType,
        use_search_type_as_array_return_type: bool,
        recursive: bool,
        include_inactive: bool,
        reverse: bool,
        result_list: crate::system::object::Object,
    ) -> ::unity2::IlInstance;

    #[method(name = "GetComponents", args = 1)]
    pub fn get_components(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::component::Component>;

    #[method(name = "GetComponents", args = 2)]
    pub fn get_components_2(
        self,
        r#type: ::unity2::SystemType,
        results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::component::Component,
        >,
    ) -> ();

    #[method(name = "GetComponentsInChildren", args = 1)]
    pub fn get_components_in_children(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::component::Component>;

    #[method(name = "GetComponentsInChildren", args = 2)]
    pub fn get_components_in_children_2(
        self,
        r#type: ::unity2::SystemType,
        include_inactive: bool,
    ) -> ::unity2::Array<crate::unity_engine::component::Component>;

    #[method(name = "GetComponentsInParent", args = 1)]
    pub fn get_components_in_parent(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::component::Component>;

    #[method(name = "GetComponentsInParent", args = 2)]
    pub fn get_components_in_parent_2(
        self,
        r#type: ::unity2::SystemType,
        include_inactive: bool,
    ) -> ::unity2::Array<crate::unity_engine::component::Component>;

    #[method(name = "TryGetComponent", args = 2)]
    pub fn try_get_component(
        self,
        r#type: ::unity2::SystemType,
        component: crate::unity_engine::component::Component,
    ) -> bool;

    #[method(name = "TryGetComponentInternal", args = 1)]
    pub fn try_get_component_internal(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "TryGetComponentFastPath", args = 2)]
    pub fn try_get_component_fast_path(
        self,
        r#type: ::unity2::SystemType,
        one_further_than_result_value: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "FindWithTag", args = 1)]
    pub fn find_with_tag(
        tag: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SendMessageUpwards", args = 2)]
    pub fn send_message_upwards(
        self,
        method_name: ::unity2::Il2CppString,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "SendMessage", args = 2)]
    pub fn send_message(
        self,
        method_name: ::unity2::Il2CppString,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "BroadcastMessage", args = 2)]
    pub fn broadcast_message(
        self,
        method_name: ::unity2::Il2CppString,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "AddComponentInternal", args = 1)]
    pub fn add_component_internal(
        self,
        class_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "Internal_AddComponentWithType", args = 1)]
    pub fn internal_add_component_with_type(
        self,
        component_type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "AddComponent", args = 1)]
    pub fn add_component(
        self,
        component_type: ::unity2::SystemType,
    ) -> crate::unity_engine::component::Component;

    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_layer", args = 0)]
    pub fn get_layer(self) -> i32;

    #[method(name = "set_layer", args = 1)]
    pub fn set_layer(self, value: i32) -> ();

    #[method(name = "get_active", args = 0)]
    pub fn get_active(self) -> bool;

    #[method(name = "set_active", args = 1)]
    pub fn set_active(self, value: bool) -> ();

    #[method(name = "get_activeSelf", args = 0)]
    pub fn get_active_self(self) -> bool;

    #[method(name = "get_activeInHierarchy", args = 0)]
    pub fn get_active_in_hierarchy(self) -> bool;

    #[method(name = "SetActiveRecursively", args = 1)]
    pub fn set_active_recursively(self, state: bool) -> ();

    #[method(name = "get_isStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "set_isStatic", args = 1)]
    pub fn set_is_static(self, value: bool) -> ();

    #[method(name = "get_isStaticBatchable", args = 0)]
    pub fn get_is_static_batchable(self) -> bool;

    #[method(name = "get_tag", args = 0)]
    pub fn get_tag(self) -> ::unity2::Il2CppString;

    #[method(name = "set_tag", args = 1)]
    pub fn set_tag(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "CompareTag", args = 1)]
    pub fn compare_tag(self, tag: ::unity2::Il2CppString) -> bool;

    #[method(name = "FindGameObjectWithTag", args = 1)]
    pub fn find_game_object_with_tag(
        tag: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindGameObjectsWithTag", args = 1)]
    pub fn find_game_objects_with_tag(
        tag: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::gameobject::GameObject>;

    #[method(name = "SendMessageUpwards", args = 3)]
    pub fn send_message_upwards_2(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "SendMessageUpwards", args = 2)]
    pub fn send_message_upwards_3(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "SendMessageUpwards", args = 1)]
    pub fn send_message_upwards_4(self, method_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SendMessage", args = 3)]
    pub fn send_message_2(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "SendMessage", args = 2)]
    pub fn send_message_3(
        self,
        method_name: ::unity2::Il2CppString,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "SendMessage", args = 1)]
    pub fn send_message_4(self, method_name: ::unity2::Il2CppString) -> ();

    #[method(name = "BroadcastMessage", args = 3)]
    pub fn broadcast_message_2(
        self,
        method_name: ::unity2::Il2CppString,
        parameter: crate::system::object::Object,
        options: crate::unity_engine::sendmessageoptions::SendMessageOptions,
    ) -> ();

    #[method(name = "BroadcastMessage", args = 2)]
    pub fn broadcast_message_3(
        self,
        method_name: ::unity2::Il2CppString,
        parameter: crate::system::object::Object,
    ) -> ();

    #[method(name = "BroadcastMessage", args = 1)]
    pub fn broadcast_message_4(self, method_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        name: ::unity2::Il2CppString,
        components: ::unity2::Array<::unity2::SystemType>,
    ) -> ();

    #[method(name = "Internal_CreateGameObject", args = 2)]
    pub fn internal_create_game_object(
        self_: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Find", args = 1)]
    pub fn find(name: ::unity2::Il2CppString) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_scene", args = 0)]
    pub fn get_scene(self) -> crate::unity_engine::scene_management::scene::Scene;

    #[method(name = "get_sceneCullingMask", args = 0)]
    pub fn get_scene_culling_mask(self) -> u64;

    #[method(name = "get_gameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_scene_Injected", args = 1)]
    pub fn get_scene_injected(self, ret: crate::unity_engine::scene_management::scene::Scene)
        -> ();
}

#[cfg(feature = "unity_engine-gameobject")]
impl GameObject {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameObject),
                ::core::stringify!(new),
            )
        });
        <Self as IGameObjectMethods>::ctor(this, name);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameObject),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGameObjectMethods>::ctor_2(this);
        this
    }

    pub fn new_3(
        name: ::unity2::Il2CppString,
        components: ::unity2::Array<::unity2::SystemType>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameObject),
                ::core::stringify!(new_3),
            )
        });
        <Self as IGameObjectMethods>::ctor_3(this, name, components);
        this
    }
}
