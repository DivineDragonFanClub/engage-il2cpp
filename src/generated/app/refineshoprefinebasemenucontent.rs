
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinebasemenucontent/RefineShopRefineBaseMenuContent_KindIcon.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineBaseMenuContent.KindIcon")]
#[parent(crate::system::object::Object)]
pub struct RefineShopRefineBaseMenuContent_KindIcon {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::itemdata::ItemData_Kinds,
}

#[cfg(feature = "app-refineshoprefinebasemenucontent")]
#[::unity2::methods]
impl RefineShopRefineBaseMenuContent_KindIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshoprefinebasemenucontent")]
impl RefineShopRefineBaseMenuContent_KindIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineBaseMenuContent_KindIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineBaseMenuContent_KindIconMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinebasemenucontent/RefineShopRefineBaseMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineBaseMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RefineShopRefineBaseMenuContent {
    #[rename(name = "m_CaptionText")]
    pub m_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_KindIcon")]
    pub m_kind_icon: ::unity2::Array<
        crate::app::refineshoprefinebasemenucontent::RefineShopRefineBaseMenuContent_KindIcon,
    >,
    #[rename(name = "m_EnabledItemKindCount")]
    pub m_enabled_item_kind_count: i32,
}

#[cfg(feature = "app-refineshoprefinebasemenucontent")]
#[::unity2::methods]
impl RefineShopRefineBaseMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetKind", args = 1)]
    pub fn set_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "GetKindNum", args = 0)]
    pub fn get_kind_num(self) -> i32;

    #[method(name = "GetKindIndex", args = 1)]
    pub fn get_kind_index(self, kind: crate::app::itemdata::ItemData_Kinds) -> i32;

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

    #[method(name = "EnableItemKind", args = 2)]
    pub fn enable_item_kind(self, kind: crate::app::itemdata::ItemData_Kinds, enabled: bool) -> ();

    #[method(name = "UpdateGetKindCount", args = 0)]
    pub fn update_get_kind_count(self) -> ();

    #[method(name = "GetKindCount", args = 0)]
    pub fn get_kind_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshoprefinebasemenucontent")]
impl RefineShopRefineBaseMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineBaseMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineBaseMenuContentMethods>::ctor(this);
        this
    }
}
