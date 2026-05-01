
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinfowindowsingle/MapBattleInfoWindowSingle.md")))]
#[::unity2::class(namespace = "App", name = "MapBattleInfoWindowSingle")]
#[parent(crate::system::object::Object)]
pub struct MapBattleInfoWindowSingle {
    #[rename(name = "m_MapBattleInfoParamSetter")]
    pub m_map_battle_info_param_setter:
        crate::app::mapbattleinfoparamsetter::MapBattleInfoParamSetter,
    #[rename(name = "m_SideType")]
    pub m_side_type: crate::app::battleside::BattleSide_Type,
}

#[cfg(feature = "app-mapbattleinfowindowsingle")]
#[::unity2::methods]
impl MapBattleInfoWindowSingle {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, side_type: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, game_object: crate::unity_engine::gameobject::GameObject) -> bool;

    #[method(name = "SetBattleInfo", args = 3)]
    pub fn set_battle_info(
        self,
        b_show_wdw: bool,
        info: crate::app::battleinfo::BattleInfo,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> ();

    #[method(name = "SetWeaponChangeVisible", args = 1)]
    pub fn set_weapon_change_visible(self, is_visible: bool) -> ();

    #[method(name = "IsSimple", args = 0)]
    pub fn is_simple(self) -> bool;
}

#[cfg(feature = "app-mapbattleinfowindowsingle")]
impl MapBattleInfoWindowSingle {
    pub fn new(side_type: crate::app::battleside::BattleSide_Type) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBattleInfoWindowSingle),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBattleInfoWindowSingleMethods>::ctor(this, side_type);
        this
    }
}
