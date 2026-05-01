
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapplayerunit/GmapPlayerUnit.md")))]
#[::unity2::class(namespace = "App", name = "GmapPlayerUnit")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: gmapplayerunit :: GmapPlayerUnit >)]
pub struct GmapPlayerUnit {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "ShipPrefabPath")]
    pub ship_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_Ship")]
    pub m_ship: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "ColliderPrefabPath")]
    pub collider_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "TeleportInEffect")]
    pub teleport_in_effect: ::unity2::Il2CppString,
    #[rename(name = "TeleportOutEffect")]
    pub teleport_out_effect: ::unity2::Il2CppString,
    #[rename(name = "m_Rotation")]
    pub m_rotation: crate::app::interpolatorrotation::InterpolatorRotation,
    #[rename(name = "m_GroundObjectNames")]
    pub m_ground_object_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "SeaObjectNames")]
    pub sea_object_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "IgnoreObjectNames")]
    pub ignore_object_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "PositionDelta")]
    pub position_delta: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-gmapplayerunit")]
#[::unity2::methods]
impl GmapPlayerUnit {
    #[method(name = "get_Actor", args = 0)]
    pub fn get_actor(self) -> crate::app::unitactor::UnitActor;

    #[method(name = "get_PlayerModelType", args = 0)]
    pub fn get_player_model_type(self) -> crate::app::gmapplayerunit::GmapPlayerUnit_ModelType;

    #[method(name = "set_PlayerModelType", args = 1)]
    pub fn set_player_model_type(
        self,
        value: crate::app::gmapplayerunit::GmapPlayerUnit_ModelType,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "UpdateRotation", args = 0)]
    pub fn update_rotation(self) -> ();

    #[method(name = "CommitRotation", args = 0)]
    pub fn commit_rotation(self) -> ();

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_Up", args = 0)]
    pub fn get_up(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_Up", args = 1)]
    pub fn set_up(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_Rotation", args = 1)]
    pub fn set_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "SetDirection", args = 2)]
    pub fn set_direction(
        self,
        dir: crate::unity_engine::vector3::Vector3,
        up: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetRotate", args = 1)]
    pub fn set_rotate(self, angle: f32) -> ();

    #[method(name = "SetRotateImmidiate", args = 1)]
    pub fn set_rotate_immidiate(self, angle: f32) -> ();

    #[method(name = "GetAngleLength", args = 2)]
    pub fn get_angle_length(current: f32, target: f32) -> f32;

    #[method(name = "SetUnitFromUnitPool", args = 0)]
    pub fn set_unit_from_unit_pool(self) -> ();

    #[method(name = "LoadActor", args = 0)]
    pub fn load_actor(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "UnloadUnit", args = 0)]
    pub fn unload_unit(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "SetupModelType", args = 0)]
    pub fn setup_model_type(self) -> ();

    #[method(name = "ChangeModelType", args = 1)]
    pub fn change_model_type(
        self,
        model_type: crate::app::gmapplayerunit::GmapPlayerUnit_ModelType,
    ) -> ();

    #[method(name = "StopRun", args = 0)]
    pub fn stop_run(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "SetMapTransparent", args = 1)]
    pub fn set_map_transparent(self, active: bool) -> ();

    #[method(name = "IsOnGround", args = 0)]
    pub fn is_on_ground(self) -> bool;

    #[method(name = "PlayTeleportOut", args = 0)]
    pub fn play_teleport_out(self) -> ();

    #[method(name = "PlayTeleportIn", args = 0)]
    pub fn play_teleport_in(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapplayerunit")]
impl GmapPlayerUnit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapPlayerUnit),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapPlayerUnitMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapplayerunit/GmapPlayerUnit_ModelType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapPlayerUnit_ModelType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapPlayerUnit_ModelType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapPlayerUnit.ModelType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapPlayerUnit_ModelType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapPlayerUnit_ModelType {
    pub fn unit() -> Self {
        Self { value: 0 }
    }

    pub fn ship() -> Self {
        Self { value: 1 }
    }
}
