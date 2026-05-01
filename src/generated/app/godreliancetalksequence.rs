
use crate::app::commonreliancetalksequence::CommonRelianceTalkSequence;
use crate::app::commonreliancetalksequence::ICommonRelianceTalkSequence;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godreliancetalksequence/GodRelianceTalkSequence.md")))]
#[::unity2::class(namespace = "App", name = "GodRelianceTalkSequence")]
#[parent(crate::app::commonreliancetalksequence::CommonRelianceTalkSequence)]
pub struct GodRelianceTalkSequence {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitAsciiName")]
    pub m_unit_ascii_name: ::unity2::Il2CppString,
    #[rename(name = "m_God")]
    pub m_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_GodAsciiName")]
    pub m_god_ascii_name: ::unity2::Il2CppString,
    #[rename(name = "m_RelianceLevel")]
    pub m_reliance_level: crate::app::goddata::GodData_RelianceLevel,
}

#[cfg(feature = "app-godreliancetalksequence")]
#[::unity2::methods]
impl GodRelianceTalkSequence {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "CreateMessFileName", args = 1)]
    pub fn create_mess_file_name(self, is_reverse: bool) -> ::unity2::Il2CppString;

    #[method(name = "CreateMessFileName", args = 2)]
    pub fn create_mess_file_name_2(
        self,
        ascii_name_a: ::unity2::Il2CppString,
        ascii_name_b: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateMid", args = 0)]
    pub fn create_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRelianceLevelText", args = 1)]
    pub fn get_reliance_level_text(
        self,
        reliance_level: crate::app::goddata::GodData_RelianceLevel,
    ) -> ::unity2::Il2CppString;

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();
}

#[cfg(feature = "app-godreliancetalksequence")]
impl GodRelianceTalkSequence {
    pub fn new(unit: crate::app::unit::Unit, god: crate::app::godunit::GodUnit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRelianceTalkSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRelianceTalkSequenceMethods>::ctor(this, unit, god);
        this
    }
}
