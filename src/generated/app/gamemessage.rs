
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamemessage/GameMessage_Status.md")))]
#[::unity2::class(namespace = "App", name = "GameMessage.Status")]
#[parent(crate::app::bitfield32::BitField32)]
pub struct GameMessage_Status {
    #[static_field]
    #[rename(name = "SkipDisable")]
    pub skip_disable: i32,
    #[static_field]
    #[rename(name = "KeyWait")]
    pub key_wait: i32,
    #[static_field]
    #[rename(name = "SystemWait")]
    pub system_wait: i32,
    #[static_field]
    #[rename(name = "SystemWaitEnd")]
    pub system_wait_end: i32,
    #[static_field]
    #[rename(name = "Warning")]
    pub warning: i32,
    #[static_field]
    #[rename(name = "FatalError")]
    pub fatal_error: i32,
    #[static_field]
    #[rename(name = "WindowOpened")]
    pub window_opened: i32,
}

#[cfg(feature = "app-gamemessage")]
#[::unity2::methods]
impl GameMessage_Status {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gamemessage")]
impl GameMessage_Status {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameMessage_Status),
                ::core::stringify!(new),
            )
        });
        <Self as IGameMessage_StatusMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamemessage/GameMessage.md")))]
#[::unity2::class(namespace = "App", name = "GameMessage")]
#[parent(crate::app::procinst::ProcInst)]
pub struct GameMessage {
    #[rename(name = "m_content")]
    pub m_content: crate::app::gamemessagecontent::GameMessageContent,
    #[rename(name = "m_status")]
    pub m_status: crate::app::gamemessage::GameMessage_Status,
    #[rename(name = "m_mess")]
    pub m_mess: ::unity2::Il2CppString,
    #[rename(name = "m_expandedMess")]
    pub m_expanded_mess: ::unity2::Il2CppString,
    #[rename(name = "m_tickCount")]
    pub m_tick_count: i32,
    #[rename(name = "m_endFrame")]
    pub m_end_frame: i32,
}

#[cfg(feature = "app-gamemessage")]
#[::unity2::methods]
impl GameMessage {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        mess: ::unity2::Il2CppString,
        content: crate::app::gamemessagecontent::GameMessageContent,
        status: i32,
    ) -> ();

    #[method(name = "CreateDefaultDesc", args = 0)]
    pub fn create_default_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetStatus", args = 0)]
    pub fn get_status(self) -> crate::app::gamemessage::GameMessage_Status;

    #[method(name = "GetMess", args = 0)]
    pub fn get_mess(self) -> ::unity2::Il2CppString;

    #[method(name = "GetExpandedMess", args = 0)]
    pub fn get_expanded_mess(self) -> ::unity2::Il2CppString;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "OpenWindow", args = 0)]
    pub fn open_window(self) -> ();

    #[method(name = "WaitOpenWindow", args = 0)]
    pub fn wait_open_window(self) -> ();

    #[method(name = "CloseWindow", args = 0)]
    pub fn close_window(self) -> ();

    #[method(name = "WaitCloseWindow", args = 0)]
    pub fn wait_close_window(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateCommon", args = 4)]
    pub fn create_common(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
        is_bind: bool,
        status: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateNoBind", args = 3)]
    pub fn create_no_bind(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
        status: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
        status: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateKeyWait", args = 2)]
    pub fn create_key_wait(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateSystem", args = 2)]
    pub fn create_system(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateWarning", args = 2)]
    pub fn create_warning(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateItemGetImpl", args = 5)]
    pub fn create_item_get_impl(
        super_: crate::app::procinst::ProcInst,
        item: crate::app::itemdata::ItemData,
        name: ::unity2::Il2CppString,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateItemGet", args = 4)]
    pub fn create_item_get(
        super_: crate::app::procinst::ProcInst,
        item_data: crate::app::itemdata::ItemData,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateUnitItemGet", args = 4)]
    pub fn create_unit_item_get(
        super_: crate::app::procinst::ProcInst,
        unit_item: crate::app::unititem::UnitItem,
        label: ::unity2::Il2CppString,
        count: i32,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateGoldGain", args = 3)]
    pub fn create_gold_gain(
        super_: crate::app::procinst::ProcInst,
        gold: i32,
        label: ::unity2::Il2CppString,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateGrowUpItemUse", args = 2)]
    pub fn create_grow_up_item_use(
        super_: crate::app::procinst::ProcInst,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "CreateEnhanceItemUse", args = 2)]
    pub fn create_enhance_item_use(
        super_: crate::app::procinst::ProcInst,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::gamemessage::GameMessage;

    #[method(name = "SetShadowOff", args = 0)]
    pub fn set_shadow_off(self) -> crate::app::gamemessage::GameMessage;

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, x: f32, y: f32) -> crate::app::gamemessage::GameMessage;
}

#[cfg(feature = "app-gamemessage")]
impl GameMessage {
    pub fn new(
        mess: ::unity2::Il2CppString,
        content: crate::app::gamemessagecontent::GameMessageContent,
        status: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameMessage),
                ::core::stringify!(new),
            )
        });
        <Self as IGameMessageMethods>::ctor(this, mess, content, status);
        this
    }
}
