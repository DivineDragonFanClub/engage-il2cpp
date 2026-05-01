
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_updateregistry/TMP_UpdateRegistry.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_UpdateRegistry")]
#[parent(crate::system::object::Object)]
pub struct TMP_UpdateRegistry {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::tm_pro::tmp_updateregistry::TMP_UpdateRegistry,
    #[rename(name = "m_LayoutRebuildQueue")]
    pub m_layout_rebuild_queue: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::ui::icanvaselement::ICanvasElement,
    >,
    #[rename(name = "m_LayoutQueueLookup")]
    pub m_layout_queue_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[rename(name = "m_GraphicRebuildQueue")]
    pub m_graphic_rebuild_queue: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::ui::icanvaselement::ICanvasElement,
    >,
    #[rename(name = "m_GraphicQueueLookup")]
    pub m_graphic_queue_lookup: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
}

#[cfg(feature = "tm_pro-tmp_updateregistry")]
#[::unity2::methods]
impl TMP_UpdateRegistry {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::tm_pro::tmp_updateregistry::TMP_UpdateRegistry;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "RegisterCanvasElementForLayoutRebuild", args = 1)]
    pub fn register_canvas_element_for_layout_rebuild(
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> ();

    #[method(name = "InternalRegisterCanvasElementForLayoutRebuild", args = 1)]
    pub fn internal_register_canvas_element_for_layout_rebuild(
        self,
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> bool;

    #[method(name = "RegisterCanvasElementForGraphicRebuild", args = 1)]
    pub fn register_canvas_element_for_graphic_rebuild(
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> ();

    #[method(name = "InternalRegisterCanvasElementForGraphicRebuild", args = 1)]
    pub fn internal_register_canvas_element_for_graphic_rebuild(
        self,
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> bool;

    #[method(name = "PerformUpdateForCanvasRendererObjects", args = 0)]
    pub fn perform_update_for_canvas_renderer_objects(self) -> ();

    #[method(name = "PerformUpdateForMeshRendererObjects", args = 0)]
    pub fn perform_update_for_mesh_renderer_objects(self) -> ();

    #[method(name = "UnRegisterCanvasElementForRebuild", args = 1)]
    pub fn un_register_canvas_element_for_rebuild(
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> ();

    #[method(name = "InternalUnRegisterCanvasElementForLayoutRebuild", args = 1)]
    pub fn internal_un_register_canvas_element_for_layout_rebuild(
        self,
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> ();

    #[method(name = "InternalUnRegisterCanvasElementForGraphicRebuild", args = 1)]
    pub fn internal_un_register_canvas_element_for_graphic_rebuild(
        self,
        element: crate::unity_engine::ui::icanvaselement::ICanvasElement,
    ) -> ();
}

#[cfg(feature = "tm_pro-tmp_updateregistry")]
impl TMP_UpdateRegistry {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_UpdateRegistry),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_UpdateRegistryMethods>::ctor(this);
        this
    }
}
