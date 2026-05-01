
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skilleditroot/SkillEditRoot.md")))]
#[::unity2::class(namespace = "App", name = "SkillEditRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SkillEditRoot {
    #[rename(name = "m_EquipSkillList")]
    pub m_equip_skill_list: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PoolSkillList")]
    pub m_pool_skill_list: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillInfo")]
    pub m_skill_info: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillInfoRoot")]
    pub m_skill_info_root: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-skilleditroot")]
#[::unity2::methods]
impl SkillEditRoot {
    #[method(name = "GetGameObjectEquip", args = 0)]
    pub fn get_game_object_equip(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetGameObjectPool", args = 0)]
    pub fn get_game_object_pool(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetSkillInfo", args = 0)]
    pub fn get_skill_info(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetSkillInfoRoot", args = 0)]
    pub fn get_skill_info_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-skilleditroot")]
impl SkillEditRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillEditRoot),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillEditRootMethods>::ctor(this);
        this
    }
}
