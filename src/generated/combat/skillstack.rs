
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/skillstack/SkillStack_Packet.md")))]
#[::unity2::class(namespace = "Combat", name = "SkillStack.Packet")]
#[parent(crate::system::object::Object)]
pub struct SkillStack_Packet {}

#[cfg(feature = "combat-skillstack")]
#[::unity2::methods]
impl SkillStack_Packet {
    #[method(name = "get_FromSide", args = 0)]
    pub fn get_from_side(self) -> i32;

    #[method(name = "set_FromSide", args = 1)]
    pub fn set_from_side(self, value: i32) -> ();

    #[method(name = "get_ToSide", args = 0)]
    pub fn get_to_side(self) -> i32;

    #[method(name = "set_ToSide", args = 1)]
    pub fn set_to_side(self, value: i32) -> ();

    #[method(name = "ChangeSideFromTo_For絆神竜破", args = 2)]
    pub fn change_side_from_to_for____(self, old: i32, new: i32) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_Data", args = 1)]
    pub fn set_data(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, from_side: i32, to_side: i32, data: crate::app::skilldata::SkillData) -> ();

    #[method(name = "IsEqualTo", args = 3)]
    pub fn is_equal_to(
        self,
        from_side: i32,
        to_side: i32,
        data: crate::app::skilldata::SkillData,
    ) -> bool;
}

#[cfg(feature = "combat-skillstack")]
impl SkillStack_Packet {
    pub fn new(from_side: i32, to_side: i32, data: crate::app::skilldata::SkillData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillStack_Packet),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillStack_PacketMethods>::ctor(this, from_side, to_side, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/skillstack/SkillStack.md")))]
#[::unity2::class(namespace = "Combat", name = "SkillStack")]
#[parent(crate::system::object::Object)]
pub struct SkillStack {
    #[rename(name = "list")]
    pub list: crate::system::collections::generic::list_1::List_1<
        crate::combat::skillstack::SkillStack_Packet,
    >,
}

#[cfg(feature = "combat-skillstack")]
#[::unity2::methods]
impl SkillStack {
    #[method(name = "get_DebuggerDisplay", args = 0)]
    pub fn get_debugger_display(self) -> ::unity2::Il2CppString;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<
        crate::combat::skillstack::SkillStack_Packet,
    >;

    #[method(name = "Damages", args = 1)]
    pub fn damages(self, side: i32) -> i32;

    #[method(name = "HasDamage", args = 0)]
    pub fn has_damage(self) -> bool;

    #[method(name = "PushSkill", args = 5)]
    pub fn push_skill(
        self,
        data: crate::app::skilldata::SkillData,
        from_side: i32,
        to_side: i32,
        pl_damage: i32,
        en_damage: i32,
    ) -> ();

    #[method(name = "ClearDamages", args = 0)]
    pub fn clear_damages(self) -> ();

    #[method(name = "CloneAndClearIfNotNull", args = 0)]
    pub fn clone_and_clear_if_not_null(self) -> crate::combat::skillstack::SkillStack;

    #[method(name = "CreatePhaseForPopup", args = 1)]
    pub fn create_phase_for_popup(
        self,
        src: crate::combat::phase::Phase,
    ) -> crate::combat::phase::Phase;

    #[method(name = "PreloadForAssetLoader", args = 0)]
    pub fn preload_for_asset_loader(self) -> ();

    #[method(name = "CalcFreezeDuration", args = 0)]
    pub fn calc_freeze_duration(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-skillstack")]
impl SkillStack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillStack),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillStackMethods>::ctor(this);
        this
    }
}
