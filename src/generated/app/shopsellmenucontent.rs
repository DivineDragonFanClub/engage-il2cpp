
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenucontent/ShopSellMenuContent_KindIcon.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuContent.KindIcon")]
#[parent(crate::system::object::Object)]
pub struct ShopSellMenuContent_KindIcon {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
}

#[cfg(feature = "app-shopsellmenucontent")]
#[::unity2::methods]
impl ShopSellMenuContent_KindIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopsellmenucontent")]
impl ShopSellMenuContent_KindIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuContent_KindIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuContent_KindIconMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopsellmenucontent/ShopSellMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "ShopSellMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct ShopSellMenuContent {
    #[rename(name = "m_KindIcon")]
    pub m_kind_icon: ::unity2::Array<crate::app::shopsellmenucontent::ShopSellMenuContent_KindIcon>,
    #[rename(name = "m_ColumnTitleItemText")]
    pub m_column_title_item_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ColumnTitleUsableCountText")]
    pub m_column_title_usable_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ColumnTitlePriceText")]
    pub m_column_title_price_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ContentObject")]
    pub m_content_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-shopsellmenucontent")]
#[::unity2::methods]
impl ShopSellMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

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

    #[method(name = "GetFirstKind", args = 0)]
    pub fn get_first_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "GetLastKind", args = 0)]
    pub fn get_last_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "IsFirstKind", args = 1)]
    pub fn is_first_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "IsLastKind", args = 1)]
    pub fn is_last_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "GetKindCount", args = 0)]
    pub fn get_kind_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopsellmenucontent")]
impl ShopSellMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopSellMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IShopSellMenuContentMethods>::ctor(this);
        this
    }
}
