
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupselectmenuitemcontent/MyRoomWakeupSelectMenuItemContent.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSelectMenuItemContent")]
#[parent(crate::app::basicmenuitemcontent::BasicMenuItemContent)]
pub struct MyRoomWakeupSelectMenuItemContent {
    #[static_field]
    #[rename(name = "TYPE_MAX")]
    pub type_max: i32,
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Name")]
    pub m_name: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RankObjects")]
    pub m_rank_objects: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
    #[rename(name = "m_Frame")]
    pub m_frame: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_RankImages")]
    pub m_rank_images: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_RankImagesBase")]
    pub m_rank_images_base: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_RankImagesNumber")]
    pub m_rank_images_number: ::unity2::Array<crate::tm_pro::textmeshprougui::TextMeshProUGUI>,
}

#[cfg(feature = "app-myroomwakeupselectmenuitemcontent")]
#[::unity2::methods]
impl MyRoomWakeupSelectMenuItemContent {
    #[method(name = "SetupObjects", args = 0)]
    pub fn setup_objects(self) -> ();

    #[method(name = "SetRankColor", args = 4)]
    pub fn set_rank_color(
        self,
        rank_image: crate::unity_engine::ui::image::Image,
        base_image: crate::unity_engine::ui::image::Image,
        number_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "GetCommandColor", args = 0)]
    pub fn get_command_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "Build", args = 1)]
    pub fn build(self, menu_item: crate::app::basicmenuitem::BasicMenuItem) -> ();

    #[method(name = "BuildText", args = 0)]
    pub fn build_text(self) -> ();

    #[method(name = "UpdateTextColor", args = 0)]
    pub fn update_text_color(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomwakeupselectmenuitemcontent")]
impl MyRoomWakeupSelectMenuItemContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSelectMenuItemContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSelectMenuItemContentMethods>::ctor(this);
        this
    }
}
