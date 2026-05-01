
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/texturecurve/TextureCurve.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "TextureCurve")]
#[parent(crate::system::object::Object)]
pub struct TextureCurve {
    #[static_field]
    #[rename(name = "k_Precision")]
    pub k_precision: i32,
    #[static_field]
    #[rename(name = "k_Step")]
    pub k_step: f32,
    #[rename(name = "m_Loop")]
    pub m_loop: bool,
    #[rename(name = "m_ZeroValue")]
    pub m_zero_value: f32,
    #[rename(name = "m_Range")]
    pub m_range: f32,
    #[rename(name = "m_Curve")]
    pub m_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_LoopingCurve")]
    pub m_looping_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_Texture")]
    pub m_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_IsCurveDirty")]
    pub m_is_curve_dirty: bool,
    #[rename(name = "m_IsTextureDirty")]
    pub m_is_texture_dirty: bool,
}

#[cfg(feature = "unity_engine-rendering-texturecurve")]
#[::unity2::methods]
impl TextureCurve {
    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "set_length", args = 1)]
    pub fn set_length(self, value: i32) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::unity_engine::keyframe::Keyframe;

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        base_curve: crate::unity_engine::animationcurve::AnimationCurve,
        zero_value: f32,
        r#loop: bool,
        bounds: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>,
        zero_value: f32,
        r#loop: bool,
        bounds: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty(self) -> ();

    #[method(name = "GetTextureFormat", args = 0)]
    pub fn get_texture_format() -> crate::unity_engine::textureformat::TextureFormat;

    #[method(name = "GetTexture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate(self, time: f32) -> f32;

    #[method(name = "AddKey", args = 2)]
    pub fn add_key(self, time: f32, value: f32) -> i32;

    #[method(name = "MoveKey", args = 2)]
    pub fn move_key(self, index: i32, key: crate::unity_engine::keyframe::Keyframe) -> i32;

    #[method(name = "RemoveKey", args = 1)]
    pub fn remove_key(self, index: i32) -> ();

    #[method(name = "SmoothTangents", args = 2)]
    pub fn smooth_tangents(self, index: i32, weight: f32) -> ();
}

#[cfg(feature = "unity_engine-rendering-texturecurve")]
impl TextureCurve {
    pub fn new(
        base_curve: crate::unity_engine::animationcurve::AnimationCurve,
        zero_value: f32,
        r#loop: bool,
        bounds: crate::unity_engine::vector2::Vector2,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextureCurve),
                ::core::stringify!(new),
            )
        });
        <Self as ITextureCurveMethods>::ctor(this, base_curve, zero_value, r#loop, bounds);
        this
    }

    pub fn new_2(
        keys: ::unity2::Array<crate::unity_engine::keyframe::Keyframe>,
        zero_value: f32,
        r#loop: bool,
        bounds: crate::unity_engine::vector2::Vector2,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextureCurve),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITextureCurveMethods>::ctor_2(this, keys, zero_value, r#loop, bounds);
        this
    }
}
