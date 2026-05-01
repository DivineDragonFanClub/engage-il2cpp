
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcollisions/MapCollisions.md")))]
#[::unity2::class(namespace = "App", name = "MapCollisions")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapcollisions :: MapCollisions >)]
pub struct MapCollisions {
    #[static_field]
    #[rename(name = "HeightOffset")]
    pub height_offset: f32,
    #[static_field]
    #[rename(name = "CombatBorders")]
    pub combat_borders: ::unity2::Il2CppString,
}

#[cfg(feature = "app-mapcollisions")]
#[::unity2::methods]
impl MapCollisions {
    #[method(name = "CreateBorder", args = 5)]
    pub fn create_border(
        prefab: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
        pos: crate::unity_engine::vector3::Vector3,
        scale: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetMinHeight", args = 2)]
    pub fn get_min_height(x: i32, z: i32) -> f32;

    #[method(name = "CreateBorder", args = 5)]
    pub fn create_border_2(
        prefab: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "CreateEdge", args = 6)]
    pub fn create_edge(
        prefab: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
        x: i32,
        z: i32,
        mask: crate::app::dir_2::Dir_Type,
    ) -> ();

    #[method(name = "CreateChild", args = 1)]
    pub fn create_child(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "CreteaCombatBorders", args = 1)]
    pub fn cretea_combat_borders(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "CreteaCombatBorders", args = 2)]
    pub fn cretea_combat_borders_2(
        self,
        can_sky_battle: bool,
        allow_side: crate::app::battleinfoside::BattleInfoSide,
    ) -> ();

    #[method(name = "DeleteCombatBorders", args = 0)]
    pub fn delete_combat_borders(self) -> ();

    #[method(name = "TryCreateInstance", args = 0)]
    pub fn try_create_instance() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapcollisions")]
impl MapCollisions {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCollisions),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCollisionsMethods>::ctor(this);
        this
    }
}
