
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponlevels/WeaponLevels.md")))]
#[::unity2::class(namespace = "App", name = "WeaponLevels")]
#[parent(crate::system::object::Object)]
pub struct WeaponLevels {
    #[rename(name = "m_Levels")]
    pub m_levels: ::unity2::Array<i8>,
}

#[cfg(feature = "app-weaponlevels")]
#[::unity2::methods]
impl WeaponLevels {
    #[method(name = "get_None", args = 0)]
    pub fn get_none(self) -> i8;

    #[method(name = "set_None", args = 1)]
    pub fn set_none(self, value: i8) -> ();

    #[method(name = "get_Sword", args = 0)]
    pub fn get_sword(self) -> i8;

    #[method(name = "set_Sword", args = 1)]
    pub fn set_sword(self, value: i8) -> ();

    #[method(name = "get_Lance", args = 0)]
    pub fn get_lance(self) -> i8;

    #[method(name = "set_Lance", args = 1)]
    pub fn set_lance(self, value: i8) -> ();

    #[method(name = "get_Axe", args = 0)]
    pub fn get_axe(self) -> i8;

    #[method(name = "set_Axe", args = 1)]
    pub fn set_axe(self, value: i8) -> ();

    #[method(name = "get_Bow", args = 0)]
    pub fn get_bow(self) -> i8;

    #[method(name = "set_Bow", args = 1)]
    pub fn set_bow(self, value: i8) -> ();

    #[method(name = "get_Dagger", args = 0)]
    pub fn get_dagger(self) -> i8;

    #[method(name = "set_Dagger", args = 1)]
    pub fn set_dagger(self, value: i8) -> ();

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> i8;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: i8) -> ();

    #[method(name = "get_Rod", args = 0)]
    pub fn get_rod(self) -> i8;

    #[method(name = "set_Rod", args = 1)]
    pub fn set_rod(self, value: i8) -> ();

    #[method(name = "get_Fist", args = 0)]
    pub fn get_fist(self) -> i8;

    #[method(name = "set_Fist", args = 1)]
    pub fn set_fist(self, value: i8) -> ();

    #[method(name = "get_Special", args = 0)]
    pub fn get_special(self) -> i8;

    #[method(name = "set_Special", args = 1)]
    pub fn set_special(self, value: i8) -> ();

    #[method(name = "SetImpl", args = 2)]
    pub fn set_impl(self, kind: crate::app::itemdata::ItemData_Kinds, value: i8) -> ();

    #[method(name = "GetImpl", args = 1)]
    pub fn get_impl(self, kind: crate::app::itemdata::ItemData_Kinds) -> i8;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
    ) -> crate::app::weaponlevel::WeaponLevel_Kind;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        kind: crate::app::itemdata::ItemData_Kinds,
        value: crate::app::weaponlevel::WeaponLevel_Kind,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::weaponlevels::WeaponLevels) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, levels: crate::app::weaponlevels::WeaponLevels) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-weaponlevels")]
impl WeaponLevels {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponLevels),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponLevelsMethods>::ctor(this);
        this
    }
}
