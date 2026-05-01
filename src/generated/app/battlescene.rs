
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::pool::IPool_Node;
use crate::app::pool::Pool_Node;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlescene/BattleScene.md")))]
#[::unity2::class(namespace = "App", name = "BattleScene")]
#[parent(crate::app::pool::Pool_Node)]
pub struct BattleScene {
    #[rename(name = "m_List")]
    pub m_list: crate::app::battlescenelist::BattleSceneList,
    #[rename(name = "m_Side")]
    pub m_side: crate::app::battleside::BattleSide_Type,
    #[rename(name = "m_Target")]
    pub m_target: crate::app::battleside::BattleSide_Type,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::battlescene::BattleScene_Kind,
    #[rename(name = "m_Skill")]
    pub m_skill: i32,
    #[rename(name = "m_Item")]
    pub m_item: i32,
    #[rename(name = "m_God")]
    pub m_god: i32,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::battlescene::BattleScene_FieldResult,
    #[rename(name = "m_Guardian")]
    pub m_guardian: crate::app::battleside::BattleSide_Type,
    #[rename(name = "m_Hps")]
    pub m_hps: crate::app::battleside::BattleSide_ShortArray,
    #[rename(name = "m_Engages")]
    pub m_engages: crate::app::battleside::BattleSide_SbyteArray,
    #[rename(name = "m_Damages")]
    pub m_damages: crate::app::battleside::BattleSide_ShortArray,
}

#[cfg(feature = "app-battlescene")]
#[::unity2::methods]
impl BattleScene {
    #[method(name = "get_DebuggerDisplay", args = 0)]
    pub fn get_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "Setup", args = 6)]
    pub fn setup(
        self,
        list: crate::app::battlescenelist::BattleSceneList,
        kind: crate::app::battlescene::BattleScene_Kind,
        current: crate::app::battleside::BattleSide_Type,
        target: crate::app::battleside::BattleSide_Type,
        info: crate::app::battleinfo::BattleInfo,
        index: i32,
    ) -> ();

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::battlescene::BattleScene_Kind;

    #[method(name = "GetIndex", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::app::battlescene::BattleScene_FieldResult;

    #[method(name = "IsResult", args = 1)]
    pub fn is_result(self, result: crate::app::battlescene::BattleScene_Result) -> bool;

    #[method(name = "SetResult", args = 1)]
    pub fn set_result(
        self,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "SetKind", args = 1)]
    pub fn set_kind(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "GetDamage", args = 1)]
    pub fn get_damage(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "GetHeal", args = 1)]
    pub fn get_heal(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "SetDamage", args = 2)]
    pub fn set_damage(
        self,
        side: crate::app::battleside::BattleSide_Type,
        damage: i32,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "GetHp", args = 1)]
    pub fn get_hp(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "AddHp", args = 2)]
    pub fn add_hp(self, side: crate::app::battleside::BattleSide_Type, value: i32) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "GetEngageCount", args = 1)]
    pub fn get_engage_count(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "GetClampDamage", args = 1)]
    pub fn get_clamp_damage(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "SetSkill", args = 1)]
    pub fn set_skill(self, skill: i32) -> crate::app::battlescene::BattleScene;

    #[method(name = "SetSkill", args = 1)]
    pub fn set_skill_2(
        self,
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "SetSkill", args = 1)]
    pub fn set_skill_3(self, sid: ::unity2::Il2CppString) -> crate::app::battlescene::BattleScene;

    #[method(name = "GetSkill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "SetItem", args = 1)]
    pub fn set_item(self, item: i32) -> crate::app::battlescene::BattleScene;

    #[method(name = "SetItem", args = 1)]
    pub fn set_item_2(
        self,
        item: crate::app::itemdata::ItemData,
    ) -> crate::app::battlescene::BattleScene;

    #[method(name = "SetItem", args = 1)]
    pub fn set_item_3(self, iid: ::unity2::Il2CppString) -> crate::app::battlescene::BattleScene;

    #[method(name = "GetItem", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "SetGod", args = 1)]
    pub fn set_god(self, god: crate::app::goddata::GodData)
        -> crate::app::battlescene::BattleScene;

    #[method(name = "GetGod", args = 0)]
    pub fn get_god(self) -> crate::app::goddata::GodData;

    #[method(name = "GetShowSkill", args = 0)]
    pub fn get_show_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "GetSkillNameForDebuggerDisplay", args = 0)]
    pub fn get_skill_name_for_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Side", args = 0)]
    pub fn get_side(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "get_Target", args = 0)]
    pub fn get_target(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "get_Guardian", args = 0)]
    pub fn get_guardian(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "set_Guardian", args = 1)]
    pub fn set_guardian(self, value: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "get_Reverse", args = 0)]
    pub fn get_reverse(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "IsEntry", args = 1)]
    pub fn is_entry(self, side: crate::app::battleside::BattleSide_Type) -> bool;
}

#[cfg(feature = "app-battlescene")]
impl BattleScene {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleScene),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleSceneMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlescene/BattleScene_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleScene_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleScene_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleScene.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleScene_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleScene_Kind {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn attack() -> Self {
        Self { value: 1 }
    }

    pub fn rod() -> Self {
        Self { value: 2 }
    }

    pub fn dance() -> Self {
        Self { value: 3 }
    }

    pub fn skill() -> Self {
        Self { value: 4 }
    }

    pub fn give_direct() -> Self {
        Self { value: 5 }
    }

    pub fn give_delay() -> Self {
        Self { value: 6 }
    }

    pub fn strip() -> Self {
        Self { value: 7 }
    }

    pub fn equip() -> Self {
        Self { value: 8 }
    }

    pub fn god() -> Self {
        Self { value: 9 }
    }

    pub fn dead() -> Self {
        Self { value: 10 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 11 }
    }

    pub fn separator() -> Self {
        Self { value: 12 }
    }

    pub fn push_battle() -> Self {
        Self { value: 13 }
    }

    pub fn push_order() -> Self {
        Self { value: 14 }
    }

    pub fn push_action() -> Self {
        Self { value: 15 }
    }

    pub fn push_attack() -> Self {
        Self { value: 16 }
    }

    pub fn pop_attack() -> Self {
        Self { value: 17 }
    }

    pub fn pop_action() -> Self {
        Self { value: 18 }
    }

    pub fn pop_order() -> Self {
        Self { value: 19 }
    }

    pub fn pop_battle() -> Self {
        Self { value: 20 }
    }

    pub fn heal() -> Self {
        Self { value: 21 }
    }

    pub fn num() -> Self {
        Self { value: 22 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlescene/BattleScene_FieldResult.md")))]
#[::unity2::class(namespace = "App", name = "BattleScene.FieldResult")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: battlescene :: BattleScene_Result >)]
pub struct BattleScene_FieldResult {}

#[cfg(feature = "app-battlescene")]
#[::unity2::methods]
impl BattleScene_FieldResult {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::battlescene::BattleScene_Result) -> i32;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        self_: crate::app::battlescene::BattleScene_FieldResult,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlescene")]
impl BattleScene_FieldResult {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleScene_FieldResult),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleScene_FieldResultMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlescene/BattleScene_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleScene_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleScene_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleScene.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleScene_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleScene_Result {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn hit() -> Self {
        Self { value: 1 }
    }

    pub fn critical() -> Self {
        Self { value: 2 }
    }

    pub fn guard() -> Self {
        Self { value: 4 }
    }

    pub fn suicide() -> Self {
        Self { value: 8 }
    }

    pub fn efficacy() -> Self {
        Self { value: 16 }
    }

    pub fn r#break() -> Self {
        Self { value: 32 }
    }

    pub fn blow() -> Self {
        Self { value: 64 }
    }

    pub fn bounce() -> Self {
        Self { value: 128 }
    }

    pub fn chain_attack() -> Self {
        Self { value: 256 }
    }

    pub fn chain_guard() -> Self {
        Self { value: 512 }
    }

    pub fn dual_guard() -> Self {
        Self { value: 1024 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 2048 }
    }

    pub fn physical() -> Self {
        Self { value: 4096 }
    }

    pub fn magic() -> Self {
        Self { value: 8192 }
    }

    pub fn ignore() -> Self {
        Self { value: 16384 }
    }
}
