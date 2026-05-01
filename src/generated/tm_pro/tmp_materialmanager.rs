
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_materialmanager/TMP_MaterialManager_MaskingMaterial.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_MaterialManager.MaskingMaterial")]
#[parent(crate::system::object::Object)]
pub struct TMP_MaterialManager_MaskingMaterial {
    #[rename(name = "baseMaterial")]
    pub base_material: crate::unity_engine::material::Material,
    #[rename(name = "stencilMaterial")]
    pub stencil_material: crate::unity_engine::material::Material,
    #[rename(name = "count")]
    pub count: i32,
    #[rename(name = "stencilID")]
    pub stencil_id: i32,
}

#[cfg(feature = "tm_pro-tmp_materialmanager")]
#[::unity2::methods]
impl TMP_MaterialManager_MaskingMaterial {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_materialmanager")]
impl TMP_MaterialManager_MaskingMaterial {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_MaterialManager_MaskingMaterial),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_MaterialManager_MaskingMaterialMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_materialmanager/TMP_MaterialManager.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_MaterialManager")]
#[parent(crate::system::object::Object)]
pub struct TMP_MaterialManager {
    #[static_field]
    #[rename(name = "m_materialList")]
    pub m_material_list: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_materialmanager::TMP_MaterialManager_MaskingMaterial,
    >,
    #[static_field]
    #[rename(name = "m_fallbackMaterials")]
    pub m_fallback_materials: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i64,
        crate::tm_pro::tmp_materialmanager::TMP_MaterialManager_FallbackMaterial,
    >,
    #[static_field]
    #[rename(name = "m_fallbackMaterialLookup")]
    pub m_fallback_material_lookup:
        crate::system::collections::generic::dictionary_2::Dictionary_2<i32, i64>,
    #[static_field]
    #[rename(name = "m_fallbackCleanupList")]
    pub m_fallback_cleanup_list: crate::system::collections::generic::list_1::List_1<
        crate::tm_pro::tmp_materialmanager::TMP_MaterialManager_FallbackMaterial,
    >,
    #[static_field]
    #[rename(name = "isFallbackListDirty")]
    pub is_fallback_list_dirty: bool,
}

#[cfg(feature = "tm_pro-tmp_materialmanager")]
#[::unity2::methods]
impl TMP_MaterialManager {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "OnPreRender", args = 0)]
    pub fn on_pre_render() -> ();

    #[method(name = "GetStencilMaterial", args = 2)]
    pub fn get_stencil_material(
        base_material: crate::unity_engine::material::Material,
        stencil_id: i32,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "ReleaseStencilMaterial", args = 1)]
    pub fn release_stencil_material(
        stencil_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "GetBaseMaterial", args = 1)]
    pub fn get_base_material(
        stencil_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "SetStencil", args = 2)]
    pub fn set_stencil(
        material: crate::unity_engine::material::Material,
        stencil_id: i32,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "AddMaskingMaterial", args = 3)]
    pub fn add_masking_material(
        base_material: crate::unity_engine::material::Material,
        stencil_material: crate::unity_engine::material::Material,
        stencil_id: i32,
    ) -> ();

    #[method(name = "RemoveStencilMaterial", args = 1)]
    pub fn remove_stencil_material(stencil_material: crate::unity_engine::material::Material)
        -> ();

    #[method(name = "ReleaseBaseMaterial", args = 1)]
    pub fn release_base_material(base_material: crate::unity_engine::material::Material) -> ();

    #[method(name = "ClearMaterials", args = 0)]
    pub fn clear_materials() -> ();

    #[method(name = "GetStencilID", args = 1)]
    pub fn get_stencil_id(obj: crate::unity_engine::gameobject::GameObject) -> i32;

    #[method(name = "GetMaterialForRendering", args = 2)]
    pub fn get_material_for_rendering(
        graphic: crate::unity_engine::ui::maskablegraphic::MaskableGraphic,
        base_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "FindRootSortOverrideCanvas", args = 1)]
    pub fn find_root_sort_override_canvas(
        start: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetFallbackMaterial", args = 3)]
    pub fn get_fallback_material(
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        source_material: crate::unity_engine::material::Material,
        atlas_index: i32,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "GetFallbackMaterial", args = 2)]
    pub fn get_fallback_material_2(
        source_material: crate::unity_engine::material::Material,
        target_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "AddFallbackMaterialReference", args = 1)]
    pub fn add_fallback_material_reference(
        target_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "RemoveFallbackMaterialReference", args = 1)]
    pub fn remove_fallback_material_reference(
        target_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "CleanupFallbackMaterials", args = 0)]
    pub fn cleanup_fallback_materials() -> ();

    #[method(name = "ReleaseFallbackMaterial", args = 1)]
    pub fn release_fallback_material(
        fallback_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "CopyMaterialPresetProperties", args = 2)]
    pub fn copy_material_preset_properties(
        source: crate::unity_engine::material::Material,
        destination: crate::unity_engine::material::Material,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_materialmanager/TMP_MaterialManager_FallbackMaterial.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_MaterialManager.FallbackMaterial")]
#[parent(crate::system::object::Object)]
pub struct TMP_MaterialManager_FallbackMaterial {
    #[rename(name = "fallbackID")]
    pub fallback_id: i64,
    #[rename(name = "sourceMaterial")]
    pub source_material: crate::unity_engine::material::Material,
    #[rename(name = "sourceMaterialCRC")]
    pub source_material_crc: i32,
    #[rename(name = "fallbackMaterial")]
    pub fallback_material: crate::unity_engine::material::Material,
    #[rename(name = "count")]
    pub count: i32,
}

#[cfg(feature = "tm_pro-tmp_materialmanager")]
#[::unity2::methods]
impl TMP_MaterialManager_FallbackMaterial {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_materialmanager")]
impl TMP_MaterialManager_FallbackMaterial {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_MaterialManager_FallbackMaterial),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_MaterialManager_FallbackMaterialMethods>::ctor(this);
        this
    }
}
