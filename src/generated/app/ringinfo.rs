
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringinfo/RingInfo_RingInfoWindowRingModel.md")))]
#[::unity2::class(namespace = "App", name = "RingInfo.RingInfoWindowRingModel")]
#[parent(crate::system::object::Object)]
pub struct RingInfo_RingInfoWindowRingModel {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CameraObject")]
    pub m_camera_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
}

#[cfg(feature = "app-ringinfo")]
#[::unity2::methods]
impl RingInfo_RingInfoWindowRingModel {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, index: i32) -> ();

    #[method(name = "CreateAsync", args = 0)]
    pub fn create_async(self) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "CreateImpl", args = 0)]
    pub fn create_impl(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> bool;

    #[method(name = "GetRenderTexture", args = 0)]
    pub fn get_render_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "get_PrefabObject", args = 0)]
    pub fn get_prefab_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_RingModelRoot", args = 0)]
    pub fn get_ring_model_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetAnimatorEnable", args = 1)]
    pub fn set_animator_enable(self, value: bool) -> ();

    #[method(name = "GetDirtyTextureValue", args = 1)]
    pub fn get_dirty_texture_value(self, dirty: i32) -> f32;

    #[method(name = "PlayDecisionAnim", args = 0)]
    pub fn play_decision_anim(self) -> ();

    #[method(name = "GetModelPosition", args = 0)]
    pub fn get_model_position(self) -> crate::unity_engine::vector3::Vector3;
}

#[cfg(feature = "app-ringinfo")]
impl RingInfo_RingInfoWindowRingModel {
    pub fn new(index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingInfo_RingInfoWindowRingModel),
                ::core::stringify!(new),
            )
        });
        <Self as IRingInfo_RingInfoWindowRingModelMethods>::ctor(this, index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringinfo/RingInfo.md")))]
#[::unity2::class(namespace = "App", name = "RingInfo")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: ringinfo :: RingInfo >)]
pub struct RingInfo {
    #[rename(name = "m_RingInfoWindows")]
    pub m_ring_info_windows:
        ::unity2::Array<crate::app::ringinfo::RingInfo_RingInfoWindowRingModel>,
    #[static_field]
    #[rename(name = "s_IsPlayAnimation")]
    pub s_is_play_animation: bool,
    #[static_field]
    #[rename(name = "LoadStartWait")]
    pub load_start_wait: i32,
    #[static_field]
    #[rename(name = "s_IsVisibleDirty")]
    pub s_is_visible_dirty: bool,
}

#[cfg(feature = "app-ringinfo")]
#[::unity2::methods]
impl RingInfo {
    #[method(name = "get_God", args = 0)]
    pub fn get_god() -> crate::app::godunit::GodUnit;

    #[method(name = "set_God", args = 1)]
    pub fn set_god(value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_GodData", args = 0)]
    pub fn get_god_data() -> crate::app::goddata::GodData;

    #[method(name = "set_GodData", args = 1)]
    pub fn set_god_data(value: crate::app::goddata::GodData) -> ();

    #[method(name = "get_Ring", args = 0)]
    pub fn get_ring() -> crate::app::ringdata::RingData;

    #[method(name = "set_Ring", args = 1)]
    pub fn set_ring(value: crate::app::ringdata::RingData) -> ();

    #[method(name = "get_Ring2", args = 0)]
    pub fn get_ring2() -> crate::app::ringdata::RingData;

    #[method(name = "set_Ring2", args = 1)]
    pub fn set_ring2(value: crate::app::ringdata::RingData) -> ();

    #[method(name = "get_s_LoadStartCount", args = 0)]
    pub fn get_s_load_start_count() -> i32;

    #[method(name = "set_s_LoadStartCount", args = 1)]
    pub fn set_s_load_start_count(value: i32) -> ();

    #[method(name = "get_s_IsLoadStart", args = 0)]
    pub fn get_s_is_load_start() -> bool;

    #[method(name = "set_s_IsLoadStart", args = 1)]
    pub fn set_s_is_load_start(value: bool) -> ();

    #[method(name = "CreateAsync", args = 1)]
    pub fn create_async(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "SetVisible", args = 3)]
    pub fn set_visible(value: bool, index: i32, is_visible_dirty: bool) -> ();

    #[method(name = "PrepareRingModel", args = 0)]
    pub fn prepare_ring_model(self) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "SetRing", args = 1)]
    pub fn set_ring_2(god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "SetRingCommon", args = 3)]
    pub fn set_ring_common(
        data: crate::app::ringdata::RingData,
        data2: crate::app::ringdata::RingData,
        is_play_animation: bool,
    ) -> ();

    #[method(name = "ResetCounter", args = 0)]
    pub fn reset_counter() -> ();

    #[method(name = "UpdateDirty", args = 0)]
    pub fn update_dirty() -> ();

    #[method(name = "SetRingImpl", args = 4)]
    pub fn set_ring_impl(
        self,
        god_unit: crate::app::godunit::GodUnit,
        ring_data: crate::app::ringdata::RingData,
        is_play_animation: bool,
        index: i32,
    ) -> ();

    #[method(name = "DeleteRingModels", args = 0)]
    pub fn delete_ring_models(self) -> ();

    #[method(name = "GetRenderTexture", args = 1)]
    pub fn get_render_texture(index: i32) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "PlayDecisionAnim", args = 0)]
    pub fn play_decision_anim() -> ();

    #[method(name = "SetAnimatorEnable", args = 1)]
    pub fn set_animator_enable(value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringinfo")]
impl RingInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRingInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringinfo/RingInfo_RingPrefabObject.md")))]
#[::unity2::class(namespace = "App", name = "RingInfo.RingPrefabObject")]
#[parent(crate::system::object::Object)]
pub struct RingInfo_RingPrefabObject {
    #[static_field]
    #[rename(name = "s_RingPrefabObjectDict")]
    pub s_ring_prefab_object_dict: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::ringinfo::RingInfo_RingPrefabObject,
    >,
    #[rename(name = "m_RingInfoWindow")]
    pub m_ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
    #[rename(name = "m_ResourceHandle")]
    pub m_resource_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_RingObject")]
    pub m_ring_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_God")]
    pub m_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_Materials")]
    pub m_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_RingData")]
    pub m_ring_data: crate::app::ringdata::RingData,
}

#[cfg(feature = "app-ringinfo")]
#[::unity2::methods]
impl RingInfo_RingPrefabObject {
    #[method(name = "get_m_IsVisibleDirty", args = 0)]
    pub fn get_m_is_visible_dirty(self) -> bool;

    #[method(name = "set_m_IsVisibleDirty", args = 1)]
    pub fn set_m_is_visible_dirty(self, value: bool) -> ();

    #[method(name = "get_IsDelete", args = 0)]
    pub fn get_is_delete(self) -> bool;

    #[method(name = "set_IsDelete", args = 1)]
    pub fn set_is_delete(self, value: bool) -> ();

    #[method(name = "CreateObject", args = 5)]
    pub fn create_object(
        ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::ringdata::RingData,
        is_play_animation: bool,
        is_visible_dirty: bool,
    ) -> crate::app::ringinfo::RingInfo_RingPrefabObject;

    #[method(name = "CreateObjectImpl", args = 3)]
    pub fn create_object_impl(
        god: crate::app::godunit::GodUnit,
        ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
        is_visible_dirty: bool,
    ) -> crate::app::ringinfo::RingInfo_RingPrefabObject;

    #[method(name = "CreateObjectImpl", args = 4)]
    pub fn create_object_impl_2(
        ring_data: crate::app::ringdata::RingData,
        ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
        is_play_animation: bool,
        is_visible_dirty: bool,
    ) -> crate::app::ringinfo::RingInfo_RingPrefabObject;

    #[method(name = "GetRingPrefabPath", args = 1)]
    pub fn get_ring_prefab_path(god: crate::app::godunit::GodUnit) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        prefab_path: ::unity2::Il2CppString,
        ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::ringdata::RingData,
        is_visible_dirty: bool,
    ) -> ();

    #[method(name = "DeleteAll", args = 0)]
    pub fn delete_all() -> ();

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = "UpdateDirtyAll", args = 0)]
    pub fn update_dirty_all() -> ();

    #[method(name = "UpdateDirty", args = 0)]
    pub fn update_dirty(self) -> ();

    #[method(name = "CreateImpl", args = 0)]
    pub fn create_impl(self) -> ();

    #[method(name = "GetGodAsciiName", args = 1)]
    pub fn get_god_ascii_name(
        ring_object: crate::unity_engine::gameobject::GameObject,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringinfo")]
impl RingInfo_RingPrefabObject {
    pub fn new(
        prefab_path: ::unity2::Il2CppString,
        ring_info_window: crate::app::ringinfo::RingInfo_RingInfoWindowRingModel,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::ringdata::RingData,
        is_visible_dirty: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingInfo_RingPrefabObject),
                ::core::stringify!(new),
            )
        });
        <Self as IRingInfo_RingPrefabObjectMethods>::ctor(
            this,
            prefab_path,
            ring_info_window,
            god,
            ring,
            is_visible_dirty,
        );
        this
    }
}
