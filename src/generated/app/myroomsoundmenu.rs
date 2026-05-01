
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsoundmenu/MyRoomSoundMenu.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomSoundMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomSoundMenu {
    #[rename(name = "m_menuType")]
    pub m_menu_type: crate::app::myroomsoundmenu::MyRoomSoundMenu_MenuType,
    #[static_field]
    #[rename(name = "DefaultBgmChangeSuppressCount")]
    pub default_bgm_change_suppress_count: i32,
    #[rename(name = "m_bgmChangeSuppressCount")]
    pub m_bgm_change_suppress_count: i32,
}

#[cfg(feature = "app-myroomsoundmenu")]
#[::unity2::methods]
impl MyRoomSoundMenu {
    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::bgmplayer::BgmPlayer;

    #[method(name = "get_SelectBgm", args = 0)]
    pub fn get_select_bgm() -> ::unity2::Il2CppString;

    #[method(name = "set_SelectBgm", args = 1)]
    pub fn set_select_bgm(value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_type: crate::app::myroomsoundmenu::MyRoomSoundMenu_MenuType,
        select_bgm: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateBindMusic", args = 1)]
    pub fn create_bind_music(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindJukebox", args = 1)]
    pub fn create_bind_jukebox(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindFieldBgmSelect", args = 3)]
    pub fn create_bind_field_bgm_select(
        super_: crate::app::procinst::ProcInst,
        select_bgm: ::unity2::Il2CppString,
        callback: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "SetMusicName", args = 1)]
    pub fn set_music_name(self, music_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetMusicHelpText", args = 1)]
    pub fn set_music_help_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlaySelect", args = 1)]
    pub fn play_select(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "BuildAttribute", args = 1)]
    pub fn build_attribute(
        self,
        item: crate::app::myroomsoundmenuitem::MyRoomSoundMenuItem,
    ) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "Decide", args = 1)]
    pub fn decide(
        self,
        item: crate::app::myroomsoundmenuitem::MyRoomSoundMenuItem,
    ) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-myroomsoundmenu")]
impl MyRoomSoundMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_type: crate::app::myroomsoundmenu::MyRoomSoundMenu_MenuType,
        select_bgm: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomSoundMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomSoundMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            menu_type,
            select_bgm,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomsoundmenu/MyRoomSoundMenu_MenuType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomSoundMenu_MenuType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomSoundMenu_MenuType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomSoundMenu.MenuType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomSoundMenu_MenuType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomSoundMenu_MenuType {
    pub fn sound_test() -> Self {
        Self { value: 0 }
    }

    pub fn jukebox() -> Self {
        Self { value: 1 }
    }

    pub fn bgm_select() -> Self {
        Self { value: 2 }
    }
}
