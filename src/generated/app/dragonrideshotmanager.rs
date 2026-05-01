
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideshotmanager/DragonRideShotManager.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideShotManager")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: dragonrideshotmanager :: DragonRideShotManager >)]
pub struct DragonRideShotManager {
    #[rename(name = "m_PoolParent")]
    pub m_pool_parent: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UseCount")]
    pub m_use_count: i32,
    #[rename(name = "m_ReturnCount")]
    pub m_return_count: i32,
    #[rename(name = "m_ReservePoolArray")]
    pub m_reserve_pool_array: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
    #[rename(name = "MaxShotCount")]
    pub max_shot_count: i32,
    #[rename(name = "m_DefaultInterpSecond")]
    pub m_default_interp_second: f32,
    #[rename(name = "m_DefaultShotSpeed")]
    pub m_default_shot_speed: f32,
    #[rename(name = "m_DefaultLifeSecond")]
    pub m_default_life_second: f32,
    #[rename(name = "m_IsPenetrate")]
    pub m_is_penetrate: bool,
}

#[cfg(feature = "app-dragonrideshotmanager")]
#[::unity2::methods]
impl DragonRideShotManager {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Destruct", args = 0)]
    pub fn destruct(self) -> ();

    #[method(name = "SetDefaultParam", args = 4)]
    pub fn set_default_param(
        self,
        interp_second: f32,
        shot_speed: f32,
        life_second: f32,
        is_penetrate: bool,
    ) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "InactiveAllShot", args = 0)]
    pub fn inactive_all_shot(self) -> ();

    #[method(name = "SetActiveShot", args = 8)]
    pub fn set_active_shot(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        straight: crate::unity_engine::vector3::Vector3,
        diff: crate::unity_engine::vector3::Vector3,
        is_assist: bool,
        is_special_time: bool,
        is_penetrate: bool,
        is_maximum: bool,
    ) -> ();

    #[method(name = "ResetShot", args = 1)]
    pub fn reset_shot(self, obj: crate::unity_engine::gameobject::GameObject) -> ();
}

#[cfg(feature = "app-dragonrideshotmanager")]
impl DragonRideShotManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideShotManager),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideShotManagerMethods>::ctor(this);
        this
    }
}
