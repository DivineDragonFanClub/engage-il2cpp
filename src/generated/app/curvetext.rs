
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/curvetext/CurveText.md")))]
#[::unity2::class(namespace = "App", name = "CurveText")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CurveText {
    #[rename(name = "m_TextComponent")]
    pub m_text_component: crate::tm_pro::tmp_text::TMP_Text,
    #[rename(name = "VertexCurve")]
    pub vertex_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_CurveScale")]
    pub m_curve_scale: f32,
    #[rename(name = "m_IsRotate")]
    pub m_is_rotate: bool,
    #[rename(name = "m_UseRectWidth")]
    pub m_use_rect_width: bool,
    #[rename(name = "m_IsDistort")]
    pub m_is_distort: bool,
}

#[cfg(feature = "app-curvetext")]
#[::unity2::methods]
impl CurveText {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "CopyAnimationCurve", args = 1)]
    pub fn copy_animation_curve(
        self,
        curve: crate::unity_engine::animationcurve::AnimationCurve,
    ) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "UpdateTextTransform", args = 0)]
    pub fn update_text_transform(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-curvetext")]
impl CurveText {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CurveText),
                ::core::stringify!(new),
            )
        });
        <Self as ICurveTextMethods>::ctor(this);
        this
    }
}
