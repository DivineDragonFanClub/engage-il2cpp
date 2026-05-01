
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsimplebattle/MapSimpleBattle.md")))]
#[::unity2::class(namespace = "App", name = "MapSimpleBattle")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapsimplebattle :: MapSimpleBattle >)]
pub struct MapSimpleBattle {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LeftSideObjName")]
    pub left_side_obj_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RightSideObjName")]
    pub right_side_obj_name: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mapsimplebattle")]
#[::unity2::methods]
impl MapSimpleBattle {
    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "SetSimpleBattle", args = 1)]
    pub fn set_simple_battle(self, calc: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "UpdateValue", args = 1)]
    pub fn update_value(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "SkipGaugeAnime", args = 1)]
    pub fn skip_gauge_anime(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "IsMoving", args = 0)]
    pub fn is_moving(self) -> bool;

    #[method(name = "IsPlayingAnim", args = 0)]
    pub fn is_playing_anim(self) -> bool;

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "Out", args = 0)]
    pub fn out(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateObjects", args = 0)]
    pub fn create_objects(self) -> ();

    #[method(name = "DestroyObjects", args = 0)]
    pub fn destroy_objects(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsimplebattle")]
impl MapSimpleBattle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSimpleBattle),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSimpleBattleMethods>::ctor(this);
        this
    }
}
