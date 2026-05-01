
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/materialreference/MaterialReference.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MaterialReference {
    pub index: i32,
    pub font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    pub sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    pub material: crate::unity_engine::material::Material,
    pub is_default_material: bool,
    pub is_fallback_material: bool,
    pub fallback_material: crate::unity_engine::material::Material,
    pub padding: f32,
    pub reference_count: i32,
}

impl ::unity2::ClassIdentity for MaterialReference {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "MaterialReference";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MaterialReference {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-materialreference")]
#[::unity2::methods(value)]
impl MaterialReference {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        index: i32,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        material: crate::unity_engine::material::Material,
        padding: f32,
    ) -> ();

    #[method(name = "Contains", args = 2)]
    pub fn contains(
        material_references: ::unity2::Array<crate::tm_pro::materialreference::MaterialReference>,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    ) -> bool;

    #[method(name = "AddMaterialReference", args = 4)]
    pub fn add_material_reference(
        material: crate::unity_engine::material::Material,
        font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
        material_references: ::unity2::Array<crate::tm_pro::materialreference::MaterialReference>,
        material_reference_index_lookup : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < i32 , i32 >,
    ) -> i32;

    #[method(name = "AddMaterialReference", args = 4)]
    pub fn add_material_reference_2(
        material: crate::unity_engine::material::Material,
        sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
        material_references: ::unity2::Array<crate::tm_pro::materialreference::MaterialReference>,
        material_reference_index_lookup : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < i32 , i32 >,
    ) -> i32;
}
