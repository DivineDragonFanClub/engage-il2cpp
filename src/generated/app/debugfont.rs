
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugfont/DebugFont.md")))]
#[::unity2::class(namespace = "App", name = "DebugFont")]
#[parent(crate::system::object::Object)]
pub struct DebugFont {
    #[static_field]
    #[rename(name = "s_Font")]
    pub s_font: crate::unity_engine::font::Font,
    #[static_field]
    #[rename(name = "s_NormalStyle")]
    pub s_normal_style: crate::unity_engine::guistyle::GUIStyle,
    #[static_field]
    #[rename(name = "s_MiddleStyle")]
    pub s_middle_style: crate::unity_engine::guistyle::GUIStyle,
    #[static_field]
    #[rename(name = "s_SmallStyle")]
    pub s_small_style: crate::unity_engine::guistyle::GUIStyle,
    #[static_field]
    #[rename(name = "s_CurrentStyle")]
    pub s_current_style: crate::unity_engine::guistyle::GUIStyle,
}

#[cfg(feature = "app-debugfont")]
#[::unity2::methods]
impl DebugFont {
    #[method(name = "Intialize", args = 0)]
    pub fn intialize() -> ();

    #[method(name = "CreateStyle", args = 1)]
    pub fn create_style(size: i32) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "GetFont", args = 0)]
    pub fn get_font() -> crate::unity_engine::font::Font;

    #[method(name = "get_NormalStyle", args = 0)]
    pub fn get_normal_style() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "get_MiddleStyle", args = 0)]
    pub fn get_middle_style() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "get_SmallStyle", args = 0)]
    pub fn get_small_style() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "get_CurrentStyle", args = 0)]
    pub fn get_current_style() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_CurrentStyle", args = 1)]
    pub fn set_current_style(value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_FontSize", args = 0)]
    pub fn get_font_size() -> i32;

    #[method(name = "get_LineSize", args = 0)]
    pub fn get_line_size() -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-debugfont")]
impl DebugFont {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugFont),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugFontMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugfont/DebugFont_Scope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DebugFont_Scope {
    pub m_style: crate::unity_engine::guistyle::GUIStyle,
}

impl ::unity2::ClassIdentity for DebugFont_Scope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DebugFont.Scope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugFont_Scope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-debugfont")]
#[::unity2::methods(value)]
impl DebugFont_Scope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, style: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}
