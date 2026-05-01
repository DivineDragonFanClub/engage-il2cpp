
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_updatemanager/TMP_UpdateManager.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_UpdateManager")]
#[parent(crate::system::object::Object)]
pub struct TMP_UpdateManager {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::tm_pro::tmp_updatemanager::TMP_UpdateManager,
    #[rename(name = "m_LayoutQueueLookup")]
    pub m_layout_queue_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_LayoutRebuildQueue")]
    pub m_layout_rebuild_queue:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_text::TMP_Text>,
    #[rename(name = "m_GraphicQueueLookup")]
    pub m_graphic_queue_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_GraphicRebuildQueue")]
    pub m_graphic_rebuild_queue:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_text::TMP_Text>,
    #[rename(name = "m_InternalUpdateLookup")]
    pub m_internal_update_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_InternalUpdateQueue")]
    pub m_internal_update_queue:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_text::TMP_Text>,
    #[rename(name = "m_CullingUpdateLookup")]
    pub m_culling_update_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_CullingUpdateQueue")]
    pub m_culling_update_queue:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_text::TMP_Text>,
}

#[cfg(feature = "tm_pro-tmp_updatemanager")]
#[::unity2::methods]
impl TMP_UpdateManager {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::tm_pro::tmp_updatemanager::TMP_UpdateManager;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "RegisterTextObjectForUpdate", args = 1)]
    pub fn register_text_object_for_update(text_object: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "InternalRegisterTextObjectForUpdate", args = 1)]
    pub fn internal_register_text_object_for_update(
        self,
        text_object: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "RegisterTextElementForLayoutRebuild", args = 1)]
    pub fn register_text_element_for_layout_rebuild(
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "InternalRegisterTextElementForLayoutRebuild", args = 1)]
    pub fn internal_register_text_element_for_layout_rebuild(
        self,
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "RegisterTextElementForGraphicRebuild", args = 1)]
    pub fn register_text_element_for_graphic_rebuild(
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "InternalRegisterTextElementForGraphicRebuild", args = 1)]
    pub fn internal_register_text_element_for_graphic_rebuild(
        self,
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "RegisterTextElementForCullingUpdate", args = 1)]
    pub fn register_text_element_for_culling_update(
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "InternalRegisterTextElementForCullingUpdate", args = 1)]
    pub fn internal_register_text_element_for_culling_update(
        self,
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "OnCameraPreCull", args = 0)]
    pub fn on_camera_pre_cull(self) -> ();

    #[method(name = "DoRebuilds", args = 0)]
    pub fn do_rebuilds(self) -> ();

    #[method(name = "UnRegisterTextObjectForUpdate", args = 1)]
    pub fn un_register_text_object_for_update(text_object: crate::tm_pro::tmp_text::TMP_Text)
        -> ();

    #[method(name = "UnRegisterTextElementForRebuild", args = 1)]
    pub fn un_register_text_element_for_rebuild(element: crate::tm_pro::tmp_text::TMP_Text) -> ();

    #[method(name = "InternalUnRegisterTextElementForGraphicRebuild", args = 1)]
    pub fn internal_un_register_text_element_for_graphic_rebuild(
        self,
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "InternalUnRegisterTextElementForLayoutRebuild", args = 1)]
    pub fn internal_un_register_text_element_for_layout_rebuild(
        self,
        element: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = "InternalUnRegisterTextObjectForUpdate", args = 1)]
    pub fn internal_un_register_text_object_for_update(
        self,
        text_object: crate::tm_pro::tmp_text::TMP_Text,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-tmp_updatemanager")]
impl TMP_UpdateManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_UpdateManager),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_UpdateManagerMethods>::ctor(this);
        this
    }
}
