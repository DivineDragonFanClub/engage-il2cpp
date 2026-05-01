
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameicon/GameIcon.md")))]
#[::unity2::class(namespace = "App", name = "GameIcon")]
#[parent(crate::system::object::Object)]
pub struct GameIcon {
    #[static_field]
    #[rename(name = "s_Skill")]
    pub s_skill: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_Item")]
    pub s_item: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_Efficacy")]
    pub s_efficacy: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_EfficacyOutline")]
    pub s_efficacy_outline: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_ItemKinds")]
    pub s_item_kinds: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_ItemOutlineKinds")]
    pub s_item_outline_kinds: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_GodSymbol")]
    pub s_god_symbol: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_GodRing")]
    pub s_god_ring: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_System")]
    pub s_system: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_UnitIconIndex")]
    pub s_unit_icon_index: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_UnitIconPallete")]
    pub s_unit_icon_pallete: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-gameicon")]
#[::unity2::methods]
impl GameIcon {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "TryGetItemKind", args = 2)]
    pub fn try_get_item_kind(
        icon_name: ::unity2::Il2CppString,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItemKind", args = 2)]
    pub fn try_get_item_kind_2(
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItemKind", args = 3)]
    pub fn try_get_item_kind_3(
        item_kind: crate::app::itemdata::ItemData_Kinds,
        is_bullet: bool,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItemKind", args = 3)]
    pub fn try_get_item_kind_4(
        item_kind: crate::app::itemdata::ItemData_Kinds,
        job_data: crate::app::jobdata::JobData,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItemKind", args = 2)]
    pub fn try_get_item_kind_5(
        item_data: crate::app::itemdata::ItemData,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItem", args = 1)]
    pub fn try_get_item(icon_name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetItem", args = 1)]
    pub fn try_get_item_2(
        item: crate::app::itemdata::ItemData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetSystemItem", args = 1)]
    pub fn try_get_system_item(
        item: crate::app::itemdata::ItemData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetEfficacy", args = 2)]
    pub fn try_get_efficacy(
        icon_label: ::unity2::Il2CppString,
        is_outline: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetSkill", args = 1)]
    pub fn try_get_skill(
        skill_icon_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetSkillEmpty", args = 0)]
    pub fn try_get_skill_empty() -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetGodSymbol", args = 1)]
    pub fn try_get_god_symbol(
        god_data: crate::app::goddata::GodData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetGodRing", args = 1)]
    pub fn try_get_god_ring(unit: crate::app::unit::Unit) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetGodRing", args = 1)]
    pub fn try_get_god_ring_2(
        god_data: crate::app::goddata::GodData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetGodRing", args = 1)]
    pub fn try_get_god_ring_3(
        ring: crate::app::ringdata::RingData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetGodRing", args = 1)]
    pub fn try_get_god_ring_4(
        rank: crate::app::ringdata::RingData_Ranks,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetAccessoryKinds", args = 1)]
    pub fn try_get_accessory_kinds(
        accessory_kinds: crate::app::accessorydata::AccessoryData_Kinds,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetCommonRing", args = 1)]
    pub fn try_get_common_ring(
        ring_data_rank: crate::app::ringdata::RingData_Ranks,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetSystem", args = 1)]
    pub fn try_get_system(icon_name: ::unity2::Il2CppString)
        -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TyrGetUnitIconIndex", args = 1)]
    pub fn tyr_get_unit_icon_index(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TyrGetUnitIconPallete", args = 1)]
    pub fn tyr_get_unit_icon_pallete(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "ClearUnitIconCache", args = 0)]
    pub fn clear_unit_icon_cache() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameicon")]
impl GameIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IGameIconMethods>::ctor(this);
        this
    }
}
