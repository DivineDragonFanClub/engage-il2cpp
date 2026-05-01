
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guistylestate/GUIStyleState.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIStyleState")]
#[parent(crate::system::object::Object)]
pub struct GUIStyleState {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_SourceStyle")]
    pub m_source_style: crate::unity_engine::guistyle::GUIStyle,
}

#[cfg(feature = "unity_engine-guistylestate")]
#[::unity2::methods]
impl GUIStyleState {
    #[method(name = "set_textColor", args = 1)]
    pub fn set_text_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init() -> ::unity2::IntPtr;

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        source_style: crate::unity_engine::guistyle::GUIStyle,
        source: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "GetGUIStyleState", args = 2)]
    pub fn get_gui_style_state(
        source_style: crate::unity_engine::guistyle::GUIStyle,
        source: ::unity2::IntPtr,
    ) -> crate::unity_engine::guistylestate::GUIStyleState;

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "set_textColor_Injected", args = 1)]
    pub fn set_text_color_injected(self, value: crate::unity_engine::color::Color) -> ();
}

#[cfg(feature = "unity_engine-guistylestate")]
impl GUIStyleState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIStyleState),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIStyleStateMethods>::ctor(this);
        this
    }

    pub fn new_2(
        source_style: crate::unity_engine::guistyle::GUIStyle,
        source: ::unity2::IntPtr,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIStyleState),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGUIStyleStateMethods>::ctor_2(this, source_style, source);
        this
    }
}
