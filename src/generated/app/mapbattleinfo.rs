
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinfo/MapBattleInfo.md")))]
#[::unity2::class(namespace = "App", name = "MapBattleInfo")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapbattleinfo :: MapBattleInfo >)]
pub struct MapBattleInfo {
    #[rename(name = "m_Calculator")]
    pub m_calculator: crate::app::battlecalculator::BattleCalculator,
}

#[cfg(feature = "app-mapbattleinfo")]
#[::unity2::methods]
impl MapBattleInfo {
    #[method(name = "SetCalculator", args = 1)]
    pub fn set_calculator(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> crate::app::mapbattleinfo::MapBattleInfo;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbattleinfo")]
impl MapBattleInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBattleInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBattleInfoMethods>::ctor(this);
        this
    }
}
