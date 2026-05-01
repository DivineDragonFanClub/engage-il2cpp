
use crate::app::pool::IPool_List_1;
use crate::app::pool::IPool_Node;
use crate::app::pool::Pool_List_1;
use crate::app::pool::Pool_Node;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeffect/MapEffect_LocationPool.md")))]
#[::unity2::class(namespace = "App", name = "MapEffect.LocationPool")]
# [parent (crate :: app :: pool :: Pool_List_1 < crate :: app :: mapeffect :: MapEffect_LocationNode >)]
pub struct MapEffect_LocationPool {}

#[cfg(feature = "app-mapeffect")]
#[::unity2::methods]
impl MapEffect_LocationPool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeffect")]
impl MapEffect_LocationPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEffect_LocationPool),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEffect_LocationPoolMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeffect/MapEffect_LocationNode.md")))]
#[::unity2::class(namespace = "App", name = "MapEffect.LocationNode")]
#[parent(crate::app::pool::Pool_Node)]
pub struct MapEffect_LocationNode {
    #[rename(name = "Name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "Position")]
    pub position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Rotation")]
    pub rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "Effect")]
    pub effect: crate::app::resourceobject::ResourceObject,
}

#[cfg(feature = "app-mapeffect")]
#[::unity2::methods]
impl MapEffect_LocationNode {
    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "IsMatch", args = 2)]
    pub fn is_match(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "TryCreateEffect", args = 0)]
    pub fn try_create_effect(self) -> ();

    #[method(name = "TryDeleteEffect", args = 0)]
    pub fn try_delete_effect(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeffect")]
impl MapEffect_LocationNode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEffect_LocationNode),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEffect_LocationNodeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeffect/MapEffect.md")))]
#[::unity2::class(namespace = "App", name = "MapEffect")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapeffect :: MapEffect >)]
pub struct MapEffect {
    #[static_field]
    #[rename(name = "s_Root")]
    pub s_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Pool")]
    pub m_pool: crate::app::mapeffect::MapEffect_LocationPool,
}

#[cfg(feature = "app-mapeffect")]
#[::unity2::methods]
impl MapEffect {
    #[method(name = "FindRoot", args = 0)]
    pub fn find_root() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryCreateRoot", args = 0)]
    pub fn try_create_root() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryDeleteRoot", args = 0)]
    pub fn try_delete_root() -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "Play", args = 2)]
    pub fn play(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "Play", args = 3)]
    pub fn play_2(name: ::unity2::Il2CppString, x: i32, z: i32) -> bool;

    #[method(name = "Play", args = 3)]
    pub fn play_3(data: crate::app::effectdata::EffectData, x: i32, z: i32) -> bool;

    #[method(name = "Play", args = 2)]
    pub fn play_4(
        data: crate::app::effectdata::EffectData,
        position: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "Play", args = 2)]
    pub fn play_5(name: ::unity2::Il2CppString, model: crate::app::unitmodel::UnitModel) -> bool;

    #[method(name = "Play", args = 2)]
    pub fn play_6(name: ::unity2::Il2CppString, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Play", args = 3)]
    pub fn play_7(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "CanPlaying", args = 0)]
    pub fn can_playing() -> bool;

    #[method(name = "Play", args = 3)]
    pub fn play_8(
        seq: crate::app::effectsequence::EffectSequence,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "Play", args = 3)]
    pub fn play_9(
        data: crate::app::effectdata::EffectData,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "Create", args = 3)]
    pub fn create(name: ::unity2::Il2CppString, x: i32, z: i32) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create_2(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Create", args = 3)]
    pub fn create_3(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Delete", args = 3)]
    pub fn delete(name: ::unity2::Il2CppString, x: i32, z: i32) -> ();

    #[method(name = "Delete", args = 2)]
    pub fn delete_2(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete_3(position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "IsExist", args = 2)]
    pub fn is_exist(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "CreateImpl", args = 3)]
    pub fn create_impl(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "DeleteImpl", args = 2)]
    pub fn delete_impl(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "IsExistImpl", args = 2)]
    pub fn is_exist_impl(
        self,
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "Resume", args = 0)]
    pub fn resume(self) -> ();

    #[method(name = "Shoot", args = 6)]
    pub fn shoot(
        name: ::unity2::Il2CppString,
        start: crate::unity_engine::vector3::Vector3,
        goal: crate::unity_engine::vector3::Vector3,
        time: f32,
        delay: f32,
        callback: crate::app::effectshoot::EffectShoot_Callback,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapeffect")]
impl MapEffect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEffect),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEffectMethods>::ctor(this);
        this
    }
}
