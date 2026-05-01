
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenucontent/AccessoryShopBuyMenuContent_KindIcon.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenuContent.KindIcon")]
#[parent(crate::system::object::Object)]
pub struct AccessoryShopBuyMenuContent_KindIcon {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::accessorydata::AccessoryData_Kinds,
}

#[cfg(feature = "app-accessoryshopbuymenucontent")]
#[::unity2::methods]
impl AccessoryShopBuyMenuContent_KindIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenucontent")]
impl AccessoryShopBuyMenuContent_KindIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenuContent_KindIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenuContent_KindIconMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuymenucontent/AccessoryShopBuyMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct AccessoryShopBuyMenuContent {
    #[rename(name = "m_CaptionText")]
    pub m_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_KindIcon")]
    pub m_kind_icon: ::unity2::Array<
        crate::app::accessoryshopbuymenucontent::AccessoryShopBuyMenuContent_KindIcon,
    >,
    #[rename(name = "m_ContentObject")]
    pub m_content_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-accessoryshopbuymenucontent")]
#[::unity2::methods]
impl AccessoryShopBuyMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetKind", args = 1)]
    pub fn set_kind(self, kind: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = "SetToPrevKind", args = 1)]
    pub fn set_to_prev_kind(
        self,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "SetToNextKind", args = 1)]
    pub fn set_to_next_kind(
        self,
        kind: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "IsFirstKind", args = 1)]
    pub fn is_first_kind(self, kind: crate::app::accessorydata::AccessoryData_Kinds) -> bool;

    #[method(name = "IsLastKind", args = 1)]
    pub fn is_last_kind(self, kind: crate::app::accessorydata::AccessoryData_Kinds) -> bool;

    #[method(name = "GetKindCount", args = 0)]
    pub fn get_kind_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessoryshopbuymenucontent")]
impl AccessoryShopBuyMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyMenuContentMethods>::ctor(this);
        this
    }
}
