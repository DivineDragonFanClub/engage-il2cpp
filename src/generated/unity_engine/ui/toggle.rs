
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/toggle/Toggle_ToggleEvent.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Toggle.ToggleEvent")]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < bool >)]
pub struct Toggle_ToggleEvent {}

#[cfg(feature = "unity_engine-ui-toggle")]
#[::unity2::methods]
impl Toggle_ToggleEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-toggle")]
impl Toggle_ToggleEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Toggle_ToggleEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IToggle_ToggleEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/toggle/Toggle.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Toggle")]
#[parent(crate::unity_engine::ui::selectable::Selectable)]
pub struct Toggle {
    #[rename(name = "toggleTransition")]
    pub toggle_transition: crate::unity_engine::ui::toggle::Toggle_ToggleTransition,
    #[rename(name = "graphic")]
    pub graphic: crate::unity_engine::ui::graphic::Graphic,
    #[rename(name = "m_Group")]
    pub m_group: crate::unity_engine::ui::togglegroup::ToggleGroup,
    #[rename(name = "onValueChanged")]
    pub on_value_changed: crate::unity_engine::ui::toggle::Toggle_ToggleEvent,
    #[rename(name = "m_IsOn")]
    pub m_is_on: bool,
}

#[cfg(feature = "unity_engine-ui-toggle")]
#[::unity2::methods]
impl Toggle {
    #[method(name = "get_group", args = 0)]
    pub fn get_group(self) -> crate::unity_engine::ui::togglegroup::ToggleGroup;

    #[method(name = "set_group", args = 1)]
    pub fn set_group(self, value: crate::unity_engine::ui::togglegroup::ToggleGroup) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, executing: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "LayoutComplete", args = 0)]
    pub fn layout_complete(self) -> ();

    #[method(name = "GraphicUpdateComplete", args = 0)]
    pub fn graphic_update_complete(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "SetToggleGroup", args = 2)]
    pub fn set_toggle_group(
        self,
        new_group: crate::unity_engine::ui::togglegroup::ToggleGroup,
        set_member_value: bool,
    ) -> ();

    #[method(name = "get_isOn", args = 0)]
    pub fn get_is_on(self) -> bool;

    #[method(name = "set_isOn", args = 1)]
    pub fn set_is_on(self, value: bool) -> ();

    #[method(name = "SetIsOnWithoutNotify", args = 1)]
    pub fn set_is_on_without_notify(self, value: bool) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, value: bool, send_callback: bool) -> ();

    #[method(name = "PlayEffect", args = 1)]
    pub fn play_effect(self, instant: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "InternalToggle", args = 0)]
    pub fn internal_toggle(self) -> ();

    #[method(name = "OnPointerClick", args = 1)]
    pub fn on_pointer_click(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSubmit", args = 1)]
    pub fn on_submit(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "UnityEngine.UI.ICanvasElement.get_transform", args = 0)]
    pub fn unity_engine_ui_i_canvas_element_get_transform(
        self,
    ) -> crate::unity_engine::transform::Transform;
}

#[cfg(feature = "unity_engine-ui-toggle")]
impl Toggle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Toggle),
                ::core::stringify!(new),
            )
        });
        <Self as IToggleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/toggle/Toggle_ToggleTransition.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Toggle_ToggleTransition {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Toggle_ToggleTransition {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Toggle.ToggleTransition";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Toggle_ToggleTransition {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Toggle_ToggleTransition {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn fade() -> Self {
        Self { value: 1 }
    }
}
