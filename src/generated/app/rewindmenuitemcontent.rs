
use crate::app::basicmenuitemcontent::BasicMenuItemContent;
use crate::app::basicmenuitemcontent::IBasicMenuItemContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewindmenuitemcontent/RewindMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "RewindMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct RewindMenuItemContent {
    #[rename(name = "m_prev")]
    pub m_prev: crate::app::rewindmenuitemcontent::RewindMenuItemContent,
    #[rename(name = "m_next")]
    pub m_next: crate::app::rewindmenuitemcontent::RewindMenuItemContent,
    #[rename(name = "m_initStateName")]
    pub m_init_state_name: ::unity2::Il2CppString,
    #[rename(name = "m_MenuPosIndex")]
    pub m_menu_pos_index: i32,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_WindowImage")]
    pub m_window_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_WhiteImage")]
    pub m_white_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_LogText")]
    pub m_log_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_UnitObject")]
    pub m_unit_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DieUnitIcon")]
    pub m_die_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_DieIconImage")]
    pub m_die_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Menu")]
    pub m_menu: crate::app::rewindmenu::RewindMenu,
}

#[cfg(feature = "app-rewindmenuitemcontent")]
#[::unity2::methods]
impl RewindMenuItemContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetEnableImageAndText", args = 1)]
    pub fn set_enable_image_and_text(self, enable: bool) -> ();

    #[method(name = "SetSelectColor", args = 1)]
    pub fn set_select_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "SetupByMenuItem", args = 1)]
    pub fn setup_by_menu_item(
        self,
        rewind_menu_item: crate::app::rewindmenu::RewindMenu_MenuItem,
    ) -> ();

    #[method(name = "GetMenuItem", args = 1)]
    pub fn get_menu_item(self, menu_pos_index: i32) -> crate::app::rewindmenu::RewindMenu_MenuItem;

    #[method(name = "IsAnimStateName", args = 1)]
    pub fn is_anim_state_name(self, state_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsMoveAnimState", args = 2)]
    pub fn is_move_anim_state(self, from: i32, to: i32) -> bool;

    #[method(name = "IsMoving", args = 0)]
    pub fn is_moving(self) -> bool;

    #[method(name = "MoveUp", args = 0)]
    pub fn move_up(self) -> ();

    #[method(name = "MoveDown", args = 0)]
    pub fn move_down(self) -> ();

    #[method(name = "MoveUpImm", args = 0)]
    pub fn move_up_imm(self) -> ();

    #[method(name = "MoveDownImm", args = 0)]
    pub fn move_down_imm(self) -> ();

    #[method(name = "PlayAnimMoveUp", args = 0)]
    pub fn play_anim_move_up(self) -> ();

    #[method(name = "PlayAnimMoveDown", args = 0)]
    pub fn play_anim_move_down(self) -> ();
}

#[cfg(feature = "app-rewindmenuitemcontent")]
impl RewindMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewindMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRewindMenuItemContentMethods>::ctor(this);
        this
    }
}
