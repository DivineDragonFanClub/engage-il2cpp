
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitactor/UnitActor_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitActor_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitActor_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitActor.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitActor_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitActor_Status {
    pub fn stasis() -> Self {
        Self { value: 1 }
    }

    pub fn active() -> Self {
        Self { value: 2 }
    }

    pub fn moved() -> Self {
        Self { value: 4 }
    }

    pub fn reload() -> Self {
        Self { value: 8 }
    }

    pub fn god_in() -> Self {
        Self { value: 16 }
    }

    pub fn god_out() -> Self {
        Self { value: 32 }
    }

    pub fn pass_height() -> Self {
        Self { value: 64 }
    }

    pub fn has_effect() -> Self {
        Self { value: 128 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitactor/UnitActor.md")))]
#[::unity2::class(namespace = "App", name = "UnitActor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UnitActor {
    #[rename(name = "m_Models")]
    pub m_models: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitModel")]
    pub m_unit_model: crate::app::unitmodel::UnitModel,
    #[rename(name = "m_GodModel")]
    pub m_god_model: crate::app::unitmodel::UnitModel,
    #[rename(name = "m_Effect")]
    pub m_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::unitactor::UnitActor_StatusField,
    #[rename(name = "m_Binder")]
    pub m_binder: crate::app::bindholder::BindHolder,
    #[rename(name = "m_Position")]
    pub m_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Rotation")]
    pub m_rotation: crate::app::interpolatorrotation::InterpolatorRotation,
    #[rename(name = "m_SlopeRotation")]
    pub m_slope_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_SlopeOffset")]
    pub m_slope_offset: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Moving")]
    pub m_moving: crate::app::interpolatorvector3::InterpolatorVector3,
    #[static_field]
    #[rename(name = "SHAKE_COUNT")]
    pub shake_count: i32,
    #[rename(name = "m_ShakeCount")]
    pub m_shake_count: i32,
    #[rename(name = "m_ShakeScale")]
    pub m_shake_scale: f32,
    #[rename(name = "m_ShakeOffset")]
    pub m_shake_offset: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-unitactor")]
#[::unity2::methods]
impl UnitActor {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_UnitModel", args = 0)]
    pub fn get_unit_model(self) -> crate::app::unitmodel::UnitModel;

    #[method(name = "get_GodModel", args = 0)]
    pub fn get_god_model(self) -> crate::app::unitmodel::UnitModel;

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "get_Sound", args = 0)]
    pub fn get_sound(self) -> crate::app::assettable::AssetTable_Sound;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_Position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetCellCenterPosition", args = 0)]
    pub fn get_cell_center_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetTargetPosition", args = 0)]
    pub fn get_target_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetTargetX", args = 0)]
    pub fn get_target_x(self) -> i32;

    #[method(name = "GetTargetZ", args = 0)]
    pub fn get_target_z(self) -> i32;

    #[method(name = "get_IsMoving", args = 0)]
    pub fn get_is_moving(self) -> bool;

    #[method(name = "Move", args = 2)]
    pub fn r#move(self, pos: crate::unity_engine::vector3::Vector3, time: f32) -> ();

    #[method(name = "GetStartPos", args = 0)]
    pub fn get_start_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetGoalPos", args = 0)]
    pub fn get_goal_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetRate", args = 0)]
    pub fn get_rate(self) -> f32;

    #[method(name = "GetAssetMode", args = 0)]
    pub fn get_asset_mode() -> crate::app::assettable::AssetTable_Modes;

    #[method(name = "GetGodUnit", args = 1)]
    pub fn get_god_unit(unit: crate::app::unit::Unit) -> crate::app::godunit::GodUnit;

    #[method(name = "GetHauntUnit", args = 1)]
    pub fn get_haunt_unit(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "TryCreateUnitModel", args = 0)]
    pub fn try_create_unit_model(self) -> ();

    #[method(name = "TryCreateGodUnitModel", args = 1)]
    pub fn try_create_god_unit_model(self, reverse: bool) -> ();

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, conditions: ::unity2::Array<::unity2::Il2CppString>) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "PreTick", args = 0)]
    pub fn pre_tick(self) -> ();

    #[method(name = "PostTick", args = 0)]
    pub fn post_tick(self) -> ();

    #[method(name = "UpdateEffect", args = 0)]
    pub fn update_effect(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "UpdateLoading", args = 0)]
    pub fn update_loading(self) -> ();

    #[method(name = "Reload", args = 0)]
    pub fn reload(self) -> ();

    #[method(name = "UpdateStatus", args = 0)]
    pub fn update_status(self) -> ();

    #[method(name = "UpdateMoving", args = 0)]
    pub fn update_moving(self) -> ();

    #[method(name = "UpdateRotate", args = 0)]
    pub fn update_rotate(self) -> ();

    #[method(name = "UpdateShake", args = 0)]
    pub fn update_shake(self) -> ();

    #[method(name = "GetAnim", args = 0)]
    pub fn get_anim(self) -> crate::app::unitanim::UnitAnim_Types;

    #[method(name = "GetIdleAnim", args = 0)]
    pub fn get_idle_anim(self) -> crate::app::unitanim::UnitAnim_Types;

    #[method(name = "GetAnimator", args = 0)]
    pub fn get_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "SetIdleAnim", args = 2)]
    pub fn set_idle_anim(
        self,
        r#type: crate::app::unitanim::UnitAnim_Types,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "SetIdleAnim", args = 2)]
    pub fn set_idle_anim_2(
        self,
        action: crate::app::unitsequence::UnitSequence_Action,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "PlayAction", args = 1)]
    pub fn play_action(self, action: crate::app::unitsequence::UnitSequence_Action) -> ();

    #[method(name = "PlayCancel", args = 1)]
    pub fn play_cancel(self, action: crate::app::unitsequence::UnitSequence_Action) -> ();

    #[method(name = "TryPlayAction", args = 1)]
    pub fn try_play_action(self, action: crate::app::unitsequence::UnitSequence_Action) -> bool;

    #[method(name = "ResetIdleAnim", args = 0)]
    pub fn reset_idle_anim(self) -> ();

    #[method(name = "PlayIdle", args = 1)]
    pub fn play_idle(self, time: crate::app::unitanim::UnitAnim_Times) -> ();

    #[method(name = "PlayIdle", args = 2)]
    pub fn play_idle_2(
        self,
        action: crate::app::unitsequence::UnitSequence_Action,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "PlayAnim", args = 2)]
    pub fn play_anim(
        self,
        r#type: crate::app::unitanim::UnitAnim_Types,
        time: crate::app::unitanim::UnitAnim_Times,
    ) -> ();

    #[method(name = "PlaySkill", args = 2)]
    pub fn play_skill(
        self,
        skill: crate::app::skilldata::SkillData,
        kind: crate::app::effectsequence::EffectSequence_Kind,
    ) -> bool;

    #[method(name = "PlaySkill", args = 5)]
    pub fn play_skill_2(
        self,
        skill: crate::app::skilldata::SkillData,
        kind: crate::app::effectsequence::EffectSequence_Kind,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        is_caption: bool,
    ) -> bool;

    #[method(name = "PlayActiveSkill", args = 1)]
    pub fn play_active_skill(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "PlayShootSkill", args = 1)]
    pub fn play_shoot_skill(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "PlayShootSkill", args = 3)]
    pub fn play_shoot_skill_2(
        self,
        skill: crate::app::skilldata::SkillData,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "PlayEffect", args = 1)]
    pub fn play_effect(self, eid: ::unity2::Il2CppString) -> bool;

    #[method(name = "PlayEffect", args = 1)]
    pub fn play_effect_2(self, data: crate::app::effectdata::EffectData) -> bool;

    #[method(name = "PlayEffect", args = 3)]
    pub fn play_effect_3(
        self,
        data: crate::app::effectdata::EffectData,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "PlayEngageCount", args = 4)]
    pub fn play_engage_count(
        self,
        start: crate::unity_engine::vector3::Vector3,
        goal: crate::unity_engine::vector3::Vector3,
        count: i32,
        delay_time: f32,
    ) -> ();

    #[method(name = "GetEffectPosition", args = 0)]
    pub fn get_effect_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetEffectRotation", args = 0)]
    pub fn get_effect_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "PlayHitSkill", args = 1)]
    pub fn play_hit_skill(self, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "SetSpeed", args = 1)]
    pub fn set_speed(self, speed: f32) -> ();

    #[method(name = "GetSpeed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "ActionBind", args = 0)]
    pub fn action_bind(self) -> ();

    #[method(name = "ActionUnbind", args = 0)]
    pub fn action_unbind(self) -> ();

    #[method(name = "IsActionBind", args = 0)]
    pub fn is_action_bind(self) -> bool;

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(self, status: crate::app::unitactor::UnitActor_Status) -> ();

    #[method(name = "ClearStatus", args = 1)]
    pub fn clear_status(self, status: crate::app::unitactor::UnitActor_Status) -> ();

    #[method(name = "CheckStatus", args = 1)]
    pub fn check_status(self, status: crate::app::unitactor::UnitActor_Status) -> bool;

    #[method(name = "CommitPosition", args = 0)]
    pub fn commit_position(self) -> ();

    #[method(name = "CommitRotation", args = 0)]
    pub fn commit_rotation(self) -> ();

    #[method(name = "UpdatePosition", args = 0)]
    pub fn update_position(self) -> ();

    #[method(name = "get_CellX", args = 0)]
    pub fn get_cell_x(self) -> i32;

    #[method(name = "get_CellZ", args = 0)]
    pub fn get_cell_z(self) -> i32;

    #[method(name = "get_SizeOffset", args = 0)]
    pub fn get_size_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_BmapSize", args = 0)]
    pub fn get_bmap_size(self) -> i32;

    #[method(name = "Rotation", args = 1)]
    pub fn rotation(self, angle: f32) -> ();

    #[method(name = "Rotation", args = 1)]
    pub fn rotation_2(self, dir: crate::app::dir_2::Dir_Type) -> ();

    #[method(name = "Cross", args = 2)]
    pub fn cross(
        v1: crate::unity_engine::vector2::Vector2,
        v2: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "Rotation", args = 1)]
    pub fn rotation_3(self, dir: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Rotation", args = 2)]
    pub fn rotation_4(self, x: i32, z: i32) -> ();

    #[method(name = "Rotation", args = 1)]
    pub fn rotation_5(self, target: crate::app::unit::Unit) -> ();

    #[method(name = "SetSlope", args = 2)]
    pub fn set_slope(
        self,
        rotation: crate::unity_engine::quaternion::Quaternion,
        offset: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetAngleLength", args = 2)]
    pub fn get_angle_length(current: f32, target: f32) -> f32;

    #[method(name = "IsRider", args = 0)]
    pub fn is_rider(self) -> bool;

    #[method(name = "GetFrontAngle", args = 0)]
    pub fn get_front_angle(self) -> f32;

    #[method(name = "GetFrontRange", args = 0)]
    pub fn get_front_range(self) -> f32;

    #[method(name = "GetFrontDelta", args = 2)]
    pub fn get_front_delta(self, angle: f32, range: f32) -> f32;

    #[method(name = "GetDirection", args = 1)]
    pub fn get_direction(angle: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "RotationFixed", args = 0)]
    pub fn rotation_fixed(self) -> ();

    #[method(name = "get_Direction", args = 0)]
    pub fn get_direction_2(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InstantRotation", args = 0)]
    pub fn instant_rotation(self) -> ();

    #[method(name = "Shake", args = 1)]
    pub fn shake(self, scale: f32) -> ();

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "ImpactShine", args = 0)]
    pub fn impact_shine(self) -> ();

    #[method(name = "HealShine", args = 0)]
    pub fn heal_shine(self) -> ();

    #[method(name = "SkillShine", args = 0)]
    pub fn skill_shine(self) -> ();

    #[method(name = "Shine", args = 1)]
    pub fn shine(self, time: f32) -> ();

    #[method(name = "BrightOn", args = 0)]
    pub fn bright_on(self) -> ();

    #[method(name = "BrightOff", args = 0)]
    pub fn bright_off(self) -> ();

    #[method(name = "FadeIn", args = 1)]
    pub fn fade_in(self, time: f32) -> ();

    #[method(name = "FadeOut", args = 1)]
    pub fn fade_out(self, time: f32) -> ();

    #[method(name = "GodIn", args = 0)]
    pub fn god_in(self) -> ();

    #[method(name = "GodOut", args = 0)]
    pub fn god_out(self) -> ();

    #[method(name = "IsOutSight", args = 3)]
    pub fn is_out_sight(
        x: i32,
        z: i32,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
    ) -> bool;

    #[method(name = "IsMoveSkip", args = 4)]
    pub fn is_move_skip(
        x: i32,
        z: i32,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> bool;

    #[method(name = "MoveRoute", args = 3)]
    pub fn move_route(
        self,
        super_: crate::app::procinst::ProcInst,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();

    #[method(name = "MoveRoute", args = 5)]
    pub fn move_route_2(
        self,
        super_: crate::app::procinst::ProcInst,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        from_x: i32,
        from_z: i32,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();

    #[method(name = "MoveRouteInstant", args = 2)]
    pub fn move_route_instant(
        self,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();

    #[method(name = "MoveRouteInstant", args = 4)]
    pub fn move_route_instant_2(
        self,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        from_x: i32,
        from_z: i32,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();

    #[method(name = "UpdateMoved", args = 1)]
    pub fn update_moved(self, flag: crate::app::mapmoveflag::MapMoveFlag) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitactor")]
impl UnitActor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitActor),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitActorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitactor/UnitActor_ViewMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitActor_ViewMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitActor_ViewMode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitActor.ViewMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitActor_ViewMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitActor_ViewMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn hide() -> Self {
        Self { value: 1 }
    }

    pub fn show() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitactor/UnitActor_StatusField.md")))]
#[::unity2::class(namespace = "App", name = "UnitActor.StatusField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: unitactor :: UnitActor_Status >)]
pub struct UnitActor_StatusField {}

#[cfg(feature = "app-unitactor")]
#[::unity2::methods]
impl UnitActor_StatusField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::unitactor::UnitActor_Status) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitactor")]
impl UnitActor_StatusField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitActor_StatusField),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitActor_StatusFieldMethods>::ctor(this);
        this
    }
}
