
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ut/Ut.md")))]
#[::unity2::class(namespace = "App", name = "Ut")]
#[parent(crate::system::object::Object)]
pub struct Ut {
    #[static_field]
    #[rename(name = "FNV_OFFSET_BASIS_32")]
    pub fnv_offset_basis_32: u32,
    #[static_field]
    #[rename(name = "FNV_PRIME_32")]
    pub fnv_prime_32: u32,
    #[static_field]
    #[rename(name = "s_ValueToString")]
    pub s_value_to_string: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-ut")]
#[::unity2::methods]
impl Ut {
    #[method(name = "Mult", args = 2)]
    pub fn mult(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Mult", args = 2)]
    pub fn mult_2(
        a: crate::unity_engine::vector2::Vector2,
        b: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "DegToRad", args = 1)]
    pub fn deg_to_rad(deg: f32) -> f32;

    #[method(name = "DegToRad", args = 1)]
    pub fn deg_to_rad_2(
        rotate: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "RoundDeg", args = 1)]
    pub fn round_deg(deg: f32) -> f32;

    #[method(name = "ClampAngle", args = 2)]
    pub fn clamp_angle(angle: f32, limit: f32) -> f32;

    #[method(name = "ClampAngle", args = 2)]
    pub fn clamp_angle_2(
        angle: crate::unity_engine::vector3::Vector3,
        limit: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ClampAngle", args = 2)]
    pub fn clamp_angle_3(
        rotation: crate::unity_engine::quaternion::Quaternion,
        limit: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "MSecToFrame", args = 1)]
    pub fn m_sec_to_frame(msec: f32) -> f32;

    #[method(name = "FrameToMSEc", args = 1)]
    pub fn frame_to_ms_ec(frame: f32) -> f32;

    #[method(name = "GetFrameStep", args = 0)]
    pub fn get_frame_step() -> f32;

    #[method(name = "IsIntercect", args = 8)]
    pub fn is_intercect(
        min_x1: i32,
        min_z1: i32,
        max_x1: i32,
        max_z1: i32,
        min_x2: i32,
        min_z2: i32,
        max_x2: i32,
        max_z2: i32,
    ) -> bool;

    #[method(name = "GetFastInputScale", args = 0)]
    pub fn get_fast_input_scale() -> i32;

    #[method(name = "ChangeKeyValue", args = 4)]
    pub fn change_key_value(value: i32, min: i32, max: i32, step: i32) -> i32;

    #[method(name = "ChangeKeyValue", args = 4)]
    pub fn change_key_value_2(value: f32, min: f32, max: f32, step: f32) -> f32;

    #[method(name = "ChangeKeyValue", args = 1)]
    pub fn change_key_value_3(value: bool) -> bool;

    #[method(name = "ChangeKeyValueImpl", args = 4)]
    pub fn change_key_value_impl(value: f64, min: f64, max: f64, step: f64) -> f64;

    #[method(name = "ChangeKeyEnum", args = 1)]
    pub fn change_key_enum(obj: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "FindGameObject", args = 2)]
    pub fn find_game_object(
        name: ::unity2::Il2CppString,
        warning: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryCreateGameObject", args = 2)]
    pub fn try_create_game_object(
        name: ::unity2::Il2CppString,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateGameObject", args = 2)]
    pub fn create_game_object(
        name: ::unity2::Il2CppString,
        parent: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindRootObject", args = 1)]
    pub fn find_root_object(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryCreateRootObject", args = 1)]
    pub fn try_create_root_object(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateGameObject", args = 2)]
    pub fn create_game_object_2(
        name: ::unity2::Il2CppString,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CreateGameObjectByAsset", args = 2)]
    pub fn create_game_object_by_asset(
        original: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "DestroyGameObject", args = 1)]
    pub fn destroy_game_object(game_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "DestroyChildObject", args = 1)]
    pub fn destroy_child_object(game_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetLayerRecursively", args = 2)]
    pub fn set_layer_recursively(
        transform: crate::unity_engine::transform::Transform,
        layer: i32,
    ) -> bool;

    #[method(name = "SetLayerRecursively", args = 2)]
    pub fn set_layer_recursively_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        layer: i32,
    ) -> bool;

    #[method(name = "SetLayerRecursively", args = 2)]
    pub fn set_layer_recursively_3(
        game_object: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "CreateDebugTextObject", args = 3)]
    pub fn create_debug_text_object(
        parent: crate::unity_engine::transform::Transform,
        name: ::unity2::Il2CppString,
        size: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetObjectFullPath", args = 1)]
    pub fn get_object_full_path(
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FindChildGameObject", args = 2)]
    pub fn find_child_game_object(
        game_object: crate::unity_engine::gameobject::GameObject,
        child_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "FindChildTransform", args = 2)]
    pub fn find_child_transform(
        parent: crate::unity_engine::transform::Transform,
        child_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetTopParentObject", args = 1)]
    pub fn get_top_parent_object(
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetUniqueObjectName", args = 1)]
    pub fn get_unique_object_name(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetCurrentScenePath", args = 0)]
    pub fn get_current_scene_path() -> ::unity2::Il2CppString;

    #[method(name = "GetCurrentSceneDir", args = 0)]
    pub fn get_current_scene_dir() -> ::unity2::Il2CppString;

    #[method(name = "GetCurrentSceneName", args = 0)]
    pub fn get_current_scene_name() -> ::unity2::Il2CppString;

    #[method(name = "GetSceneName", args = 1)]
    pub fn get_scene_name(
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetSceneName", args = 1)]
    pub fn get_scene_name_2(
        component: crate::unity_engine::component::Component,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPrefixless", args = 1)]
    pub fn get_prefixless(path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetRotation", args = 2)]
    pub fn get_rotation(
        start: crate::unity_engine::vector3::Vector3,
        end: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetCurrentAnimationClip", args = 2)]
    pub fn get_current_animation_clip(
        animator: crate::unity_engine::animator::Animator,
        layer_index: i32,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "RoundKB", args = 1)]
    pub fn round_kb(size: i64) -> i64;

    #[method(name = "SetComponentEnable", args = 2)]
    pub fn set_component_enable(
        game_object: crate::unity_engine::gameobject::GameObject,
        enable: bool,
    ) -> ();

    #[method(name = "DumpTransform", args = 2)]
    pub fn dump_transform(transform: crate::unity_engine::transform::Transform, depth: i32) -> ();

    #[method(name = "ClearProperties", args = 2)]
    pub fn clear_properties(r#type: ::unity2::SystemType, obj: crate::system::object::Object)
        -> ();

    #[method(name = "CopyProperties", args = 3)]
    pub fn copy_properties(
        r#type: ::unity2::SystemType,
        src: crate::system::object::Object,
        dst: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetStreamingAssetsPath", args = 0)]
    pub fn get_streaming_assets_path() -> ::unity2::Il2CppString;

    #[method(name = "GetStreamingAssetsPath", args = 2)]
    pub fn get_streaming_assets_path_2(
        path: ::unity2::Il2CppString,
        platform: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SaveMembers", args = 1)]
    pub fn save_members(obj: crate::system::object::Object) -> ();

    #[method(name = "LoadMembers", args = 1)]
    pub fn load_members(obj: crate::system::object::Object) -> ();

    #[method(name = "GetMemberItems", args = 1)]
    pub fn get_member_items(
        instance: crate::system::object::Object,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>;

    #[method(name = "GetFieldItems", args = 1)]
    pub fn get_field_items(
        instance: crate::system::object::Object,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>;

    #[method(name = "GetPropetyItems", args = 1)]
    pub fn get_propety_items(
        instance: crate::system::object::Object,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>;

    #[method(name = "GetInstanceMaterials", args = 1)]
    pub fn get_instance_materials(
        render: crate::unity_engine::renderer::Renderer,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "Hash_FNV_1", args = 1)]
    pub fn hash_fnv_1(bytes: ::unity2::Array<u8>) -> i32;

    #[method(name = "Hash_FNV_1", args = 2)]
    pub fn hash_fnv_1_2(bytes: ::unity2::Array<u8>, length: i32) -> i32;

    #[method(name = "Hash_FNV_1", args = 1)]
    pub fn hash_fnv_1_3(values: ::unity2::Array<u32>) -> i32;

    #[method(name = "Hash_FNV_1", args = 2)]
    pub fn hash_fnv_1_4(values: ::unity2::Array<u32>, length: i32) -> i32;

    #[method(name = "Hash_FNV_1", args = 1)]
    pub fn hash_fnv_1_5(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "Hash_FNV_1", args = 1)]
    pub fn hash_fnv_1_6(array: crate::system::collections::bitarray::BitArray) -> i32;

    #[method(name = "GetAllAssetPaths", args = 2)]
    pub fn get_all_asset_paths(
        root: ::unity2::Il2CppString,
        extensions: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "ToAssetPath", args = 1)]
    pub fn to_asset_path(path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "ToSystemPath", args = 1)]
    pub fn to_system_path(path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "UnityEditorSelectObject", args = 1)]
    pub fn unity_editor_select_object(
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "UnityEditorSelectObject", args = 1)]
    pub fn unity_editor_select_object_2(component: crate::unity_engine::component::Component)
        -> ();

    #[method(name = "EachChildren", args = 2)]
    pub fn each_children(
        go: crate::unity_engine::gameobject::GameObject,
        func: crate::app::ut::Ut_GameObjectFunction,
    ) -> ();

    #[method(name = "EachParents", args = 2)]
    pub fn each_parents(
        go: crate::unity_engine::gameobject::GameObject,
        func: crate::app::ut::Ut_GameObjectFunction,
    ) -> ();

    #[method(name = "SetOneShotParticle", args = 1)]
    pub fn set_one_shot_particle(go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "ValueToString", args = 1)]
    pub fn value_to_string(value: i32) -> ::unity2::Il2CppString;

    #[method(name = "SetTextValue", args = 2)]
    pub fn set_text_value(tmp_text: crate::tm_pro::tmp_text::TMP_Text, value: i32) -> ();

    #[method(name = "ToStringWithComma", args = 1)]
    pub fn to_string_with_comma(value: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetLanguageLabel", args = 1)]
    pub fn get_language_label(label: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetShortName", args = 2)]
    pub fn get_short_name(name: ::unity2::Il2CppString, length: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetSizeName", args = 1)]
    pub fn get_size_name(size: i64) -> ::unity2::Il2CppString;

    #[method(name = "CaptureTexture", args = 2)]
    pub fn capture_texture(w: i32, h: i32) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "GetTimeStamp", args = 0)]
    pub fn get_time_stamp() -> ::unity2::Il2CppString;

    #[method(name = "ForceRebuildLayout", args = 1)]
    pub fn force_rebuild_layout(component: crate::unity_engine::component::Component) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ut")]
impl Ut {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ut),
                ::core::stringify!(new),
            )
        });
        <Self as IUtMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ut/Ut_EnumList.md")))]
#[::unity2::class(namespace = "App", name = "Ut.EnumList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: system :: object :: Object >)]
pub struct Ut_EnumList {}

#[cfg(feature = "app-ut")]
#[::unity2::methods]
impl Ut_EnumList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, r#type: ::unity2::SystemType) -> ();

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(self, value: crate::system::object::Object) -> i32;
}

#[cfg(feature = "app-ut")]
impl Ut_EnumList {
    pub fn new(r#type: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ut_EnumList),
                ::core::stringify!(new),
            )
        });
        <Self as IUt_EnumListMethods>::ctor(this, r#type);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ut/Ut_GameObjectFunction.md")))]
#[::unity2::class(namespace = "App", name = "Ut.GameObjectFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Ut_GameObjectFunction {}

#[cfg(feature = "app-ut")]
#[::unity2::methods]
impl Ut_GameObjectFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, go: crate::unity_engine::gameobject::GameObject) -> ();
}

#[cfg(feature = "app-ut")]
impl Ut_GameObjectFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ut_GameObjectFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IUt_GameObjectFunctionMethods>::ctor(this, object, method);
        this
    }
}
