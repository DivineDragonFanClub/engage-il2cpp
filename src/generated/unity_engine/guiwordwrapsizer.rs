
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::guilayoutentry::GUILayoutEntry;
use crate::unity_engine::guilayoutentry::IGUILayoutEntry;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiwordwrapsizer/GUIWordWrapSizer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIWordWrapSizer")]
#[parent(crate::unity_engine::guilayoutentry::GUILayoutEntry)]
pub struct GUIWordWrapSizer {
    #[rename(name = "m_Content")]
    pub m_content: crate::unity_engine::guicontent::GUIContent,
    #[rename(name = "m_ForcedMinHeight")]
    pub m_forced_min_height: f32,
    #[rename(name = "m_ForcedMaxHeight")]
    pub m_forced_max_height: f32,
}

#[cfg(feature = "unity_engine-guiwordwrapsizer")]
#[::unity2::methods]
impl GUIWordWrapSizer {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        style: crate::unity_engine::guistyle::GUIStyle,
        content: crate::unity_engine::guicontent::GUIContent,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "CalcWidth", args = 0)]
    pub fn calc_width(self) -> ();

    #[method(name = "CalcHeight", args = 0)]
    pub fn calc_height(self) -> ();
}

#[cfg(feature = "unity_engine-guiwordwrapsizer")]
impl GUIWordWrapSizer {
    pub fn new(
        style: crate::unity_engine::guistyle::GUIStyle,
        content: crate::unity_engine::guicontent::GUIContent,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIWordWrapSizer),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIWordWrapSizerMethods>::ctor(this, style, content, options);
        this
    }
}
