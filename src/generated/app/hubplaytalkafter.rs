
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubPlayTalkAfter_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubPlayTalkAfter_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubPlayTalkAfter.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubPlayTalkAfter_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubPlayTalkAfter_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn select_menu() -> Self {
        Self { value: 1 }
    }

    pub fn select_gift() -> Self {
        Self { value: 2 }
    }

    pub fn reaction_gift() -> Self {
        Self { value: 3 }
    }

    pub fn reliance() -> Self {
        Self { value: 4 }
    }

    pub fn cooking() -> Self {
        Self { value: 5 }
    }

    pub fn fishing() -> Self {
        Self { value: 6 }
    }

    pub fn fishing_picture_book() -> Self {
        Self { value: 7 }
    }

    pub fn dragon_ride() -> Self {
        Self { value: 8 }
    }

    pub fn muscle() -> Self {
        Self { value: 9 }
    }

    pub fn flea_market() -> Self {
        Self { value: 10 }
    }

    pub fn fortune_telling() -> Self {
        Self { value: 11 }
    }

    pub fn end() -> Self {
        Self { value: 12 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_ReplacePerson.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.ReplacePerson")]
#[parent(crate::app::procinst::ProcInst)]
pub struct HubPlayTalkAfter_ReplacePerson {
    #[rename(name = "m_LoadingCharacterCount")]
    pub m_loading_character_count: i32,
}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_ReplacePerson {
    #[method(name = "IsChangePerson", args = 0)]
    pub fn is_change_person(self) -> bool;

    #[method(name = "SetupUnit", args = 2)]
    pub fn setup_unit(
        self,
        locator: crate::unity_engine::gameobject::GameObject,
        pid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ResetUnit", args = 1)]
    pub fn reset_unit(self, locator_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "PrepareChangePerson", args = 0)]
    pub fn prepare_change_person(self) -> ();

    #[method(name = "ChangePerson", args = 0)]
    pub fn change_person(self) -> ();

    #[method(name = "ChangePersonAfter", args = 0)]
    pub fn change_person_after(self) -> ();

    #[method(name = "CallPersonVoice", args = 0)]
    pub fn call_person_voice(self) -> ();

    #[method(name = "ResetPerson", args = 0)]
    pub fn reset_person(self) -> ();

    #[method(name = "ChangePersonBind", args = 1)]
    pub fn change_person_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ResetPersonBind", args = 1)]
    pub fn reset_person_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ChangePersonBindNoFade", args = 1)]
    pub fn change_person_bind_no_fade(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ResetPersonBindNoFade", args = 1)]
    pub fn reset_person_bind_no_fade(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_ReplacePerson {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_ReplacePerson),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_ReplacePersonMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_RingMenu_YesItem.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.RingMenu.YesItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct HubPlayTalkAfter_RingMenu_YesItem {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_RingMenu_YesItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_RingMenu_YesItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_RingMenu_YesItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_RingMenu_YesItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_FleaMarketMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.FleaMarketMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_FleaMarketMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_FleaMarketMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_FleaMarketMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_FleaMarketMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_FleaMarketMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_GodRelianceMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.GodRelianceMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_GodRelianceMenu {
    #[rename(name = "m_access")]
    pub m_access: crate::app::hubaccess::HubAccess,
}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_GodRelianceMenu {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_GodRelianceMenu {
    pub fn new(access: crate::app::hubaccess::HubAccess) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_GodRelianceMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_GodRelianceMenuMethods>::ctor(this, access);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_FishingMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.FishingMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_FishingMenu {
    #[rename(name = "m_FishingTopMenu")]
    pub m_fishing_top_menu: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayCount")]
    pub m_play_count: i32,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
    #[rename(name = "m_gyotakuSprite")]
    pub m_gyotaku_sprite: crate::unity_engine::sprite::Sprite,
}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_FishingMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "IsEnableFishing", args = 0)]
    pub fn is_enable_fishing(self) -> bool;

    #[method(name = "SetTargetFish", args = 0)]
    pub fn set_target_fish(self) -> ();
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_FishingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_FishingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_FishingMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_RelianceMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.RelianceMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_RelianceMenu {
    #[rename(name = "m_access")]
    pub m_access: crate::app::hubaccess::HubAccess,
    #[rename(name = "m_relianceLevelUp")]
    pub m_reliance_level_up: bool,
}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_RelianceMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, access: crate::app::hubaccess::HubAccess, reliance_level_up: bool) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_RelianceMenu {
    pub fn new(access: crate::app::hubaccess::HubAccess, reliance_level_up: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_RelianceMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_RelianceMenuMethods>::ctor(this, access, reliance_level_up);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: hubplaytalkafter :: HubPlayTalkAfter >)]
pub struct HubPlayTalkAfter {
    #[static_field]
    #[rename(name = "cMenuPartsPath")]
    pub c_menu_parts_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cFishingTextureAtlasPath")]
    pub c_fishing_texture_atlas_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "MuscleMainPID")]
    pub muscle_main_pid: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FleaMarketMainPID")]
    pub flea_market_main_pid: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FishingMainPID")]
    pub fishing_main_pid: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "DragonRideMainPID")]
    pub dragon_ride_main_pid: ::unity2::Il2CppString,
    #[rename(name = "m_ReliancePopUp")]
    pub m_reliance_pop_up: crate::app::reliancepopupcontroller::ReliancePopUpController,
}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter {
    #[method(name = "get_IsExistGift", args = 0)]
    pub fn get_is_exist_gift() -> bool;

    #[method(name = "get_CurrentPersonLocator", args = 0)]
    pub fn get_current_person_locator(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_CurrentPersonLocator", args = 1)]
    pub fn set_current_person_locator(
        self,
        value: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "get_ChangeUnit", args = 0)]
    pub fn get_change_unit(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "set_ChangeUnit", args = 1)]
    pub fn set_change_unit(self, value: crate::app::hubunitcontroller::HubUnitController) -> ();

    #[method(name = "EndExercise", args = 0)]
    pub fn end_exercise(self) -> ();

    #[method(name = "get_Access", args = 0)]
    pub fn get_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "set_Access", args = 1)]
    pub fn set_access(self, value: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "get_CanAdd", args = 0)]
    pub fn get_can_add(self) -> bool;

    #[method(name = "set_CanAdd", args = 1)]
    pub fn set_can_add(self, value: bool) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "get_TargetUnit", args = 0)]
    pub fn get_target_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_TargetUnit", args = 1)]
    pub fn set_target_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: crate::app::reliancedata::RelianceData_Level) -> ();

    #[method(name = "get_Other", args = 0)]
    pub fn get_other(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Other", args = 1)]
    pub fn set_other(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ChangePersonPID", args = 0)]
    pub fn get_change_person_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ChangePersonPID", args = 1)]
    pub fn set_change_person_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsChangePerson", args = 0)]
    pub fn get_is_change_person(self) -> bool;

    #[method(name = "set_IsChangePerson", args = 1)]
    pub fn set_is_change_person(self, value: bool) -> ();

    #[method(name = "get_PrevGodLevel", args = 0)]
    pub fn get_prev_god_level(self) -> i32;

    #[method(name = "set_PrevGodLevel", args = 1)]
    pub fn set_prev_god_level(self, value: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "TryGetGodUnit", args = 1)]
    pub fn try_get_god_unit(self, gid: ::unity2::Il2CppString) -> crate::app::godunit::GodUnit;

    #[method(name = "CreateBindSelectMenu", args = 0)]
    pub fn create_bind_select_menu(self) -> ();

    #[method(name = "CreateBindSelectGift", args = 0)]
    pub fn create_bind_select_gift(self) -> ();

    #[method(name = "GetGiftMessage", args = 0)]
    pub fn get_gift_message(self) -> ::unity2::Il2CppString;

    #[method(name = "StartReactionGift", args = 0)]
    pub fn start_reaction_gift(self) -> ();

    #[method(name = "EndReactionGift", args = 0)]
    pub fn end_reaction_gift(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "ExitPhase1", args = 0)]
    pub fn exit_phase1(self) -> ();

    #[method(name = "ExitPhase2", args = 0)]
    pub fn exit_phase2(self) -> ();

    #[method(name = "StartReliance", args = 0)]
    pub fn start_reliance(self) -> ();

    #[method(name = "MainReliance", args = 0)]
    pub fn main_reliance(self) -> ();

    #[method(name = "ExitReliance", args = 0)]
    pub fn exit_reliance(self) -> ();

    #[method(name = "UpdateAchive", args = 3)]
    pub fn update_achive(
        self,
        a: crate::app::unit::Unit,
        b: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "LevelUpReliance", args = 0)]
    pub fn level_up_reliance(self) -> ();

    #[method(name = "EndReliance", args = 0)]
    pub fn end_reliance(self) -> ();

    #[method(name = "EndRelianceAfter", args = 0)]
    pub fn end_reliance_after(self) -> ();

    #[method(name = "StartReliance_God", args = 1)]
    pub fn start_reliance_god(self, gid: ::unity2::Il2CppString) -> ();

    #[method(name = "StartReliance_Unit", args = 2)]
    pub fn start_reliance_unit(
        self,
        pid: ::unity2::Il2CppString,
        other: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "StartDragonRide", args = 0)]
    pub fn start_dragon_ride(self) -> ();

    #[method(name = "CreateFleaMarket", args = 0)]
    pub fn create_flea_market(self) -> ();

    #[method(name = "CreateFortuneTelling", args = 0)]
    pub fn create_fortune_telling(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        access: crate::app::hubaccess::HubAccess,
    ) -> crate::app::procinst::ProcInst;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter {
    pub fn new(access: crate::app::hubaccess::HubAccess) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfterMethods>::ctor(this, access);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_FishingPictureBookMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.FishingPictureBookMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_FishingPictureBookMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_FishingPictureBookMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_FishingPictureBookMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_FishingPictureBookMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_FishingPictureBookMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_MuscleMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.MuscleMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_MuscleMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_MuscleMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_MuscleMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_MuscleMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_MuscleMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_RingMenu_NoItem.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.RingMenu.NoItem")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct HubPlayTalkAfter_RingMenu_NoItem {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_RingMenu_NoItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_RingMenu_NoItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_RingMenu_NoItem),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_RingMenu_NoItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_RingMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.RingMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_RingMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_RingMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_RingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_RingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_RingMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_FortuneTellingMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.FortuneTellingMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_FortuneTellingMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_FortuneTellingMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_FortuneTellingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_FortuneTellingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_FortuneTellingMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_DragonRideMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.DragonRideMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_DragonRideMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_DragonRideMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_DragonRideMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_DragonRideMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_DragonRideMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_TalkGiftMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.TalkGiftMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_TalkGiftMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_TalkGiftMenu {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_TalkGiftMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_TalkGiftMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_TalkGiftMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_SelectListMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.SelectListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct HubPlayTalkAfter_SelectListMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_SelectListMenu {
    #[method(name = "get_CurrentMenuSelect", args = 0)]
    pub fn get_current_menu_select() -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        super_: crate::app::procinst::ProcInst,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        access: crate::app::hubaccess::HubAccess,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_SelectListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        super_: crate::app::procinst::ProcInst,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_SelectListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_SelectListMenuMethods>::ctor(this, menu_item_list, super_);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubplaytalkafter/HubPlayTalkAfter_CookingMenu.md")))]
#[::unity2::class(namespace = "App", name = "HubPlayTalkAfter.CookingMenu")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct HubPlayTalkAfter_CookingMenu {}

#[cfg(feature = "app-hubplaytalkafter")]
#[::unity2::methods]
impl HubPlayTalkAfter_CookingMenu {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-hubplaytalkafter")]
impl HubPlayTalkAfter_CookingMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubPlayTalkAfter_CookingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IHubPlayTalkAfter_CookingMenuMethods>::ctor(this);
        this
    }
}
