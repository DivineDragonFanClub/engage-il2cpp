
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalk/HubPlayTalk.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalk")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubPlayTalk {
    #[static_field]
    #[rename(name = "GiveAccessoryFlag1")]
    pub give_accessory_flag1: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "GiveAccessoryFlag2")]
    pub give_accessory_flag2: ::unity2::Il2CppString,
}

#[cfg(feature = "app-hubplaytalk")]
#[::unity2::methods]
impl HubPlayTalk {
    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "set_PlayerController", args = 1)]
    pub fn set_player_controller(
        self,
        value: crate::app::hubplayercontroller::HubPlayerController,
    ) -> ();

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "set_Player", args = 1)]
    pub fn set_player(self, value: crate::app::hubunitcontroller::HubUnitController) -> ();

    #[method(name = "get_Other", args = 0)]
    pub fn get_other(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::hubunitcontroller::HubUnitController,
    >;

    #[method(name = "set_Other", args = 1)]
    pub fn set_other(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::hubunitcontroller::HubUnitController,
        >,
    ) -> ();

    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera(self) -> crate::app::hubcamera::HubCamera;

    #[method(name = "set_Camera", args = 1)]
    pub fn set_camera(self, value: crate::app::hubcamera::HubCamera) -> ();

    #[method(name = "get_Access", args = 0)]
    pub fn get_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "set_Access", args = 1)]
    pub fn set_access(self, value: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Args", args = 0)]
    pub fn get_args(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Args", args = 1)]
    pub fn set_args(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FlagName", args = 0)]
    pub fn get_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PieceOfBond_Tutorial", args = 0)]
    pub fn get_piece_of_bond_tutorial(self) -> bool;

    #[method(name = "set_PieceOfBond_Tutorial", args = 1)]
    pub fn set_piece_of_bond_tutorial(self, value: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_2(
        self,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
        mid: ::unity2::Il2CppString,
        args: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "IsPlayerMove", args = 0)]
    pub fn is_player_move(self) -> bool;

    #[method(name = "GetOtherPosition", args = 0)]
    pub fn get_other_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetOtherHeadPosition", args = 0)]
    pub fn get_other_head_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetMessPID", args = 1)]
    pub fn get_mess_pid(mid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetMessPIDList", args = 1)]
    pub fn get_mess_pid_list(
        mid: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "SetupNormal", args = 0)]
    pub fn setup_normal(self) -> ();

    #[method(name = "Tutorial", args = 0)]
    pub fn tutorial(self) -> ();

    #[method(name = "SetupShop", args = 0)]
    pub fn setup_shop(self) -> ();

    #[method(name = "IsExistAdditionalStock", args = 1)]
    pub fn is_exist_additional_stock(self, shop_type: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetupShop", args = 1)]
    pub fn setup_shop_2(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "SetupEvent", args = 0)]
    pub fn setup_event(self) -> ();

    #[method(name = "ExecTalk", args = 0)]
    pub fn exec_talk(self) -> ();

    #[method(name = "ExecTalkPresent", args = 0)]
    pub fn exec_talk_present(self) -> ();

    #[method(name = "GetFavoriteItem", args = 0)]
    pub fn get_favorite_item(self) -> ::unity2::Il2CppString;

    #[method(name = "ExecTalkAfter", args = 0)]
    pub fn exec_talk_after(self) -> ();

    #[method(name = "TryJoinUnit", args = 1)]
    pub fn try_join_unit(self, pid: ::unity2::Il2CppString) -> ();

    #[method(name = "JoinEvent", args = 0)]
    pub fn join_event(self) -> ();

    #[method(name = "ExecEvent", args = 0)]
    pub fn exec_event(self) -> ();

    #[method(name = "ExecEventAfter", args = 0)]
    pub fn exec_event_after(self) -> ();

    #[method(name = "ExecShop", args = 0)]
    pub fn exec_shop(self) -> ();

    #[method(name = "ReturnTalk", args = 0)]
    pub fn return_talk(self) -> ();

    #[method(name = "ReturnTalkEvent", args = 0)]
    pub fn return_talk_event(self) -> ();

    #[method(name = "CreateDescNormal", args = 0)]
    pub fn create_desc_normal(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateDescShop", args = 0)]
    pub fn create_desc_shop(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateDescEvent", args = 0)]
    pub fn create_desc_event(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateProcDesc", args = 1)]
    pub fn create_proc_desc(
        self,
        talk_type: crate::app::hubplaytalk::HubPlayTalk_TalkType,
    ) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        talk_type: crate::app::hubplaytalk::HubPlayTalk_TalkType,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
    ) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        talk_type: crate::app::hubplaytalk::HubPlayTalk_TalkType,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        mid: ::unity2::Il2CppString,
        script: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateShopBind", args = 6)]
    pub fn create_shop_bind(
        super_: crate::app::procinst::ProcInst,
        talk_type: crate::app::hubplaytalk::HubPlayTalk_TalkType,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
        shop_type: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TrySilverCardMessage", args = 0)]
    pub fn try_silver_card_message(self) -> ();

    #[method(name = "TryGiveAccessory", args = 2)]
    pub fn try_give_accessory(
        self,
        aid_table: ::unity2::Array<::unity2::Il2CppString>,
        mid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TryGiveAccessory1", args = 0)]
    pub fn try_give_accessory1(self) -> ();

    #[method(name = "TryGiveAccessory2", args = 0)]
    pub fn try_give_accessory2(self) -> ();

    #[method(name = "CanGiveAccessory1", args = 0)]
    pub fn can_give_accessory1() -> bool;

    #[method(name = "CanGiveAccessory2", args = 0)]
    pub fn can_give_accessory2() -> bool;

    #[method(name = "CreateDescGiveAccessory", args = 0)]
    pub fn create_desc_give_accessory(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "TryCreateGiveAccessoryBind", args = 5)]
    pub fn try_create_give_accessory_bind(
        super_: crate::app::procinst::ProcInst,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
        mid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "get_WellTutorial", args = 0)]
    pub fn get_well_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "set_WellTutorial", args = 1)]
    pub fn set_well_tutorial(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "TryWellTutorial", args = 0)]
    pub fn try_well_tutorial(self) -> ();

    #[method(name = "CreateDescShopTalk", args = 0)]
    pub fn create_desc_shop_talk(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "CreateShopTalkBind", args = 6)]
    pub fn create_shop_talk_bind(
        super_: crate::app::procinst::ProcInst,
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
        mid: ::unity2::Il2CppString,
        tutorial: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-hubplaytalk")]
impl HubPlayTalk {
    pub fn new(
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalk),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkMethods>::ctor(this, player, camera, access);
        this
    }

    pub fn new_2(
        player: crate::app::hubplayercontroller::HubPlayerController,
        camera: crate::app::hubcamera::HubCamera,
        access: crate::app::hubaccess::HubAccess,
        mid: ::unity2::Il2CppString,
        args: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalk),
                ::core::stringify!(new_2),
            )
        });
        <Self as IHubPlayTalkMethods>::ctor_2(this, player, camera, access, mid, args);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalk/HubPlayTalk_CallPuppetEvent.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalk.CallPuppetEvent")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubPlayTalk_CallPuppetEvent {
    #[rename(name = "File")]
    pub file: ::unity2::Il2CppString,
    #[rename(name = "Mid")]
    pub mid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-hubplaytalk")]
#[::unity2::methods]
impl HubPlayTalk_CallPuppetEvent {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        file: ::unity2::Il2CppString,
        mid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, file: ::unity2::Il2CppString, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "Entry", args = 0)]
    pub fn entry(self) -> ();

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;
}

#[cfg(feature = "app-hubplaytalk")]
impl HubPlayTalk_CallPuppetEvent {
    pub fn new(file: ::unity2::Il2CppString, mid: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalk_CallPuppetEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalk_CallPuppetEventMethods>::ctor(this, file, mid);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalk/HubPlayTalk_TalkType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubPlayTalk_TalkType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubPlayTalk_TalkType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubPlayTalk.TalkType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubPlayTalk_TalkType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubPlayTalk_TalkType {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn shop() -> Self {
        Self { value: 1 }
    }

    pub fn event() -> Self {
        Self { value: 2 }
    }
}
