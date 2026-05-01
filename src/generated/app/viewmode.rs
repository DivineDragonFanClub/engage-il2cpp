
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/viewmode/ViewMode.md")))]
#[::unity2::class(namespace = "App", name = "ViewMode")]
#[parent(crate::system::object::Object)]
pub struct ViewMode {
    #[static_field]
    #[rename(name = "s_Transition")]
    pub s_transition: f32,
    #[static_field]
    #[rename(name = "s_Mode")]
    pub s_mode: crate::app::viewmode::ViewMode_Mode,
    #[static_field]
    #[rename(name = "s_Stack")]
    pub s_stack:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::viewmode::ViewMode_Mode>,
    #[static_field]
    #[rename(name = "s_BmapSkinQuality")]
    pub s_bmap_skin_quality: crate::unity_engine::skinquality::SkinQuality,
    #[static_field]
    #[rename(name = "m_ModeThreshold")]
    pub m_mode_threshold: crate::app::gameparam::GameParam_Holder,
}

#[cfg(feature = "app-viewmode")]
#[::unity2::methods]
impl ViewMode {
    #[method(name = "get_ModeThreshold", args = 0)]
    pub fn get_mode_threshold() -> f32;

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode() -> crate::app::viewmode::ViewMode_Mode;

    #[method(name = "GetBmapSkinQuality", args = 0)]
    pub fn get_bmap_skin_quality() -> crate::unity_engine::skinquality::SkinQuality;

    #[method(name = "GetBmapAlpha", args = 0)]
    pub fn get_bmap_alpha() -> f32;

    #[method(name = "IsBmapShowing", args = 0)]
    pub fn is_bmap_showing() -> bool;

    #[method(name = "GetTransition", args = 0)]
    pub fn get_transition() -> f32;

    #[method(name = "SetMode", args = 1)]
    pub fn set_mode(mode: crate::app::viewmode::ViewMode_Mode) -> ();

    #[method(name = "SetMode", args = 2)]
    pub fn set_mode_2(
        camera: crate::unity_engine::camera::Camera,
        mode: crate::app::viewmode::ViewMode_Mode,
    ) -> ();

    #[method(name = "PushMode", args = 1)]
    pub fn push_mode(mode: crate::app::viewmode::ViewMode_Mode) -> ();

    #[method(name = "PopMode", args = 0)]
    pub fn pop_mode() -> ();

    #[method(name = "SetTransition", args = 1)]
    pub fn set_transition(transition: f32) -> ();

    #[method(name = "SetLayerCullingMask", args = 3)]
    pub fn set_layer_culling_mask(
        camera: crate::unity_engine::camera::Camera,
        name: ::unity2::Il2CppString,
        enable: bool,
    ) -> ();

    #[method(name = "SetTransition", args = 2)]
    pub fn set_transition_2(camera: crate::unity_engine::camera::Camera, transition: f32) -> ();

    #[method(name = "UpdateBoostMode", args = 1)]
    pub fn update_boost_mode(is_boost: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-viewmode")]
impl ViewMode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ViewMode),
                ::core::stringify!(new),
            )
        });
        <Self as IViewModeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/viewmode/ViewMode_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ViewMode_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ViewMode_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ViewMode.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ViewMode_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ViewMode_Mode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn bmap() -> Self {
        Self { value: 1 }
    }

    pub fn combat() -> Self {
        Self { value: 2 }
    }

    pub fn kizuna() -> Self {
        Self { value: 3 }
    }

    pub fn hub() -> Self {
        Self { value: 4 }
    }

    pub fn ride() -> Self {
        Self { value: 5 }
    }

    pub fn gmap() -> Self {
        Self { value: 6 }
    }

    pub fn demo() -> Self {
        Self { value: 7 }
    }
}
