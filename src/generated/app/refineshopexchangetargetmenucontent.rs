
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangetargetmenucontent/RefineShopExchangeTargetMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeTargetMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RefineShopExchangeTargetMenuContent {
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NoticeText")]
    pub m_notice_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineshopexchangetargetmenucontent")]
#[::unity2::methods]
impl RefineShopExchangeTargetMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "UpdateContent", args = 0)]
    pub fn update_content(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshopexchangetargetmenucontent")]
impl RefineShopExchangeTargetMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeTargetMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeTargetMenuContentMethods>::ctor(this);
        this
    }
}
