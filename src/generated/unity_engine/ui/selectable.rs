
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/selectable/Selectable_Transition.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Selectable_Transition {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Selectable_Transition {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Selectable.Transition";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Selectable_Transition {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Selectable_Transition {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn color_tint() -> Self {
        Self { value: 1 }
    }

    pub fn sprite_swap() -> Self {
        Self { value: 2 }
    }

    pub fn animation() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/selectable/Selectable_SelectionState.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Selectable_SelectionState {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Selectable_SelectionState {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Selectable.SelectionState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Selectable_SelectionState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Selectable_SelectionState {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn highlighted() -> Self {
        Self { value: 1 }
    }

    pub fn pressed() -> Self {
        Self { value: 2 }
    }

    pub fn selected() -> Self {
        Self { value: 3 }
    }

    pub fn disabled() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/selectable/Selectable.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Selectable")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct Selectable {
    #[static_field]
    #[rename(name = "s_Selectables")]
    pub s_selectables: ::unity2::Array<crate::unity_engine::ui::selectable::Selectable>,
    #[static_field]
    #[rename(name = "s_SelectableCount")]
    pub s_selectable_count: i32,
    #[rename(name = "m_EnableCalled")]
    pub m_enable_called: bool,
    #[rename(name = "m_Navigation")]
    pub m_navigation: crate::unity_engine::ui::navigation::Navigation,
    #[rename(name = "m_Transition")]
    pub m_transition: crate::unity_engine::ui::selectable::Selectable_Transition,
    #[rename(name = "m_Colors")]
    pub m_colors: crate::unity_engine::ui::colorblock::ColorBlock,
    #[rename(name = "m_SpriteState")]
    pub m_sprite_state: crate::unity_engine::ui::spritestate::SpriteState,
    #[rename(name = "m_AnimationTriggers")]
    pub m_animation_triggers: crate::unity_engine::ui::animationtriggers::AnimationTriggers,
    #[rename(name = "m_Interactable")]
    pub m_interactable: bool,
    #[rename(name = "m_TargetGraphic")]
    pub m_target_graphic: crate::unity_engine::ui::graphic::Graphic,
    #[rename(name = "m_GroupsAllowInteraction")]
    pub m_groups_allow_interaction: bool,
    #[rename(name = "m_CurrentIndex")]
    pub m_current_index: i32,
    #[rename(name = "m_CanvasGroupCache")]
    pub m_canvas_group_cache: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::canvasgroup::CanvasGroup,
    >,
}

#[cfg(feature = "unity_engine-ui-selectable")]
#[::unity2::methods]
impl Selectable {
    #[method(name = "get_allSelectablesArray", args = 0)]
    pub fn get_all_selectables_array(
    ) -> ::unity2::Array<crate::unity_engine::ui::selectable::Selectable>;

    #[method(name = "get_allSelectableCount", args = 0)]
    pub fn get_all_selectable_count() -> i32;

    #[method(name = "get_allSelectables", args = 0)]
    pub fn get_all_selectables() -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::ui::selectable::Selectable,
    >;

    #[method(name = "AllSelectablesNoAlloc", args = 1)]
    pub fn all_selectables_no_alloc(
        selectables: ::unity2::Array<crate::unity_engine::ui::selectable::Selectable>,
    ) -> i32;

    #[method(name = "get_navigation", args = 0)]
    pub fn get_navigation(self) -> crate::unity_engine::ui::navigation::Navigation;

    #[method(name = "set_navigation", args = 1)]
    pub fn set_navigation(self, value: crate::unity_engine::ui::navigation::Navigation) -> ();

    #[method(name = "get_transition", args = 0)]
    pub fn get_transition(self) -> crate::unity_engine::ui::selectable::Selectable_Transition;

    #[method(name = "set_transition", args = 1)]
    pub fn set_transition(
        self,
        value: crate::unity_engine::ui::selectable::Selectable_Transition,
    ) -> ();

    #[method(name = "get_colors", args = 0)]
    pub fn get_colors(self) -> crate::unity_engine::ui::colorblock::ColorBlock;

    #[method(name = "set_colors", args = 1)]
    pub fn set_colors(self, value: crate::unity_engine::ui::colorblock::ColorBlock) -> ();

    #[method(name = "get_spriteState", args = 0)]
    pub fn get_sprite_state(self) -> crate::unity_engine::ui::spritestate::SpriteState;

    #[method(name = "set_spriteState", args = 1)]
    pub fn set_sprite_state(self, value: crate::unity_engine::ui::spritestate::SpriteState) -> ();

    #[method(name = "get_animationTriggers", args = 0)]
    pub fn get_animation_triggers(
        self,
    ) -> crate::unity_engine::ui::animationtriggers::AnimationTriggers;

    #[method(name = "set_animationTriggers", args = 1)]
    pub fn set_animation_triggers(
        self,
        value: crate::unity_engine::ui::animationtriggers::AnimationTriggers,
    ) -> ();

    #[method(name = "get_targetGraphic", args = 0)]
    pub fn get_target_graphic(self) -> crate::unity_engine::ui::graphic::Graphic;

    #[method(name = "set_targetGraphic", args = 1)]
    pub fn set_target_graphic(self, value: crate::unity_engine::ui::graphic::Graphic) -> ();

    #[method(name = "get_interactable", args = 0)]
    pub fn get_interactable(self) -> bool;

    #[method(name = "set_interactable", args = 1)]
    pub fn set_interactable(self, value: bool) -> ();

    #[method(name = "get_isPointerInside", args = 0)]
    pub fn get_is_pointer_inside(self) -> bool;

    #[method(name = "set_isPointerInside", args = 1)]
    pub fn set_is_pointer_inside(self, value: bool) -> ();

    #[method(name = "get_isPointerDown", args = 0)]
    pub fn get_is_pointer_down(self) -> bool;

    #[method(name = "set_isPointerDown", args = 1)]
    pub fn set_is_pointer_down(self, value: bool) -> ();

    #[method(name = "get_hasSelection", args = 0)]
    pub fn get_has_selection(self) -> bool;

    #[method(name = "set_hasSelection", args = 1)]
    pub fn set_has_selection(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_image", args = 0)]
    pub fn get_image(self) -> crate::unity_engine::ui::image::Image;

    #[method(name = "set_image", args = 1)]
    pub fn set_image(self, value: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "get_animator", args = 0)]
    pub fn get_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnCanvasGroupChanged", args = 0)]
    pub fn on_canvas_group_changed(self) -> ();

    #[method(name = "IsInteractable", args = 0)]
    pub fn is_interactable(self) -> bool;

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "OnSetProperty", args = 0)]
    pub fn on_set_property(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "get_currentSelectionState", args = 0)]
    pub fn get_current_selection_state(
        self,
    ) -> crate::unity_engine::ui::selectable::Selectable_SelectionState;

    #[method(name = "InstantClearState", args = 0)]
    pub fn instant_clear_state(self) -> ();

    #[method(name = "DoStateTransition", args = 2)]
    pub fn do_state_transition(
        self,
        state: crate::unity_engine::ui::selectable::Selectable_SelectionState,
        instant: bool,
    ) -> ();

    #[method(name = "FindSelectable", args = 1)]
    pub fn find_selectable(
        self,
        dir: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "GetPointOnRectEdge", args = 2)]
    pub fn get_point_on_rect_edge(
        rect: crate::unity_engine::recttransform::RectTransform,
        dir: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Navigate", args = 2)]
    pub fn navigate(
        self,
        event_data: crate::unity_engine::event_systems::axiseventdata::AxisEventData,
        sel: crate::unity_engine::ui::selectable::Selectable,
    ) -> ();

    #[method(name = "FindSelectableOnLeft", args = 0)]
    pub fn find_selectable_on_left(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnRight", args = 0)]
    pub fn find_selectable_on_right(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnUp", args = 0)]
    pub fn find_selectable_on_up(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "FindSelectableOnDown", args = 0)]
    pub fn find_selectable_on_down(self) -> crate::unity_engine::ui::selectable::Selectable;

    #[method(name = "OnMove", args = 1)]
    pub fn on_move(
        self,
        event_data: crate::unity_engine::event_systems::axiseventdata::AxisEventData,
    ) -> ();

    #[method(name = "StartColorTween", args = 2)]
    pub fn start_color_tween(
        self,
        target_color: crate::unity_engine::color::Color,
        instant: bool,
    ) -> ();

    #[method(name = "DoSpriteSwap", args = 1)]
    pub fn do_sprite_swap(self, new_sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "TriggerAnimation", args = 1)]
    pub fn trigger_animation(self, triggername: ::unity2::Il2CppString) -> ();

    #[method(name = "IsHighlighted", args = 0)]
    pub fn is_highlighted(self) -> bool;

    #[method(name = "IsPressed", args = 0)]
    pub fn is_pressed(self) -> bool;

    #[method(name = "EvaluateAndTransitionToSelectionState", args = 0)]
    pub fn evaluate_and_transition_to_selection_state(self) -> ();

    #[method(name = "OnPointerDown", args = 1)]
    pub fn on_pointer_down(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerUp", args = 1)]
    pub fn on_pointer_up(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerEnter", args = 1)]
    pub fn on_pointer_enter(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerExit", args = 1)]
    pub fn on_pointer_exit(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSelect", args = 1)]
    pub fn on_select(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnDeselect", args = 1)]
    pub fn on_deselect(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "Select", args = 0)]
    pub fn select(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-ui-selectable")]
impl Selectable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Selectable),
                ::core::stringify!(new),
            )
        });
        <Self as ISelectableMethods>::ctor(this);
        this
    }
}
