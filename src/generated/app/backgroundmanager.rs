
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/backgroundmanager/BackgroundManager.md")))]
#[::unity2::class(namespace = "App", name = "BackgroundManager")]
#[parent(crate::system::object::Object)]
pub struct BackgroundManager {
    #[static_field]
    #[rename(name = "s_Binder")]
    pub s_binder: crate::app::bindholder::BindHolder,
    #[static_field]
    #[rename(name = "s_Camera")]
    pub s_camera: crate::unity_engine::camera::Camera,
    #[static_field]
    #[rename(name = "s_IsCaptured")]
    pub s_is_captured: bool,
    #[static_field]
    #[rename(name = "s_CaptureCount")]
    pub s_capture_count: i32,
}

#[cfg(feature = "app-backgroundmanager")]
#[::unity2::methods]
impl BackgroundManager {
    #[method(name = "SetCamera", args = 2)]
    pub fn set_camera(camera: crate::unity_engine::camera::Camera, enable: bool) -> ();

    #[method(name = "SetCapture", args = 2)]
    pub fn set_capture(camera: crate::unity_engine::camera::Camera, enable: bool) -> ();

    #[method(name = "SetWallPaper", args = 1)]
    pub fn set_wall_paper(enable: bool) -> ();

    #[method(name = "SetBlur", args = 1)]
    pub fn set_blur(enable: bool) -> ();

    #[method(name = "SetCapture", args = 1)]
    pub fn set_capture_2(enable: bool) -> ();

    #[method(name = "GetBackground", args = 0)]
    pub fn get_background() -> i32;

    #[method(name = "Bind", args = 1)]
    pub fn bind(r#type: crate::app::backgroundmanager::BackgroundManager_BindType) -> ();

    #[method(name = "Bind", args = 0)]
    pub fn bind_2() -> ();

    #[method(name = "Update", args = 0)]
    pub fn update() -> ();

    #[method(name = "Unbind", args = 0)]
    pub fn unbind() -> ();

    #[method(name = "IsBind", args = 0)]
    pub fn is_bind() -> bool;

    #[method(name = "IsCaptured", args = 0)]
    pub fn is_captured() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-backgroundmanager")]
impl BackgroundManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BackgroundManager),
                ::core::stringify!(new),
            )
        });
        <Self as IBackgroundManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/backgroundmanager/BackgroundManager_BindType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BackgroundManager_BindType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BackgroundManager_BindType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BackgroundManager.BindType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BackgroundManager_BindType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BackgroundManager_BindType {
    pub fn take_new_capture() -> Self {
        Self { value: 0 }
    }

    pub fn use_prev_capture() -> Self {
        Self { value: 1 }
    }
}
