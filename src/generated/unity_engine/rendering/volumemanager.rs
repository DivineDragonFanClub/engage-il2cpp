
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volumemanager/VolumeManager.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "VolumeManager")]
#[parent(crate::system::object::Object)]
pub struct VolumeManager {
    #[static_field]
    #[rename(name = "k_MaxLayerCount")]
    pub k_max_layer_count: i32,
    #[rename(name = "m_SortedVolumes")]
    pub m_sorted_volumes: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::volume::Volume,
        >,
    >,
    #[rename(name = "m_Volumes")]
    pub m_volumes: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::volume::Volume,
    >,
    #[rename(name = "m_SortNeeded")]
    pub m_sort_needed: crate::system::collections::generic::dictionary_2::Dictionary_2<i32, bool>,
    #[rename(name = "m_ComponentsDefaultState")]
    pub m_components_default_state: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::volumecomponent::VolumeComponent,
    >,
    #[rename(name = "m_TempColliders")]
    pub m_temp_colliders: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::collider::Collider,
    >,
}

#[cfg(feature = "unity_engine-rendering-volumemanager")]
#[::unity2::methods]
impl VolumeManager {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::unity_engine::rendering::volumemanager::VolumeManager;

    #[method(name = "get_stack", args = 0)]
    pub fn get_stack(self) -> crate::unity_engine::rendering::volumestack::VolumeStack;

    #[method(name = "set_stack", args = 1)]
    pub fn set_stack(self, value: crate::unity_engine::rendering::volumestack::VolumeStack) -> ();

    #[method(name = "get_baseComponentTypes", args = 0)]
    pub fn get_base_component_types(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>;

    #[method(name = "set_baseComponentTypes", args = 1)]
    pub fn set_base_component_types(
        self,
        value: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            ::unity2::SystemType,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateStack", args = 0)]
    pub fn create_stack(self) -> crate::unity_engine::rendering::volumestack::VolumeStack;

    #[method(name = "DestroyStack", args = 1)]
    pub fn destroy_stack(
        self,
        stack: crate::unity_engine::rendering::volumestack::VolumeStack,
    ) -> ();

    #[method(name = "ReloadBaseTypes", args = 0)]
    pub fn reload_base_types(self) -> ();

    #[method(name = "Register", args = 2)]
    pub fn register(self, volume: crate::unity_engine::rendering::volume::Volume, layer: i32)
        -> ();

    #[method(name = "Unregister", args = 2)]
    pub fn unregister(
        self,
        volume: crate::unity_engine::rendering::volume::Volume,
        layer: i32,
    ) -> ();

    #[method(name = "SetLayerDirty", args = 1)]
    pub fn set_layer_dirty(self, layer: i32) -> ();

    #[method(name = "UpdateVolumeLayer", args = 3)]
    pub fn update_volume_layer(
        self,
        volume: crate::unity_engine::rendering::volume::Volume,
        prev_layer: i32,
        new_layer: i32,
    ) -> ();

    #[method(name = "OverrideData", args = 3)]
    pub fn override_data(
        self,
        stack: crate::unity_engine::rendering::volumestack::VolumeStack,
        components: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::volumecomponent::VolumeComponent,
        >,
        interp_factor: f32,
    ) -> ();

    #[method(name = "ReplaceData", args = 2)]
    pub fn replace_data(
        self,
        stack: crate::unity_engine::rendering::volumestack::VolumeStack,
        components: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::volumecomponent::VolumeComponent,
        >,
    ) -> ();

    #[method(name = "CheckBaseTypes", args = 0)]
    pub fn check_base_types(self) -> ();

    #[method(name = "CheckStack", args = 1)]
    pub fn check_stack(self, stack: crate::unity_engine::rendering::volumestack::VolumeStack)
        -> ();

    #[method(name = "Update", args = 2)]
    pub fn update(
        self,
        trigger: crate::unity_engine::transform::Transform,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> ();

    #[method(name = "Update", args = 3)]
    pub fn update_2(
        self,
        stack: crate::unity_engine::rendering::volumestack::VolumeStack,
        trigger: crate::unity_engine::transform::Transform,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> ();

    #[method(name = "UpdateByVolumeProfile", args = 1)]
    pub fn update_by_volume_profile(
        self,
        profile: crate::unity_engine::rendering::volumeprofile::VolumeProfile,
    ) -> ();

    #[method(name = "GetVolumes", args = 1)]
    pub fn get_volumes(
        self,
        layer_mask: crate::unity_engine::layermask::LayerMask,
    ) -> ::unity2::Array<crate::unity_engine::rendering::volume::Volume>;

    #[method(name = "GrabVolumes", args = 1)]
    pub fn grab_volumes(
        self,
        mask: crate::unity_engine::layermask::LayerMask,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::volume::Volume,
    >;

    #[method(name = "SortByPriority", args = 1)]
    pub fn sort_by_priority(
        volumes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::volume::Volume,
        >,
    ) -> ();

    #[method(name = "IsVolumeRenderedByCamera", args = 2)]
    pub fn is_volume_rendered_by_camera(
        volume: crate::unity_engine::rendering::volume::Volume,
        camera: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-volumemanager")]
impl VolumeManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VolumeManager),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeManagerMethods>::ctor(this);
        this
    }
}
