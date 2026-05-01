
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistcontent/FriendListContent_FriendContent.md")))]
#[::unity2::class(namespace = "App", name = "FriendListContent.FriendContent")]
#[parent(crate::system::object::Object)]
pub struct FriendListContent_FriendContent {}

#[cfg(feature = "app-friendlistcontent")]
#[::unity2::methods]
impl FriendListContent_FriendContent {
    #[method(name = "get_TitleObj", args = 0)]
    pub fn get_title_obj(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_TitleObj", args = 1)]
    pub fn set_title_obj(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_MessageObj", args = 0)]
    pub fn get_message_obj(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_MessageObj", args = 1)]
    pub fn set_message_obj(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_StampObj", args = 0)]
    pub fn get_stamp_obj(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_StampObj", args = 1)]
    pub fn set_stamp_obj(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_PictureObj", args = 0)]
    pub fn get_picture_obj(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_PictureObj", args = 1)]
    pub fn set_picture_obj(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetTextObj", args = 1)]
    pub fn set_text_obj(self, text_root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetTitle", args = 1)]
    pub fn set_title(self, title: ::unity2::Il2CppString) -> ();

    #[method(name = "SetMessage", args = 1)]
    pub fn set_message(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = "SetStamp", args = 1)]
    pub fn set_stamp(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "SetImage", args = 1)]
    pub fn set_image(self, sprite: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "StartShow", args = 0)]
    pub fn start_show(self) -> ();

    #[method(name = "StartHide", args = 1)]
    pub fn start_hide(self, is_chara_change: bool) -> ();

    #[method(name = "IsChanging", args = 0)]
    pub fn is_changing(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-friendlistcontent")]
impl FriendListContent_FriendContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListContent_FriendContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListContent_FriendContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistcontent/FriendListContent.md")))]
#[::unity2::class(namespace = "App", name = "FriendListContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FriendListContent {
    #[rename(name = "m_FriendContentArray")]
    pub m_friend_content_array:
        ::unity2::Array<crate::app::friendlistcontent::FriendListContent_FriendContent>,
    #[rename(name = "m_ActiveFriendContentIndex")]
    pub m_active_friend_content_index: i32,
    #[rename(name = "m_IndexAnimatorArray")]
    pub m_index_animator_array: ::unity2::Array<crate::unity_engine::animator::Animator>,
}

#[cfg(feature = "app-friendlistcontent")]
#[::unity2::methods]
impl FriendListContent {
    #[method(name = "set_IsInitialized", args = 1)]
    pub fn set_is_initialized(self, value: bool) -> ();

    #[method(name = "get_IsInitialized", args = 0)]
    pub fn get_is_initialized(self) -> bool;

    #[method(name = "set_IsPageChanging", args = 1)]
    pub fn set_is_page_changing(self, value: bool) -> ();

    #[method(name = "get_IsPageChanging", args = 0)]
    pub fn get_is_page_changing(self) -> bool;

    #[method(name = "set_IsShowArrow", args = 1)]
    pub fn set_is_show_arrow(self, value: bool) -> ();

    #[method(name = "get_IsShowArrow", args = 0)]
    pub fn get_is_show_arrow(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetFriendDataCore", args = 3)]
    pub fn set_friend_data_core(
        self,
        friend_data: crate::app::friendlistdata::FriendListData,
        chara_image_sprite: crate::unity_engine::sprite::Sprite,
        stamp_sprite: crate::unity_engine::sprite::Sprite,
    ) -> ();

    #[method(name = "InitCountryTag", args = 1)]
    pub fn init_country_tag(
        self,
        data_array: crate::system::collections::generic::list_1::List_1<
            crate::app::friendlistdata::FriendListData,
        >,
    ) -> ();

    #[method(name = "SetFriendDataAtFirst", args = 3)]
    pub fn set_friend_data_at_first(
        self,
        friend_data: crate::app::friendlistdata::FriendListData,
        chara_image_sprite: crate::unity_engine::sprite::Sprite,
        stamp_sprite: crate::unity_engine::sprite::Sprite,
    ) -> ();

    #[method(name = "SetFriendData", args = 3)]
    pub fn set_friend_data(
        self,
        friend_data: crate::app::friendlistdata::FriendListData,
        chara_image_sprite: crate::unity_engine::sprite::Sprite,
        stamp_sprite: crate::unity_engine::sprite::Sprite,
    ) -> ();

    #[method(name = "get_IsCharaChange", args = 0)]
    pub fn get_is_chara_change(self) -> bool;

    #[method(name = "set_IsCharaChange", args = 1)]
    pub fn set_is_chara_change(self, value: bool) -> ();

    #[method(name = "get_IsCountryChange", args = 0)]
    pub fn get_is_country_change(self) -> bool;

    #[method(name = "set_IsCountryChange", args = 1)]
    pub fn set_is_country_change(self, value: bool) -> ();

    #[method(name = "HideArrow", args = 0)]
    pub fn hide_arrow(self) -> ();

    #[method(name = "ShowArrow", args = 0)]
    pub fn show_arrow(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-friendlistcontent")]
impl FriendListContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListContent),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListContentMethods>::ctor(this);
        this
    }
}
