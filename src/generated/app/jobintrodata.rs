
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrodata/JobIntroData.md")))]
#[::unity2::class(namespace = "App", name = "JobIntroData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: jobintrodata :: JobIntroData >)]
pub struct JobIntroData {}

#[cfg(feature = "app-jobintrodata")]
#[::unity2::methods]
impl JobIntroData {
    #[method(name = "get_DLC", args = 0)]
    pub fn get_dlc(self) -> bool;

    #[method(name = "set_DLC", args = 1)]
    pub fn set_dlc(self, value: bool) -> ();

    #[method(name = "get_Person", args = 0)]
    pub fn get_person(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Person", args = 1)]
    pub fn set_person(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Job", args = 0)]
    pub fn get_job(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Job", args = 1)]
    pub fn set_job(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Weapon", args = 0)]
    pub fn get_weapon(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Weapon", args = 1)]
    pub fn set_weapon(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Enemy", args = 0)]
    pub fn get_enemy(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Enemy", args = 1)]
    pub fn set_enemy(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_EnemyJob", args = 0)]
    pub fn get_enemy_job(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_EnemyJob", args = 1)]
    pub fn set_enemy_job(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_EnemyWeapon", args = 0)]
    pub fn get_enemy_weapon(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_EnemyWeapon", args = 1)]
    pub fn set_enemy_weapon(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_Distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_Field", args = 0)]
    pub fn get_field(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Field", args = 1)]
    pub fn set_field(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack", args = 0)]
    pub fn get_attack(self) -> crate::app::jobintrodata::JobIntroData_BattleType;

    #[method(name = "set_Attack", args = 1)]
    pub fn set_attack(self, value: crate::app::jobintrodata::JobIntroData_BattleType) -> ();

    #[method(name = "get_Counter", args = 0)]
    pub fn get_counter(self) -> crate::app::jobintrodata::JobIntroData_BattleType;

    #[method(name = "set_Counter", args = 1)]
    pub fn set_counter(self, value: crate::app::jobintrodata::JobIntroData_BattleType) -> ();

    #[method(name = "get_Pursuit", args = 0)]
    pub fn get_pursuit(self) -> crate::app::jobintrodata::JobIntroData_BattleType;

    #[method(name = "set_Pursuit", args = 1)]
    pub fn set_pursuit(self, value: crate::app::jobintrodata::JobIntroData_BattleType) -> ();

    #[method(name = "Load", args = 1)]
    pub fn load(progress: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jobintrodata")]
impl JobIntroData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JobIntroData),
                ::core::stringify!(new),
            )
        });
        <Self as IJobIntroDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jobintrodata/JobIntroData_BattleType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct JobIntroData_BattleType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for JobIntroData_BattleType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "JobIntroData.BattleType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for JobIntroData_BattleType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl JobIntroData_BattleType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn attack1() -> Self {
        Self { value: 1 }
    }

    pub fn attack2() -> Self {
        Self { value: 2 }
    }

    pub fn attack3() -> Self {
        Self { value: 3 }
    }

    pub fn attack4() -> Self {
        Self { value: 4 }
    }

    pub fn attack5() -> Self {
        Self { value: 5 }
    }

    pub fn miss() -> Self {
        Self { value: 6 }
    }

    pub fn parry() -> Self {
        Self { value: 7 }
    }

    pub fn guard() -> Self {
        Self { value: 8 }
    }

    pub fn heal() -> Self {
        Self { value: 9 }
    }

    pub fn finish() -> Self {
        Self { value: 10 }
    }
}
