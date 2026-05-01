
use crate::app::basicitemmenucontent::BasicItemMenuContent;
use crate::app::basicitemmenucontent::IBasicItemMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/senditemmenucontent/SendItemMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "SendItemMenuContent")]
#[parent(crate::app::basicitemmenucontent::BasicItemMenuContent)]
pub struct SendItemMenuContent {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-senditemmenucontent")]
#[::unity2::methods]
impl SendItemMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-senditemmenucontent")]
impl SendItemMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SendItemMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISendItemMenuContentMethods>::ctor(this);
        this
    }
}
