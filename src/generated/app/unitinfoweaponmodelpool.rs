
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitinfoweaponmodelpool/UnitInfoWeaponModelPool.md")))]
#[::unity2::class(namespace = "App", name = "UnitInfoWeaponModelPool")]
#[parent(crate::system::object::Object)]
pub struct UnitInfoWeaponModelPool {
    #[rename(name = "m_LeftWeaponDictionary")]
    pub m_left_weapon_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::combat::characterasset::CharacterAsset,
    >,
    #[rename(name = "m_RightWeaponDictionary")]
    pub m_right_weapon_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::combat::characterasset::CharacterAsset,
    >,
    #[static_field]
    #[rename(name = "m_ShopWeaponAsset")]
    pub m_shop_weapon_asset: crate::combat::characterasset::CharacterAsset,
    #[static_field]
    #[rename(name = "m_ShopWeaponAssetNewest")]
    pub m_shop_weapon_asset_newest: crate::combat::characterasset::CharacterAsset,
    #[static_field]
    #[rename(name = "m_ShopWeaponItemNewest")]
    pub m_shop_weapon_item_newest: crate::app::unititem::UnitItem,
}

#[cfg(feature = "app-unitinfoweaponmodelpool")]
#[::unity2::methods]
impl UnitInfoWeaponModelPool {
    #[method(name = "GetAndLoadWeapon", args = 3)]
    pub fn get_and_load_weapon(
        self,
        unit: crate::app::unit::Unit,
        equipped: crate::app::unititem::UnitItem,
        asset_type: crate::combat::assettype::AssetType,
    ) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "GetAndLoadWeaponShop", args = 2)]
    pub fn get_and_load_weapon_shop(
        self,
        equipped: crate::app::unititem::UnitItem,
        asset_type: crate::combat::assettype::AssetType,
    ) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "AddLeft", args = 2)]
    pub fn add_left(
        self,
        unit: crate::app::unit::Unit,
        equipped: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "AddRight", args = 2)]
    pub fn add_right(
        self,
        unit: crate::app::unit::Unit,
        equipped: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "TryGetLeft", args = 1)]
    pub fn try_get_left(
        self,
        equipped: crate::app::unititem::UnitItem,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryGetRight", args = 1)]
    pub fn try_get_right(
        self,
        equipped: crate::app::unititem::UnitItem,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryActiveLeft", args = 1)]
    pub fn try_active_left(self, equipped: crate::app::unititem::UnitItem) -> ();

    #[method(name = "TryActiveRight", args = 1)]
    pub fn try_active_right(self, equipped: crate::app::unititem::UnitItem) -> ();

    #[method(name = "SetShopWeapon", args = 1)]
    pub fn set_shop_weapon(self, equipped: crate::app::unititem::UnitItem) -> ();

    #[method(name = "ClearShopWeapon", args = 0)]
    pub fn clear_shop_weapon() -> ();

    #[method(name = "IsShopWeapon", args = 0)]
    pub fn is_shop_weapon() -> bool;

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, callback: crate::system::action::Action) -> ();

    #[method(name = "WaitLoaded", args = 0)]
    pub fn wait_loaded(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded(self) -> bool;

    #[method(name = "Delete", args = 0)]
    pub fn delete(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-unitinfoweaponmodelpool")]
impl UnitInfoWeaponModelPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitInfoWeaponModelPool),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitInfoWeaponModelPoolMethods>::ctor(this);
        this
    }
}
