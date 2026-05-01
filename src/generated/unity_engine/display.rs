
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/display/Display.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Display")]
#[parent(crate::system::object::Object)]
pub struct Display {
    #[rename(name = "nativeDisplay")]
    pub native_display: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "displays")]
    pub displays: ::unity2::Array<crate::unity_engine::display::Display>,
    #[static_field]
    #[rename(name = "_mainDisplay")]
    pub main_display: crate::unity_engine::display::Display,
    #[static_field]
    #[rename(name = "onDisplaysUpdated")]
    pub on_displays_updated: crate::unity_engine::display::Display_DisplaysUpdatedDelegate,
}

#[cfg(feature = "unity_engine-display")]
#[::unity2::methods]
impl Display {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, native_display: ::unity2::IntPtr) -> ();

    #[method(name = "get_renderingWidth", args = 0)]
    pub fn get_rendering_width(self) -> i32;

    #[method(name = "get_renderingHeight", args = 0)]
    pub fn get_rendering_height(self) -> i32;

    #[method(name = "get_systemWidth", args = 0)]
    pub fn get_system_width(self) -> i32;

    #[method(name = "get_systemHeight", args = 0)]
    pub fn get_system_height(self) -> i32;

    #[method(name = "get_requiresSrgbBlitToBackbuffer", args = 0)]
    pub fn get_requires_srgb_blit_to_backbuffer(self) -> bool;

    #[method(name = "RelativeMouseAt", args = 1)]
    pub fn relative_mouse_at(
        input_mouse_coordinates: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_main", args = 0)]
    pub fn get_main() -> crate::unity_engine::display::Display;

    #[method(name = "RecreateDisplayList", args = 1)]
    pub fn recreate_display_list(native_display: ::unity2::Array<::unity2::IntPtr>) -> ();

    #[method(name = "FireDisplaysUpdated", args = 0)]
    pub fn fire_displays_updated() -> ();

    #[method(name = "GetSystemExtImpl", args = 3)]
    pub fn get_system_ext_impl(native_display: ::unity2::IntPtr, w: i32, h: i32) -> ();

    #[method(name = "GetRenderingExtImpl", args = 3)]
    pub fn get_rendering_ext_impl(native_display: ::unity2::IntPtr, w: i32, h: i32) -> ();

    #[method(name = "RelativeMouseAtImpl", args = 4)]
    pub fn relative_mouse_at_impl(x: i32, y: i32, rx: i32, ry: i32) -> i32;

    #[method(name = "RequiresSrgbBlitToBackbufferImpl", args = 1)]
    pub fn requires_srgb_blit_to_backbuffer_impl(native_display: ::unity2::IntPtr) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-display")]
impl Display {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Display),
                ::core::stringify!(new),
            )
        });
        <Self as IDisplayMethods>::ctor(this);
        this
    }

    pub fn new_2(native_display: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Display),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDisplayMethods>::ctor_2(this, native_display);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/display/Display_DisplaysUpdatedDelegate.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Display.DisplaysUpdatedDelegate")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Display_DisplaysUpdatedDelegate {}

#[cfg(feature = "unity_engine-display")]
#[::unity2::methods]
impl Display_DisplaysUpdatedDelegate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-display")]
impl Display_DisplaysUpdatedDelegate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Display_DisplaysUpdatedDelegate),
                ::core::stringify!(new),
            )
        });
        <Self as IDisplay_DisplaysUpdatedDelegateMethods>::ctor(this, object, method);
        this
    }
}
