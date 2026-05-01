
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/camerapositiondata/CameraPositionData.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraPositionData")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CameraPositionData {
    #[rename(name = "ShakeSetting")]
    pub shake_setting: crate::combat::camerapositiondata::CameraPositionData_CameraShakeSettings,
    #[rename(name = "m_DrawCount")]
    pub m_draw_count: crate::app::gameparam::GameParam_Holder,
}

#[cfg(feature = "combat-camerapositiondata")]
#[::unity2::methods]
impl CameraPositionData {
    #[method(name = "get_IsReverse", args = 0)]
    pub fn get_is_reverse(self) -> bool;

    #[method(name = "set_IsReverse", args = 1)]
    pub fn set_is_reverse(self, value: bool) -> ();

    #[method(name = "get_BattleCenter", args = 0)]
    pub fn get_battle_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_BattleCenter", args = 1)]
    pub fn set_battle_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_BattleVector", args = 0)]
    pub fn get_battle_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_BattleVector", args = 1)]
    pub fn set_battle_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_BattleNormalVector", args = 0)]
    pub fn get_battle_normal_vector(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_BattleNormalVector", args = 1)]
    pub fn set_battle_normal_vector(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_BattleDistance", args = 0)]
    pub fn get_battle_distance(self) -> f32;

    #[method(name = "set_BattleDistance", args = 1)]
    pub fn set_battle_distance(self, value: f32) -> ();

    #[method(name = "get_BasePosition", args = 0)]
    pub fn get_base_position(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_BasePosition", args = 1)]
    pub fn set_base_position(
        self,
        value: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "get_CharaArray", args = 0)]
    pub fn get_chara_array(self) -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "set_CharaArray", args = 1)]
    pub fn set_chara_array(self, value: ::unity2::Array<crate::combat::character::Character>)
        -> ();

    #[method(name = "get_m_DeadHeight", args = 0)]
    pub fn get_m_dead_height(self) -> ::unity2::Array<f32>;

    #[method(name = "set_m_DeadHeight", args = 1)]
    pub fn set_m_dead_height(self, value: ::unity2::Array<f32>) -> ();

    #[method(name = "get_UnitTall", args = 0)]
    pub fn get_unit_tall(self) -> ::unity2::Array<f32>;

    #[method(name = "set_UnitTall", args = 1)]
    pub fn set_unit_tall(self, value: ::unity2::Array<f32>) -> ();

    #[method(name = "get_JointList", args = 0)]
    pub fn get_joint_list(self) -> ::unity2::Array<crate::combat::characterjoint::CharacterJoint>;

    #[method(name = "set_JointList", args = 1)]
    pub fn set_joint_list(
        self,
        value: ::unity2::Array<crate::combat::characterjoint::CharacterJoint>,
    ) -> ();

    #[method(name = "get_IsBeforeDragonChange", args = 0)]
    pub fn get_is_before_dragon_change(self) -> ::unity2::Array<bool>;

    #[method(name = "set_IsBeforeDragonChange", args = 1)]
    pub fn set_is_before_dragon_change(self, value: ::unity2::Array<bool>) -> ();

    #[method(name = "get_IsInitialized", args = 0)]
    pub fn get_is_initialized(self) -> bool;

    #[method(name = "get_IsCharacterLoadFinished", args = 0)]
    pub fn get_is_character_load_finished(self) -> bool;

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "UpdateImpl", args = 0)]
    pub fn update_impl(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize_2(self, chara: ::unity2::Array<crate::combat::character::Character>) -> ();

    #[method(name = "GetCenter", args = 1)]
    pub fn get_center(self, side: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GeTall", args = 1)]
    pub fn ge_tall(self, side: i32) -> f32;

    #[method(name = "LoadTall", args = 1)]
    pub fn load_tall(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "LoadTall", args = 1)]
    pub fn load_tall_2(
        self,
        chara_array: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "LoadJoint", args = 0)]
    pub fn load_joint(self) -> ();

    #[method(name = "CulculateInfo", args = 2)]
    pub fn culculate_info(
        self,
        is_initialize: bool,
        record: crate::combat::combatrecord::CombatRecord,
    ) -> ();

    #[method(name = "GetJointPos", args = 2)]
    pub fn get_joint_pos(
        self,
        side: i32,
        target: crate::combat::camerapositiondata::CameraPositionData_TargetJoint,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetJoint", args = 2)]
    pub fn get_joint(
        self,
        side: i32,
        t: crate::combat::camerapositiondata::CameraPositionData_TargetJoint,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_LookAtStacks", args = 0)]
    pub fn get_look_at_stacks(
        self,
    ) -> ::unity2::Array<
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    >;

    #[method(name = "get_Lines", args = 0)]
    pub fn get_lines(self) -> ::unity2::Array<crate::unity_engine::linerenderer::LineRenderer>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-camerapositiondata")]
impl CameraPositionData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraPositionData),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraPositionDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/camerapositiondata/CameraPositionData_CameraShakeSettings.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraPositionData.CameraShakeSettings")]
#[parent(crate::system::object::Object)]
pub struct CameraPositionData_CameraShakeSettings {
    #[rename(name = "ShakeDuration")]
    pub shake_duration: i32,
    #[rename(name = "ShakeRadius")]
    pub shake_radius: f32,
    #[rename(name = "ArmorStepDuraton")]
    pub armor_step_duraton: i32,
    #[rename(name = "ArmorStepMagnitude")]
    pub armor_step_magnitude: f32,
    #[rename(name = "ScaleCurveDistance")]
    pub scale_curve_distance: crate::unity_engine::animationcurve::AnimationCurve,
}

#[cfg(feature = "combat-camerapositiondata")]
#[::unity2::methods]
impl CameraPositionData_CameraShakeSettings {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-camerapositiondata")]
impl CameraPositionData_CameraShakeSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraPositionData_CameraShakeSettings),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraPositionData_CameraShakeSettingsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/camerapositiondata/CameraPositionData_TargetJoint.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CameraPositionData_TargetJoint {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CameraPositionData_TargetJoint {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "CameraPositionData.TargetJoint";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CameraPositionData_TargetJoint {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CameraPositionData_TargetJoint {
    pub fn center() -> Self {
        Self { value: 0 }
    }

    pub fn look_at_loc() -> Self {
        Self { value: 1 }
    }

    pub fn neck_jnt() -> Self {
        Self { value: 2 }
    }

    pub fn head_loc() -> Self {
        Self { value: 3 }
    }

    pub fn look_at_loc_human() -> Self {
        Self { value: 4 }
    }
}
