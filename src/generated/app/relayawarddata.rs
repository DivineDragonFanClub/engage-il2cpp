
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData.md")))]
#[::unity2::class(namespace = "App", name = "RelayAwardData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: relayawarddata :: RelayAwardData >)]
pub struct RelayAwardData {
    #[static_field]
    #[rename(name = "s_Infos")]
    pub s_infos: ::unity2::Array<crate::app::relayawarddata::RelayAwardData_Info>,
}

#[cfg(feature = "app-relayawarddata")]
#[::unity2::methods]
impl RelayAwardData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Raid", args = 0)]
    pub fn get_raid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Raid", args = 1)]
    pub fn set_raid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ResultText", args = 0)]
    pub fn get_result_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ResultText", args = 1)]
    pub fn set_result_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Awards", args = 0)]
    pub fn get_awards(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Awards", args = 1)]
    pub fn set_awards(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::relayawarddata::RelayAwardData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::relayawarddata::RelayAwardData_FlagField) -> ();

    #[method(name = "GetRaid", args = 1)]
    pub fn get_raid_2(
        kind: crate::app::relayawarddata::RelayAwardData_Kinds,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetRecordKind", args = 1)]
    pub fn get_record_kind(
        kind: crate::app::relayawarddata::RelayAwardData_Kinds,
    ) -> crate::app::unitrecord::UnitRecord_Kinds;

    #[method(name = "GetCompareOp", args = 1)]
    pub fn get_compare_op(
        kind: crate::app::relayawarddata::RelayAwardData_Kinds,
    ) -> crate::app::relayawarddata::RelayAwardData_CompareOp;

    #[method(name = "Test", args = 2)]
    pub fn test(
        kind: crate::app::relayawarddata::RelayAwardData_Kinds,
        flag: crate::app::relayawarddata::RelayAwardData_Flags,
    ) -> bool;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(
        kind: crate::app::relayawarddata::RelayAwardData_Kinds,
    ) -> crate::app::relayawarddata::RelayAwardData;

    #[method(name = "OnCompletedEnd", args = 0)]
    pub fn on_completed_end(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "MakeInfos", args = 0)]
    pub fn make_infos(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relayawarddata")]
impl RelayAwardData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAwardData),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayAwardDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData_Info.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RelayAwardData_Info {
    pub raid: ::unity2::Il2CppString,
    pub record_kind: crate::app::unitrecord::UnitRecord_Kinds,
    pub compare_op: crate::app::relayawarddata::RelayAwardData_CompareOp,
}

impl ::unity2::ClassIdentity for RelayAwardData_Info {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAwardData.Info";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAwardData_Info {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-relayawarddata")]
#[::unity2::methods(value)]
impl RelayAwardData_Info {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        raid: ::unity2::Il2CppString,
        kind: crate::app::unitrecord::UnitRecord_Kinds,
        comp: crate::app::relayawarddata::RelayAwardData_CompareOp,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayAwardData_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayAwardData_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAwardData.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAwardData_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayAwardData_Kinds {
    pub fn kill() -> Self {
        Self { value: 0 }
    }

    pub fn damaged() -> Self {
        Self { value: 1 }
    }

    pub fn heal() -> Self {
        Self { value: 2 }
    }

    pub fn r#break() -> Self {
        Self { value: 3 }
    }

    pub fn engage() -> Self {
        Self { value: 4 }
    }

    pub fn use_item() -> Self {
        Self { value: 5 }
    }

    pub fn poison() -> Self {
        Self { value: 6 }
    }

    pub fn r#move() -> Self {
        Self { value: 7 }
    }

    pub fn skill() -> Self {
        Self { value: 8 }
    }

    pub fn damaged0() -> Self {
        Self { value: 9 }
    }

    pub fn critical() -> Self {
        Self { value: 10 }
    }

    pub fn efficacy() -> Self {
        Self { value: 11 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 12 }
    }

    pub fn chain_guard() -> Self {
        Self { value: 13 }
    }

    pub fn blow() -> Self {
        Self { value: 14 }
    }

    pub fn direct_attack() -> Self {
        Self { value: 15 }
    }

    pub fn indirect_attack() -> Self {
        Self { value: 16 }
    }

    pub fn attack() -> Self {
        Self { value: 17 }
    }

    pub fn chain_attack() -> Self {
        Self { value: 18 }
    }

    pub fn mixed_least() -> Self {
        Self { value: 19 }
    }

    pub fn move_least() -> Self {
        Self { value: 20 }
    }

    pub fn damage_least() -> Self {
        Self { value: 21 }
    }

    pub fn fixed() -> Self {
        Self { value: 22 }
    }

    pub fn healed() -> Self {
        Self { value: 23 }
    }

    pub fn skilled() -> Self {
        Self { value: 24 }
    }

    pub fn num() -> Self {
        Self { value: 25 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "RelayAwardData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: relayawarddata :: RelayAwardData_Flags >)]
pub struct RelayAwardData_FlagField {}

#[cfg(feature = "app-relayawarddata")]
#[::unity2::methods]
impl RelayAwardData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::relayawarddata::RelayAwardData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::relayawarddata::RelayAwardData_Flags) -> i32;
}

#[cfg(feature = "app-relayawarddata")]
impl RelayAwardData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAwardData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayAwardData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::relayawarddata::RelayAwardData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayAwardData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRelayAwardData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayAwardData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayAwardData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAwardData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAwardData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayAwardData_Flags {
    pub fn mixed() -> Self {
        Self { value: 1 }
    }

    pub fn show_count() -> Self {
        Self { value: 2 }
    }

    pub fn negative() -> Self {
        Self { value: 4 }
    }

    pub fn attack_plus() -> Self {
        Self { value: 8 }
    }

    pub fn attack_minus() -> Self {
        Self { value: 16 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawarddata/RelayAwardData_CompareOp.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayAwardData_CompareOp {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayAwardData_CompareOp {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayAwardData.CompareOp";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayAwardData_CompareOp {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayAwardData_CompareOp {
    pub fn greater() -> Self {
        Self { value: 0 }
    }

    pub fn less() -> Self {
        Self { value: 1 }
    }

    pub fn zero() -> Self {
        Self { value: 2 }
    }

    pub fn mixed_less() -> Self {
        Self { value: 3 }
    }

    pub fn attack_greater_kill_less() -> Self {
        Self { value: 4 }
    }
}
