
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangesourcemenucontent/RefineShopExchangeSourceMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeSourceMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RefineShopExchangeSourceMenuContent {
    #[rename(name = "m_HelpText")]
    pub m_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialSourceCaption")]
    pub m_material_source_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialSourceIconImage")]
    pub m_material_source_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MaterialSourceNameText")]
    pub m_material_source_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialSourceValueText")]
    pub m_material_source_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialTargetCaption")]
    pub m_material_target_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialTargetIconImage")]
    pub m_material_target_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_MaterialTargetNameText")]
    pub m_material_target_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaterialTargetValueText")]
    pub m_material_target_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineshopexchangesourcemenucontent")]
#[::unity2::methods]
impl RefineShopExchangeSourceMenuContent {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

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

    #[method(name = "UpdateRate", args = 0)]
    pub fn update_rate(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshopexchangesourcemenucontent")]
impl RefineShopExchangeSourceMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeSourceMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeSourceMenuContentMethods>::ctor(this);
        this
    }
}
