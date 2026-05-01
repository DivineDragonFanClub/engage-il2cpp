
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence.md")))]
#[::unity2::class(namespace = "App", name = "WellSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct WellSequence {
    #[static_field]
    #[rename(name = "UseFlagName")]
    pub use_flag_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ExchangeLevelName")]
    pub exchange_level_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SeedName")]
    pub seed_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "EvilWeaponState")]
    pub evil_weapon_state: ::unity2::Il2CppString,
    #[rename(name = "m_TopMenuResult")]
    pub m_top_menu_result: crate::app::welltopmenu::WellTopMenu_MenuResult,
    #[static_field]
    #[rename(name = "m_WellEffectManager")]
    pub m_well_effect_manager: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "EFF_NAME_TABLE")]
    pub eff_name_table: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-wellsequence")]
#[::unity2::methods]
impl WellSequence {
    #[method(name = "CreateGlobalFlags", args = 0)]
    pub fn create_global_flags() -> ();

    #[method(name = "CalcExpected", args = 1)]
    pub fn calc_expected(
        unit_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unititem::UnitItem,
        >,
    ) -> i32;

    #[method(name = "get_Variable", args = 0)]
    pub fn get_variable() -> crate::app::gamevariable::GameVariable;

    #[method(name = "get_UseFlag", args = 0)]
    pub fn get_use_flag() -> crate::app::wellsequence::WellSequence_UseFlags;

    #[method(name = "set_UseFlag", args = 1)]
    pub fn set_use_flag(value: crate::app::wellsequence::WellSequence_UseFlags) -> ();

    #[method(name = "get_ExchangeLevel", args = 0)]
    pub fn get_exchange_level() -> i32;

    #[method(name = "set_ExchangeLevel", args = 1)]
    pub fn set_exchange_level(value: i32) -> ();

    #[method(name = "get_Seed", args = 0)]
    pub fn get_seed() -> i32;

    #[method(name = "set_Seed", args = 1)]
    pub fn set_seed(value: i32) -> ();

    #[method(name = "get_EvilWeaponEventState", args = 0)]
    pub fn get_evil_weapon_event_state(
    ) -> crate::app::wellsequence::WellSequence_EvilWeaponEventStates;

    #[method(name = "set_EvilWeaponEventState", args = 1)]
    pub fn set_evil_weapon_event_state(
        value: crate::app::wellsequence::WellSequence_EvilWeaponEventStates,
    ) -> ();

    #[method(name = "SetExchange", args = 1)]
    pub fn set_exchange(level: i32) -> ();

    #[method(name = "MapClear", args = 0)]
    pub fn map_clear() -> ();

    #[method(name = "get_IsItemReturn", args = 0)]
    pub fn get_is_item_return() -> bool;

    #[method(name = "get_CanItemIn", args = 0)]
    pub fn get_can_item_in() -> bool;

    #[method(name = "GetEffectName", args = 0)]
    pub fn get_effect_name() -> ::unity2::Il2CppString;

    #[method(name = "TryCreateItemEffect", args = 0)]
    pub fn try_create_item_effect() -> ();

    #[method(name = "TryFadeoutEffect", args = 0)]
    pub fn try_fadeout_effect() -> ();

    #[method(name = "TryDestroyEffect", args = 0)]
    pub fn try_destroy_effect() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(parent: crate::app::procinst::ProcInst) -> ();

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async(self) -> ();

    #[method(name = "WaitLoadPrefab", args = 0)]
    pub fn wait_load_prefab(self) -> ();

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab(self) -> ();

    #[method(name = "OpenHeader", args = 0)]
    pub fn open_header(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CalcItemExchange", args = 2)]
    pub fn calc_item_exchange(
        self,
        level: i32,
        random: crate::app::random_2::Random_2,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "GetItem", args = 0)]
    pub fn get_item(self) -> ();

    #[method(name = "CheckCcTutorial", args = 0)]
    pub fn check_cc_tutorial(self) -> ();

    #[method(name = "CreateItemSelectMenu", args = 0)]
    pub fn create_item_select_menu(self) -> ();

    #[method(name = "CheckItemSelectResult", args = 0)]
    pub fn check_item_select_result(self) -> ();

    #[method(name = "CreateGotoEvilFirstConfirmDialog", args = 0)]
    pub fn create_goto_evil_first_confirm_dialog(self) -> ();

    #[method(name = "CreateGotoEvilConfirmDialog", args = 0)]
    pub fn create_goto_evil_confirm_dialog(self) -> ();

    #[method(name = "OnChangeNormal", args = 0)]
    pub fn on_change_normal(self) -> ();

    #[method(name = "OnChangeHard", args = 0)]
    pub fn on_change_hard(self) -> ();

    #[method(name = "OnChangeLunatic", args = 0)]
    pub fn on_change_lunatic(self) -> ();

    #[method(name = "CreateChangeDifficultyDialog", args = 0)]
    pub fn create_change_difficulty_dialog(self) -> ();

    #[method(name = "CreateChangeDifficultyResultDialog", args = 0)]
    pub fn create_change_difficulty_result_dialog(self) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "GetEvilWeapon1", args = 0)]
    pub fn get_evil_weapon1(self) -> ();

    #[method(name = "GetEvilWeapon2", args = 0)]
    pub fn get_evil_weapon2(self) -> ();

    #[method(name = "EvilMapFirstDemo", args = 0)]
    pub fn evil_map_first_demo(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "FirstTutorial", args = 0)]
    pub fn first_tutorial(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-wellsequence")]
impl WellSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IWellSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence_ChangeDifficultyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "WellSequence.ChangeDifficultyMenuItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct WellSequence_ChangeDifficultyMenuItem {
    #[rename(name = "YesEventHandler")]
    pub yes_event_handler: crate::system::action::Action,
}

#[cfg(feature = "app-wellsequence")]
#[::unity2::methods]
impl WellSequence_ChangeDifficultyMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        yes_event_handler: crate::system::action::Action,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-wellsequence")]
impl WellSequence_ChangeDifficultyMenuItem {
    pub fn new(
        yes_event_handler: crate::system::action::Action,
        text: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellSequence_ChangeDifficultyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWellSequence_ChangeDifficultyMenuItemMethods>::ctor(
            this,
            yes_event_handler,
            text,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WellSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WellSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WellSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WellSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WellSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn get_evil_weapons() -> Self {
        Self { value: 1 }
    }

    pub fn evil_map_first_demo() -> Self {
        Self { value: 2 }
    }

    pub fn get_exchange_item() -> Self {
        Self { value: 3 }
    }

    pub fn top_menu() -> Self {
        Self { value: 4 }
    }

    pub fn item_select() -> Self {
        Self { value: 5 }
    }

    pub fn first_evil_map_confirm() -> Self {
        Self { value: 6 }
    }

    pub fn evil_map_confirm() -> Self {
        Self { value: 7 }
    }

    pub fn change_difficulty() -> Self {
        Self { value: 8 }
    }

    pub fn change_difficulty_result() -> Self {
        Self { value: 9 }
    }

    pub fn exit() -> Self {
        Self { value: 10 }
    }

    pub fn end() -> Self {
        Self { value: 11 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence_DialogItemGotoEvilFirst.md")))]
#[::unity2::class(namespace = "App", name = "WellSequence.DialogItemGotoEvilFirst")]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct WellSequence_DialogItemGotoEvilFirst {}

#[cfg(feature = "app-wellsequence")]
#[::unity2::methods]
impl WellSequence_DialogItemGotoEvilFirst {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, label: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-wellsequence")]
impl WellSequence_DialogItemGotoEvilFirst {
    pub fn new(label: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellSequence_DialogItemGotoEvilFirst),
                ::core::stringify!(new),
            )
        });
        <Self as IWellSequence_DialogItemGotoEvilFirstMethods>::ctor(this, label);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence_EvilWeaponEventStates.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WellSequence_EvilWeaponEventStates {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WellSequence_EvilWeaponEventStates {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WellSequence.EvilWeaponEventStates";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WellSequence_EvilWeaponEventStates {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WellSequence_EvilWeaponEventStates {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn can_get() -> Self {
        Self { value: 1 }
    }

    pub fn already_get() -> Self {
        Self { value: 2 }
    }

    pub fn already_talk() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellsequence/WellSequence_UseFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WellSequence_UseFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WellSequence_UseFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WellSequence.UseFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WellSequence_UseFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WellSequence_UseFlags {
    pub fn not_use() -> Self {
        Self { value: 0 }
    }

    pub fn used() -> Self {
        Self { value: 1 }
    }

    pub fn item_return() -> Self {
        Self { value: 2 }
    }
}
