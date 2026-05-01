
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenucontent/DiscardItemMenuContent_KindIcon.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenuContent.KindIcon")]
#[parent(crate::system::object::Object)]
pub struct DiscardItemMenuContent_KindIcon {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
}

#[cfg(feature = "app-discarditemmenucontent")]
#[::unity2::methods]
impl DiscardItemMenuContent_KindIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-discarditemmenucontent")]
impl DiscardItemMenuContent_KindIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenuContent_KindIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenuContent_KindIconMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/discarditemmenucontent/DiscardItemMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "DiscardItemMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct DiscardItemMenuContent {
    #[rename(name = "m_KindIcon")]
    pub m_kind_icon:
        ::unity2::Array<crate::app::discarditemmenucontent::DiscardItemMenuContent_KindIcon>,
    #[rename(name = "m_MenuTitleText")]
    pub m_menu_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TransporterText")]
    pub m_transporter_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StockValueText")]
    pub m_stock_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StockValueMaxText")]
    pub m_stock_value_max_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-discarditemmenucontent")]
#[::unity2::methods]
impl DiscardItemMenuContent {
    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetKind", args = 1)]
    pub fn set_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "SetToPrevKind", args = 1)]
    pub fn set_to_prev_kind(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
    ) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "SetToNextKind", args = 1)]
    pub fn set_to_next_kind(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
    ) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "IsFirstKind", args = 1)]
    pub fn is_first_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "IsLastKind", args = 1)]
    pub fn is_last_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "GetKindCount", args = 0)]
    pub fn get_kind_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-discarditemmenucontent")]
impl DiscardItemMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiscardItemMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IDiscardItemMenuContentMethods>::ctor(this);
        this
    }
}
