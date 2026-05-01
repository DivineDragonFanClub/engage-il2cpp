
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitgrowsequence/UnitGrowSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitGrowSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitGrowSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitGrowSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitGrowSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitGrowSequence_Label {
    pub fn level_up_start() -> Self {
        Self { value: 0 }
    }

    pub fn level_up_end() -> Self {
        Self { value: 1 }
    }

    pub fn class_change_start() -> Self {
        Self { value: 2 }
    }

    pub fn class_change_end() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitgrowsequence/UnitGrowSequence.md")))]
#[::unity2::class(namespace = "App", name = "UnitGrowSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct UnitGrowSequence {
    #[rename(name = "m_CameraMode")]
    pub m_camera_mode: crate::app::viewmode::ViewMode_Mode,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Exp")]
    pub m_exp: i32,
    #[rename(name = "m_OldLevel")]
    pub m_old_level: i32,
    #[rename(name = "m_IsTalk")]
    pub m_is_talk: bool,
    #[rename(name = "m_SkillPoint")]
    pub m_skill_point: i32,
    #[rename(name = "m_ClassChangeJob")]
    pub m_class_change_job: crate::app::jobdata::JobData,
    #[rename(name = "m_ClassChangeItem")]
    pub m_class_change_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_ClassChangeWeaponMask")]
    pub m_class_change_weapon_mask: crate::app::weaponmask::WeaponMask,
    #[rename(name = "m_ClassChangeWeapon")]
    pub m_class_change_weapon: crate::app::itemdata::ItemData,
}

#[cfg(feature = "app-unitgrowsequence")]
#[::unity2::methods]
impl UnitGrowSequence {
    #[method(name = "setUnitGrowData", args = 2)]
    pub fn set_unit_grow_data(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        is_talk: bool,
    ) -> ();

    #[method(name = "setUnitGrowData", args = 2)]
    pub fn set_unit_grow_data_2(self, unit: crate::app::unit::Unit, exp: i32) -> ();

    #[method(name = "setUnitGrowData", args = 4)]
    pub fn set_unit_grow_data_3(
        self,
        unit: crate::app::unit::Unit,
        exp: i32,
        skill_point: i32,
        is_talk: bool,
    ) -> ();

    #[method(name = "setUnitClassChange", args = 3)]
    pub fn set_unit_class_change(
        self,
        unit: crate::app::unit::Unit,
        job: crate::app::jobdata::JobData,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "setUnitClassChange", args = 2)]
    pub fn set_unit_class_change_2(
        self,
        unit: crate::app::unit::Unit,
        data: crate::app::classchange::ClassChange_ChangeJobData,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::unitgrowsequence::UnitGrowSequence;

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "GainExp", args = 0)]
    pub fn gain_exp(self) -> ();

    #[method(name = "CheckLevelUp", args = 0)]
    pub fn check_level_up(self) -> ();

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();

    #[method(name = "CheckClassChange", args = 0)]
    pub fn check_class_change(self) -> ();

    #[method(name = "ClassChange", args = 0)]
    pub fn class_change(self) -> ();

    #[method(name = "SetWeapon", args = 0)]
    pub fn set_weapon(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitgrowsequence")]
impl UnitGrowSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitGrowSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitGrowSequenceMethods>::ctor(this);
        this
    }
}
