
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
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::selectable::ISelectable;
use crate::unity_engine::ui::selectable::Selectable;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/slider/Slider.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Slider")]
#[parent(crate::unity_engine::ui::selectable::Selectable)]
pub struct Slider {
    #[rename(name = "m_FillRect")]
    pub m_fill_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_HandleRect")]
    pub m_handle_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Direction")]
    pub m_direction: crate::unity_engine::ui::slider::Slider_Direction,
    #[rename(name = "m_MinValue")]
    pub m_min_value: f32,
    #[rename(name = "m_MaxValue")]
    pub m_max_value: f32,
    #[rename(name = "m_WholeNumbers")]
    pub m_whole_numbers: bool,
    #[rename(name = "m_Value")]
    pub m_value: f32,
    #[rename(name = "m_OnValueChanged")]
    pub m_on_value_changed: crate::unity_engine::ui::slider::Slider_SliderEvent,
    #[rename(name = "m_FillImage")]
    pub m_fill_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FillTransform")]
    pub m_fill_transform: crate::unity_engine::transform::Transform,
    #[rename(name = "m_FillContainerRect")]
    pub m_fill_container_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_HandleTransform")]
    pub m_handle_transform: crate::unity_engine::transform::Transform,
    #[rename(name = "m_HandleContainerRect")]
    pub m_handle_container_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Offset")]
    pub m_offset: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Tracker")]
    pub m_tracker: crate::unity_engine::drivenrecttransformtracker::DrivenRectTransformTracker,
    #[rename(name = "m_DelayedUpdateVisuals")]
    pub m_delayed_update_visuals: bool,
}

#[cfg(feature = "unity_engine-ui-slider")]
#[::unity2::methods]
impl Slider {
    #[method(name = "get_fillRect", args = 0)]
    pub fn get_fill_rect(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "set_fillRect", args = 1)]
    pub fn set_fill_rect(self, value: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "get_handleRect", args = 0)]
    pub fn get_handle_rect(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "set_handleRect", args = 1)]
    pub fn set_handle_rect(self, value: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "get_direction", args = 0)]
    pub fn get_direction(self) -> crate::unity_engine::ui::slider::Slider_Direction;

    #[method(name = "set_direction", args = 1)]
    pub fn set_direction(self, value: crate::unity_engine::ui::slider::Slider_Direction) -> ();

    #[method(name = "get_minValue", args = 0)]
    pub fn get_min_value(self) -> f32;

    #[method(name = "set_minValue", args = 1)]
    pub fn set_min_value(self, value: f32) -> ();

    #[method(name = "get_maxValue", args = 0)]
    pub fn get_max_value(self) -> f32;

    #[method(name = "set_maxValue", args = 1)]
    pub fn set_max_value(self, value: f32) -> ();

    #[method(name = "get_wholeNumbers", args = 0)]
    pub fn get_whole_numbers(self) -> bool;

    #[method(name = "set_wholeNumbers", args = 1)]
    pub fn set_whole_numbers(self, value: bool) -> ();

    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: f32) -> ();

    #[method(name = "SetValueWithoutNotify", args = 1)]
    pub fn set_value_without_notify(self, input: f32) -> ();

    #[method(name = "get_normalizedValue", args = 0)]
    pub fn get_normalized_value(self) -> f32;

    #[method(name = "set_normalizedValue", args = 1)]
    pub fn set_normalized_value(self, value: f32) -> ();

    #[method(name = "get_onValueChanged", args = 0)]
    pub fn get_on_value_changed(self) -> crate::unity_engine::ui::slider::Slider_SliderEvent;

    #[method(name = "set_onValueChanged", args = 1)]
    pub fn set_on_value_changed(
        self,
        value: crate::unity_engine::ui::slider::Slider_SliderEvent,
    ) -> ();

    #[method(name = "get_stepSize", args = 0)]
    pub fn get_step_size(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, executing: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "LayoutComplete", args = 0)]
    pub fn layout_complete(self) -> ();

    #[method(name = "GraphicUpdateComplete", args = 0)]
    pub fn graphic_update_complete(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "UpdateCachedReferences", args = 0)]
    pub fn update_cached_references(self) -> ();

    #[method(name = "ClampValue", args = 1)]
    pub fn clamp_value(self, input: f32) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, input: f32, send_callback: bool) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "get_axis", args = 0)]
    pub fn get_axis(self) -> crate::unity_engine::ui::slider::Slider_Axis;

    #[method(name = "get_reverseValue", args = 0)]
    pub fn get_reverse_value(self) -> bool;

    #[method(name = "UpdateVisuals", args = 0)]
    pub fn update_visuals(self) -> ();

    #[method(name = "UpdateDrag", args = 2)]
    pub fn update_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        cam: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "MayDrag", args = 1)]
    pub fn may_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> bool;

    #[method(name = "OnPointerDown", args = 1)]
    pub fn on_pointer_down(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnDrag", args = 1)]
    pub fn on_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnMove", args = 1)]
    pub fn on_move(
        self,
        event_data: crate::unity_engine::event_systems::axiseventdata::AxisEventData,
    ) -> ();

    #[method(name = "FindSelectableOnLeft", args = 0)]
    pub fn find_selectable_on_left(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnRight", args = 0)]
    pub fn find_selectable_on_right(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnUp", args = 0)]
    pub fn find_selectable_on_up(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnDown", args = 0)]
    pub fn find_selectable_on_down(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "OnInitializePotentialDrag", args = 1)]
    pub fn on_initialize_potential_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "SetDirection", args = 2)]
    pub fn set_direction_2(
        self,
        direction: crate::unity_engine::ui::slider::Slider_Direction,
        include_rect_layouts: bool,
    ) -> ();

    #[method(name = "UnityEngine.UI.ICanvasElement.get_transform", args = 0)]
    pub fn unity_engine_ui_i_canvas_element_get_transform(
        self,
    ) -> crate::unity_engine::transform::Transform;
}

#[cfg(feature = "unity_engine-ui-slider")]
impl Slider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Slider),
                ::core::stringify!(new),
            )
        });
        <Self as ISliderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/slider/Slider_SliderEvent.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Slider.SliderEvent")]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < f32 >)]
pub struct Slider_SliderEvent {}

#[cfg(feature = "unity_engine-ui-slider")]
#[::unity2::methods]
impl Slider_SliderEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-slider")]
impl Slider_SliderEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Slider_SliderEvent),
                ::core::stringify!(new),
            )
        });
        <Self as ISlider_SliderEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/slider/Slider_Axis.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Slider_Axis {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Slider_Axis {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Slider.Axis";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Slider_Axis {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Slider_Axis {
    pub fn horizontal() -> Self {
        Self { value: 0 }
    }

    pub fn vertical() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/slider/Slider_Direction.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Slider_Direction {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Slider_Direction {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Slider.Direction";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Slider_Direction {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Slider_Direction {
    pub fn left_to_right() -> Self {
        Self { value: 0 }
    }

    pub fn right_to_left() -> Self {
        Self { value: 1 }
    }

    pub fn bottom_to_top() -> Self {
        Self { value: 2 }
    }

    pub fn top_to_bottom() -> Self {
        Self { value: 3 }
    }
}
