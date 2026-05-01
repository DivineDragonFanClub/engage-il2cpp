
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/scrollbar/Scrollbar_ScrollEvent.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Scrollbar.ScrollEvent")]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < f32 >)]
pub struct Scrollbar_ScrollEvent {}

#[cfg(feature = "unity_engine-ui-scrollbar")]
#[::unity2::methods]
impl Scrollbar_ScrollEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-scrollbar")]
impl Scrollbar_ScrollEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Scrollbar_ScrollEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IScrollbar_ScrollEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/scrollbar/Scrollbar.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Scrollbar")]
#[parent(crate::unity_engine::ui::selectable::Selectable)]
pub struct Scrollbar {
    #[rename(name = "m_HandleRect")]
    pub m_handle_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Direction")]
    pub m_direction: crate::unity_engine::ui::scrollbar::Scrollbar_Direction,
    #[rename(name = "m_Value")]
    pub m_value: f32,
    #[rename(name = "m_Size")]
    pub m_size: f32,
    #[rename(name = "m_NumberOfSteps")]
    pub m_number_of_steps: i32,
    #[rename(name = "m_OnValueChanged")]
    pub m_on_value_changed: crate::unity_engine::ui::scrollbar::Scrollbar_ScrollEvent,
    #[rename(name = "m_ContainerRect")]
    pub m_container_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Offset")]
    pub m_offset: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Tracker")]
    pub m_tracker: crate::unity_engine::drivenrecttransformtracker::DrivenRectTransformTracker,
    #[rename(name = "m_PointerDownRepeat")]
    pub m_pointer_down_repeat: crate::unity_engine::coroutine::Coroutine,
    #[rename(name = "isPointerDownAndNotDragging")]
    pub is_pointer_down_and_not_dragging: bool,
    #[rename(name = "m_DelayedUpdateVisuals")]
    pub m_delayed_update_visuals: bool,
}

#[cfg(feature = "unity_engine-ui-scrollbar")]
#[::unity2::methods]
impl Scrollbar {
    #[method(name = "get_handleRect", args = 0)]
    pub fn get_handle_rect(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "set_handleRect", args = 1)]
    pub fn set_handle_rect(self, value: crate::unity_engine::recttransform::RectTransform) -> ();

    #[method(name = "get_direction", args = 0)]
    pub fn get_direction(self) -> crate::unity_engine::ui::scrollbar::Scrollbar_Direction;

    #[method(name = "set_direction", args = 1)]
    pub fn set_direction(
        self,
        value: crate::unity_engine::ui::scrollbar::Scrollbar_Direction,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_value", args = 1)]
    pub fn set_value(self, value: f32) -> ();

    #[method(name = "SetValueWithoutNotify", args = 1)]
    pub fn set_value_without_notify(self, input: f32) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> f32;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: f32) -> ();

    #[method(name = "get_numberOfSteps", args = 0)]
    pub fn get_number_of_steps(self) -> i32;

    #[method(name = "set_numberOfSteps", args = 1)]
    pub fn set_number_of_steps(self, value: i32) -> ();

    #[method(name = "get_onValueChanged", args = 0)]
    pub fn get_on_value_changed(self) -> crate::unity_engine::ui::scrollbar::Scrollbar_ScrollEvent;

    #[method(name = "set_onValueChanged", args = 1)]
    pub fn set_on_value_changed(
        self,
        value: crate::unity_engine::ui::scrollbar::Scrollbar_ScrollEvent,
    ) -> ();

    #[method(name = "get_stepSize", args = 0)]
    pub fn get_step_size(self) -> f32;

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

    #[method(name = "UpdateCachedReferences", args = 0)]
    pub fn update_cached_references(self) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, input: f32, send_callback: bool) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "get_axis", args = 0)]
    pub fn get_axis(self) -> crate::unity_engine::ui::scrollbar::Scrollbar_Axis;

    #[method(name = "get_reverseValue", args = 0)]
    pub fn get_reverse_value(self) -> bool;

    #[method(name = "UpdateVisuals", args = 0)]
    pub fn update_visuals(self) -> ();

    #[method(name = "UpdateDrag", args = 1)]
    pub fn update_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "DoUpdateDrag", args = 2)]
    pub fn do_update_drag(
        self,
        handle_corner: crate::unity_engine::vector2::Vector2,
        remaining_size: f32,
    ) -> ();

    #[method(name = "MayDrag", args = 1)]
    pub fn may_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> bool;

    #[method(name = "OnBeginDrag", args = 1)]
    pub fn on_begin_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnDrag", args = 1)]
    pub fn on_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerDown", args = 1)]
    pub fn on_pointer_down(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "ClickRepeat", args = 1)]
    pub fn click_repeat(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ClickRepeat", args = 2)]
    pub fn click_repeat_2(
        self,
        screen_position: crate::unity_engine::vector2::Vector2,
        camera: crate::unity_engine::camera::Camera,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OnPointerUp", args = 1)]
    pub fn on_pointer_up(
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
        direction: crate::unity_engine::ui::scrollbar::Scrollbar_Direction,
        include_rect_layouts: bool,
    ) -> ();

    #[method(name = "UnityEngine.UI.ICanvasElement.get_transform", args = 0)]
    pub fn unity_engine_ui_i_canvas_element_get_transform(
        self,
    ) -> crate::unity_engine::transform::Transform;
}

#[cfg(feature = "unity_engine-ui-scrollbar")]
impl Scrollbar {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Scrollbar),
                ::core::stringify!(new),
            )
        });
        <Self as IScrollbarMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/scrollbar/Scrollbar_Axis.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Scrollbar_Axis {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Scrollbar_Axis {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Scrollbar.Axis";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Scrollbar_Axis {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Scrollbar_Axis {
    pub fn horizontal() -> Self {
        Self { value: 0 }
    }

    pub fn vertical() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/scrollbar/Scrollbar_Direction.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Scrollbar_Direction {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Scrollbar_Direction {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Scrollbar.Direction";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Scrollbar_Direction {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Scrollbar_Direction {
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
