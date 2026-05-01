
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/aspectratiofitter/AspectRatioFitter_AspectMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AspectRatioFitter_AspectMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AspectRatioFitter_AspectMode {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "AspectRatioFitter.AspectMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AspectRatioFitter_AspectMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AspectRatioFitter_AspectMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn width_controls_height() -> Self {
        Self { value: 1 }
    }

    pub fn height_controls_width() -> Self {
        Self { value: 2 }
    }

    pub fn fit_in_parent() -> Self {
        Self { value: 3 }
    }

    pub fn envelope_parent() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/aspectratiofitter/AspectRatioFitter.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "AspectRatioFitter")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct AspectRatioFitter {
    #[rename(name = "m_AspectMode")]
    pub m_aspect_mode: crate::unity_engine::ui::aspectratiofitter::AspectRatioFitter_AspectMode,
    #[rename(name = "m_AspectRatio")]
    pub m_aspect_ratio: f32,
    #[rename(name = "m_Rect")]
    pub m_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_DelayedSetDirty")]
    pub m_delayed_set_dirty: bool,
    #[rename(name = "m_DoesParentExist")]
    pub m_does_parent_exist: bool,
    #[rename(name = "m_Tracker")]
    pub m_tracker: crate::unity_engine::drivenrecttransformtracker::DrivenRectTransformTracker,
}

#[cfg(feature = "unity_engine-ui-aspectratiofitter")]
#[::unity2::methods]
impl AspectRatioFitter {
    #[method(name = "get_aspectMode", args = 0)]
    pub fn get_aspect_mode(
        self,
    ) -> crate::unity_engine::ui::aspectratiofitter::AspectRatioFitter_AspectMode;

    #[method(name = "set_aspectMode", args = 1)]
    pub fn set_aspect_mode(
        self,
        value: crate::unity_engine::ui::aspectratiofitter::AspectRatioFitter_AspectMode,
    ) -> ();

    #[method(name = "get_aspectRatio", args = 0)]
    pub fn get_aspect_ratio(self) -> f32;

    #[method(name = "set_aspectRatio", args = 1)]
    pub fn set_aspect_ratio(self, value: f32) -> ();

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "UpdateRect", args = 0)]
    pub fn update_rect(self) -> ();

    #[method(name = "GetSizeDeltaToProduceSize", args = 2)]
    pub fn get_size_delta_to_produce_size(self, size: f32, axis: i32) -> f32;

    #[method(name = "GetParentSize", args = 0)]
    pub fn get_parent_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SetLayoutHorizontal", args = 0)]
    pub fn set_layout_horizontal(self) -> ();

    #[method(name = "SetLayoutVertical", args = 0)]
    pub fn set_layout_vertical(self) -> ();

    #[method(name = "SetDirty", args = 0)]
    pub fn set_dirty(self) -> ();

    #[method(name = "IsComponentValidOnObject", args = 0)]
    pub fn is_component_valid_on_object(self) -> bool;

    #[method(name = "IsAspectModeValid", args = 0)]
    pub fn is_aspect_mode_valid(self) -> bool;

    #[method(name = "DoesParentExists", args = 0)]
    pub fn does_parent_exists(self) -> bool;
}

#[cfg(feature = "unity_engine-ui-aspectratiofitter")]
impl AspectRatioFitter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AspectRatioFitter),
                ::core::stringify!(new),
            )
        });
        <Self as IAspectRatioFitterMethods>::ctor(this);
        this
    }
}
