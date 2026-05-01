
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/drawobjectcategoryinfo/DrawObjectCategoryInfo.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom",
    name = "DrawObjectCategoryInfo"
)]
#[parent(crate::system::object::Object)]
pub struct DrawObjectCategoryInfo {
    #[static_field]
    #[rename(name = "s_DrawObjectVisibilities")]
    pub s_draw_object_visibilities: ::unity2::Array<bool>,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-drawobjectcategoryinfo")]
#[::unity2::methods]
impl DrawObjectCategoryInfo {
    #[method(name = "GetNoOverrideStateIndexBegin", args = 0)]
    pub fn get_no_override_state_index_begin() -> i32;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "IsVisible", args = 1)]
    pub fn is_visible(
        category : crate :: unity_engine :: rendering :: universal :: custom :: drawobjectcategory :: DrawObjectCategory,
    ) -> bool;

    #[method(name = "SetVisibility", args = 2)]
    pub fn set_visibility(
        category : crate :: unity_engine :: rendering :: universal :: custom :: drawobjectcategory :: DrawObjectCategory,
        visibility: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-drawobjectcategoryinfo")]
impl DrawObjectCategoryInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DrawObjectCategoryInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IDrawObjectCategoryInfoMethods>::ctor(this);
        this
    }
}
