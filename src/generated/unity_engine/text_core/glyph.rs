
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/text_core/glyph/Glyph.md")))]
#[::unity2::class(namespace = "UnityEngine.TextCore", name = "Glyph")]
#[parent(crate::system::object::Object)]
pub struct Glyph {
    #[rename(name = "m_Index")]
    pub m_index: u32,
    #[rename(name = "m_Metrics")]
    pub m_metrics: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
    #[rename(name = "m_GlyphRect")]
    pub m_glyph_rect: crate::unity_engine::text_core::glyphrect::GlyphRect,
    #[rename(name = "m_Scale")]
    pub m_scale: f32,
    #[rename(name = "m_AtlasIndex")]
    pub m_atlas_index: i32,
}

#[cfg(feature = "unity_engine-text_core-glyph")]
#[::unity2::methods]
impl Glyph {
    #[method(name = "get_index", args = 0)]
    pub fn get_index(self) -> u32;

    #[method(name = "set_index", args = 1)]
    pub fn set_index(self, value: u32) -> ();

    #[method(name = "get_metrics", args = 0)]
    pub fn get_metrics(self) -> crate::unity_engine::text_core::glyphmetrics::GlyphMetrics;

    #[method(name = "set_metrics", args = 1)]
    pub fn set_metrics(
        self,
        value: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
    ) -> ();

    #[method(name = "get_glyphRect", args = 0)]
    pub fn get_glyph_rect(self) -> crate::unity_engine::text_core::glyphrect::GlyphRect;

    #[method(name = "set_glyphRect", args = 1)]
    pub fn set_glyph_rect(self, value: crate::unity_engine::text_core::glyphrect::GlyphRect) -> ();

    #[method(name = "get_scale", args = 0)]
    pub fn get_scale(self) -> f32;

    #[method(name = "set_scale", args = 1)]
    pub fn set_scale(self, value: f32) -> ();

    #[method(name = "get_atlasIndex", args = 0)]
    pub fn get_atlas_index(self) -> i32;

    #[method(name = "set_atlasIndex", args = 1)]
    pub fn set_atlas_index(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        glyph_struct : crate :: unity_engine :: text_core :: low_level :: glyphmarshallingstruct :: GlyphMarshallingStruct,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_3(
        self,
        index: u32,
        metrics: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
        glyph_rect: crate::unity_engine::text_core::glyphrect::GlyphRect,
        scale: f32,
        atlas_index: i32,
    ) -> ();
}

#[cfg(feature = "unity_engine-text_core-glyph")]
impl Glyph {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Glyph),
                ::core::stringify!(new),
            )
        });
        <Self as IGlyphMethods>::ctor(this);
        this
    }

    pub fn new_2(
        glyph_struct : crate :: unity_engine :: text_core :: low_level :: glyphmarshallingstruct :: GlyphMarshallingStruct,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Glyph),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGlyphMethods>::ctor_2(this, glyph_struct);
        this
    }

    pub fn new_3(
        index: u32,
        metrics: crate::unity_engine::text_core::glyphmetrics::GlyphMetrics,
        glyph_rect: crate::unity_engine::text_core::glyphrect::GlyphRect,
        scale: f32,
        atlas_index: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Glyph),
                ::core::stringify!(new_3),
            )
        });
        <Self as IGlyphMethods>::ctor_3(this, index, metrics, glyph_rect, scale, atlas_index);
        this
    }
}
