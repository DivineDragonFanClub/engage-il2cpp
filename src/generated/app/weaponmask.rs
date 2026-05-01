
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponmask/WeaponMask_Flag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WeaponMask_Flag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WeaponMask_Flag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WeaponMask.Flag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WeaponMask_Flag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WeaponMask_Flag {
    pub fn none() -> Self {
        Self { value: 1 }
    }

    pub fn sword() -> Self {
        Self { value: 2 }
    }

    pub fn lance() -> Self {
        Self { value: 4 }
    }

    pub fn axe() -> Self {
        Self { value: 8 }
    }

    pub fn bow() -> Self {
        Self { value: 16 }
    }

    pub fn dagger() -> Self {
        Self { value: 32 }
    }

    pub fn magic() -> Self {
        Self { value: 64 }
    }

    pub fn rod() -> Self {
        Self { value: 128 }
    }

    pub fn fist() -> Self {
        Self { value: 256 }
    }

    pub fn special() -> Self {
        Self { value: 512 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponmask/WeaponMask.md")))]
#[::unity2::class(namespace = "App", name = "WeaponMask")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: weaponmask :: WeaponMask_Flag >)]
pub struct WeaponMask {}

#[cfg(feature = "app-weaponmask")]
#[::unity2::methods]
impl WeaponMask {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, f: crate::app::weaponmask::WeaponMask_Flag) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::weaponmask::WeaponMask_Flag) -> i32;

    #[method(name = "Test", args = 1)]
    pub fn test(self, item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_2(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "Set", args = 1)]
    pub fn set(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, kind: crate::app::itemdata::ItemData_Kinds) -> ();

    #[method(name = "CanEquipFromKind", args = 1)]
    pub fn can_equip_from_kind(self, kind: crate::app::itemdata::ItemData_Kinds) -> bool;

    #[method(name = "get_Exist", args = 0)]
    pub fn get_exist(self) -> bool;

    #[method(name = "GetRandomForArena", args = 0)]
    pub fn get_random_for_arena(self) -> crate::app::itemdata::ItemData_Kinds;
}

#[cfg(feature = "app-weaponmask")]
impl WeaponMask {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponMask),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponMaskMethods>::ctor(this);
        this
    }

    pub fn new_2(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponMask),
                ::core::stringify!(new_2),
            )
        });
        <Self as IWeaponMaskMethods>::ctor_2(this, f);
        this
    }

    pub fn new_3(f: crate::app::weaponmask::WeaponMask_Flag) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponMask),
                ::core::stringify!(new_3),
            )
        });
        <Self as IWeaponMaskMethods>::ctor_3(this, f);
        this
    }
}
