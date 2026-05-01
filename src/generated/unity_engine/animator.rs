
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animator/Animator.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Animator")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Animator {}

#[cfg(feature = "unity_engine-animator")]
#[::unity2::methods]
impl Animator {
    #[method(name = "get_isOptimizable", args = 0)]
    pub fn get_is_optimizable(self) -> bool;

    #[method(name = "get_isHuman", args = 0)]
    pub fn get_is_human(self) -> bool;

    #[method(name = "get_hasRootMotion", args = 0)]
    pub fn get_has_root_motion(self) -> bool;

    #[method(name = "get_isRootPositionOrRotationControlledByCurves", args = 0)]
    pub fn get_is_root_position_or_rotation_controlled_by_curves(self) -> bool;

    #[method(name = "get_humanScale", args = 0)]
    pub fn get_human_scale(self) -> f32;

    #[method(name = "get_isInitialized", args = 0)]
    pub fn get_is_initialized(self) -> bool;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(self, name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float_2(self, id: i32) -> f32;

    #[method(name = "SetFloat", args = 2)]
    pub fn set_float(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetFloat", args = 4)]
    pub fn set_float_2(
        self,
        name: ::unity2::Il2CppString,
        value: f32,
        damp_time: f32,
        delta_time: f32,
    ) -> ();

    #[method(name = "SetFloat", args = 2)]
    pub fn set_float_3(self, id: i32, value: f32) -> ();

    #[method(name = "SetFloat", args = 4)]
    pub fn set_float_4(self, id: i32, value: f32, damp_time: f32, delta_time: f32) -> ();

    #[method(name = "GetBool", args = 1)]
    pub fn get_bool(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetBool", args = 1)]
    pub fn get_bool_2(self, id: i32) -> bool;

    #[method(name = "SetBool", args = 2)]
    pub fn set_bool(self, name: ::unity2::Il2CppString, value: bool) -> ();

    #[method(name = "SetBool", args = 2)]
    pub fn set_bool_2(self, id: i32, value: bool) -> ();

    #[method(name = "GetInteger", args = 1)]
    pub fn get_integer(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetInteger", args = 1)]
    pub fn get_integer_2(self, id: i32) -> i32;

    #[method(name = "SetInteger", args = 2)]
    pub fn set_integer(self, name: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "SetInteger", args = 2)]
    pub fn set_integer_2(self, id: i32, value: i32) -> ();

    #[method(name = "SetTrigger", args = 1)]
    pub fn set_trigger(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetTrigger", args = 1)]
    pub fn set_trigger_2(self, id: i32) -> ();

    #[method(name = "ResetTrigger", args = 1)]
    pub fn reset_trigger(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "ResetTrigger", args = 1)]
    pub fn reset_trigger_2(self, id: i32) -> ();

    #[method(name = "IsParameterControlledByCurve", args = 1)]
    pub fn is_parameter_controlled_by_curve(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsParameterControlledByCurve", args = 1)]
    pub fn is_parameter_controlled_by_curve_2(self, id: i32) -> bool;

    #[method(name = "get_deltaPosition", args = 0)]
    pub fn get_delta_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_deltaRotation", args = 0)]
    pub fn get_delta_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "get_velocity", args = 0)]
    pub fn get_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_angularVelocity", args = 0)]
    pub fn get_angular_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_rootPosition", args = 0)]
    pub fn get_root_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_rootPosition", args = 1)]
    pub fn set_root_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rootRotation", args = 0)]
    pub fn get_root_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_rootRotation", args = 1)]
    pub fn set_root_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_applyRootMotion", args = 0)]
    pub fn get_apply_root_motion(self) -> bool;

    #[method(name = "set_applyRootMotion", args = 1)]
    pub fn set_apply_root_motion(self, value: bool) -> ();

    #[method(name = "get_linearVelocityBlending", args = 0)]
    pub fn get_linear_velocity_blending(self) -> bool;

    #[method(name = "set_linearVelocityBlending", args = 1)]
    pub fn set_linear_velocity_blending(self, value: bool) -> ();

    #[method(name = "get_animatePhysics", args = 0)]
    pub fn get_animate_physics(self) -> bool;

    #[method(name = "set_animatePhysics", args = 1)]
    pub fn set_animate_physics(self, value: bool) -> ();

    #[method(name = "get_updateMode", args = 0)]
    pub fn get_update_mode(self) -> crate::unity_engine::animatorupdatemode::AnimatorUpdateMode;

    #[method(name = "set_updateMode", args = 1)]
    pub fn set_update_mode(
        self,
        value: crate::unity_engine::animatorupdatemode::AnimatorUpdateMode,
    ) -> ();

    #[method(name = "get_hasTransformHierarchy", args = 0)]
    pub fn get_has_transform_hierarchy(self) -> bool;

    #[method(name = "get_allowConstantClipSamplingOptimization", args = 0)]
    pub fn get_allow_constant_clip_sampling_optimization(self) -> bool;

    #[method(name = "set_allowConstantClipSamplingOptimization", args = 1)]
    pub fn set_allow_constant_clip_sampling_optimization(self, value: bool) -> ();

    #[method(name = "get_gravityWeight", args = 0)]
    pub fn get_gravity_weight(self) -> f32;

    #[method(name = "get_bodyPosition", args = 0)]
    pub fn get_body_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_bodyPosition", args = 1)]
    pub fn set_body_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_bodyPositionInternal", args = 0)]
    pub fn get_body_position_internal(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_bodyPositionInternal", args = 1)]
    pub fn set_body_position_internal(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_bodyRotation", args = 0)]
    pub fn get_body_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_bodyRotation", args = 1)]
    pub fn set_body_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_bodyRotationInternal", args = 0)]
    pub fn get_body_rotation_internal(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_bodyRotationInternal", args = 1)]
    pub fn set_body_rotation_internal(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetIKPosition", args = 1)]
    pub fn get_ik_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetGoalPosition", args = 1)]
    pub fn get_goal_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetIKPosition", args = 2)]
    pub fn set_ik_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetGoalPosition", args = 2)]
    pub fn set_goal_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetIKRotation", args = 1)]
    pub fn get_ik_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetGoalRotation", args = 1)]
    pub fn get_goal_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "SetIKRotation", args = 2)]
    pub fn set_ik_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SetGoalRotation", args = 2)]
    pub fn set_goal_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetIKPositionWeight", args = 1)]
    pub fn get_ik_position_weight(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> f32;

    #[method(name = "GetGoalWeightPosition", args = 1)]
    pub fn get_goal_weight_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> f32;

    #[method(name = "SetIKPositionWeight", args = 2)]
    pub fn set_ik_position_weight(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        value: f32,
    ) -> ();

    #[method(name = "SetGoalWeightPosition", args = 2)]
    pub fn set_goal_weight_position(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        value: f32,
    ) -> ();

    #[method(name = "GetIKRotationWeight", args = 1)]
    pub fn get_ik_rotation_weight(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> f32;

    #[method(name = "GetGoalWeightRotation", args = 1)]
    pub fn get_goal_weight_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
    ) -> f32;

    #[method(name = "SetIKRotationWeight", args = 2)]
    pub fn set_ik_rotation_weight(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        value: f32,
    ) -> ();

    #[method(name = "SetGoalWeightRotation", args = 2)]
    pub fn set_goal_weight_rotation(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        value: f32,
    ) -> ();

    #[method(name = "GetIKHintPosition", args = 1)]
    pub fn get_ik_hint_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetHintPosition", args = 1)]
    pub fn get_hint_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetIKHintPosition", args = 2)]
    pub fn set_ik_hint_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        hint_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetHintPosition", args = 2)]
    pub fn set_hint_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        hint_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetIKHintPositionWeight", args = 1)]
    pub fn get_ik_hint_position_weight(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
    ) -> f32;

    #[method(name = "GetHintWeightPosition", args = 1)]
    pub fn get_hint_weight_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
    ) -> f32;

    #[method(name = "SetIKHintPositionWeight", args = 2)]
    pub fn set_ik_hint_position_weight(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        value: f32,
    ) -> ();

    #[method(name = "SetHintWeightPosition", args = 2)]
    pub fn set_hint_weight_position(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        value: f32,
    ) -> ();

    #[method(name = "SetLookAtPosition", args = 1)]
    pub fn set_look_at_position(
        self,
        look_at_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetLookAtPositionInternal", args = 1)]
    pub fn set_look_at_position_internal(
        self,
        look_at_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetLookAtWeight", args = 1)]
    pub fn set_look_at_weight(self, weight: f32) -> ();

    #[method(name = "SetLookAtWeight", args = 2)]
    pub fn set_look_at_weight_2(self, weight: f32, body_weight: f32) -> ();

    #[method(name = "SetLookAtWeight", args = 3)]
    pub fn set_look_at_weight_3(self, weight: f32, body_weight: f32, head_weight: f32) -> ();

    #[method(name = "SetLookAtWeight", args = 4)]
    pub fn set_look_at_weight_4(
        self,
        weight: f32,
        body_weight: f32,
        head_weight: f32,
        eyes_weight: f32,
    ) -> ();

    #[method(name = "SetLookAtWeight", args = 5)]
    pub fn set_look_at_weight_5(
        self,
        weight: f32,
        body_weight: f32,
        head_weight: f32,
        eyes_weight: f32,
        clamp_weight: f32,
    ) -> ();

    #[method(name = "SetLookAtWeightInternal", args = 5)]
    pub fn set_look_at_weight_internal(
        self,
        weight: f32,
        body_weight: f32,
        head_weight: f32,
        eyes_weight: f32,
        clamp_weight: f32,
    ) -> ();

    #[method(name = "SetBoneLocalRotation", args = 2)]
    pub fn set_bone_local_rotation(
        self,
        human_bone_id: crate::unity_engine::humanbodybones::HumanBodyBones,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SetBoneLocalRotationInternal", args = 2)]
    pub fn set_bone_local_rotation_internal(
        self,
        human_bone_id: i32,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetBehaviour", args = 1)]
    pub fn get_behaviour(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::scriptableobject::ScriptableObject;

    #[method(name = "InternalGetBehaviours", args = 1)]
    pub fn internal_get_behaviours(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::scriptableobject::ScriptableObject>;

    #[method(name = "GetBehaviours", args = 2)]
    pub fn get_behaviours(
        self,
        full_path_hash: i32,
        layer_index: i32,
    ) -> ::unity2::Array<crate::unity_engine::statemachinebehaviour::StateMachineBehaviour>;

    #[method(name = "InternalGetBehavioursByKey", args = 3)]
    pub fn internal_get_behaviours_by_key(
        self,
        full_path_hash: i32,
        layer_index: i32,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::scriptableobject::ScriptableObject>;

    #[method(name = "get_stabilizeFeet", args = 0)]
    pub fn get_stabilize_feet(self) -> bool;

    #[method(name = "set_stabilizeFeet", args = 1)]
    pub fn set_stabilize_feet(self, value: bool) -> ();

    #[method(name = "get_layerCount", args = 0)]
    pub fn get_layer_count(self) -> i32;

    #[method(name = "GetLayerName", args = 1)]
    pub fn get_layer_name(self, layer_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetLayerIndex", args = 1)]
    pub fn get_layer_index(self, layer_name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetLayerWeight", args = 1)]
    pub fn get_layer_weight(self, layer_index: i32) -> f32;

    #[method(name = "SetLayerWeight", args = 2)]
    pub fn set_layer_weight(self, layer_index: i32, weight: f32) -> ();

    #[method(name = "GetAnimatorStateInfo", args = 3)]
    pub fn get_animator_state_info(
        self,
        layer_index: i32,
        state_info_index: crate::unity_engine::stateinfoindex::StateInfoIndex,
        info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
    ) -> ();

    #[method(name = "GetCurrentAnimatorStateInfo", args = 1)]
    pub fn get_current_animator_state_info(
        self,
        layer_index: i32,
    ) -> crate::unity_engine::animatorstateinfo::AnimatorStateInfo;

    #[method(name = "GetNextAnimatorStateInfo", args = 1)]
    pub fn get_next_animator_state_info(
        self,
        layer_index: i32,
    ) -> crate::unity_engine::animatorstateinfo::AnimatorStateInfo;

    #[method(name = "GetAnimatorTransitionInfo", args = 2)]
    pub fn get_animator_transition_info(
        self,
        layer_index: i32,
        info: crate::unity_engine::animatortransitioninfo::AnimatorTransitionInfo,
    ) -> ();

    #[method(name = "GetAnimatorTransitionInfo", args = 1)]
    pub fn get_animator_transition_info_2(
        self,
        layer_index: i32,
    ) -> crate::unity_engine::animatortransitioninfo::AnimatorTransitionInfo;

    #[method(name = "GetAnimatorClipInfoCount", args = 2)]
    pub fn get_animator_clip_info_count(self, layer_index: i32, current: bool) -> i32;

    #[method(name = "GetCurrentAnimatorClipInfoCount", args = 1)]
    pub fn get_current_animator_clip_info_count(self, layer_index: i32) -> i32;

    #[method(name = "GetNextAnimatorClipInfoCount", args = 1)]
    pub fn get_next_animator_clip_info_count(self, layer_index: i32) -> i32;

    #[method(name = "GetCurrentAnimatorClipInfo", args = 1)]
    pub fn get_current_animator_clip_info(
        self,
        layer_index: i32,
    ) -> ::unity2::Array<crate::unity_engine::animatorclipinfo::AnimatorClipInfo>;

    #[method(name = "GetNextAnimatorClipInfo", args = 1)]
    pub fn get_next_animator_clip_info(
        self,
        layer_index: i32,
    ) -> ::unity2::Array<crate::unity_engine::animatorclipinfo::AnimatorClipInfo>;

    #[method(name = "GetCurrentAnimatorClipInfo", args = 2)]
    pub fn get_current_animator_clip_info_2(
        self,
        layer_index: i32,
        clips: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animatorclipinfo::AnimatorClipInfo,
        >,
    ) -> ();

    #[method(name = "GetAnimatorClipInfoInternal", args = 3)]
    pub fn get_animator_clip_info_internal(
        self,
        layer_index: i32,
        is_current: bool,
        clips: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetNextAnimatorClipInfo", args = 2)]
    pub fn get_next_animator_clip_info_2(
        self,
        layer_index: i32,
        clips: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animatorclipinfo::AnimatorClipInfo,
        >,
    ) -> ();

    #[method(name = "IsInTransition", args = 1)]
    pub fn is_in_transition(self, layer_index: i32) -> bool;

    #[method(name = "get_parameters", args = 0)]
    pub fn get_parameters(
        self,
    ) -> ::unity2::Array<
        crate::unity_engine::animatorcontrollerparameter::AnimatorControllerParameter,
    >;

    #[method(name = "get_parameterCount", args = 0)]
    pub fn get_parameter_count(self) -> i32;

    #[method(name = "GetParameter", args = 1)]
    pub fn get_parameter(
        self,
        index: i32,
    ) -> crate::unity_engine::animatorcontrollerparameter::AnimatorControllerParameter;

    #[method(name = "get_feetPivotActive", args = 0)]
    pub fn get_feet_pivot_active(self) -> f32;

    #[method(name = "set_feetPivotActive", args = 1)]
    pub fn set_feet_pivot_active(self, value: f32) -> ();

    #[method(name = "get_pivotWeight", args = 0)]
    pub fn get_pivot_weight(self) -> f32;

    #[method(name = "get_pivotPosition", args = 0)]
    pub fn get_pivot_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "MatchTarget", args = 7)]
    pub fn match_target(
        self,
        match_position: crate::unity_engine::vector3::Vector3,
        match_rotation: crate::unity_engine::quaternion::Quaternion,
        target_body_part: i32,
        weight_mask: crate::unity_engine::matchtargetweightmask::MatchTargetWeightMask,
        start_normalized_time: f32,
        target_normalized_time: f32,
        complete_match: bool,
    ) -> ();

    #[method(name = "MatchTarget", args = 5)]
    pub fn match_target_2(
        self,
        match_position: crate::unity_engine::vector3::Vector3,
        match_rotation: crate::unity_engine::quaternion::Quaternion,
        target_body_part: crate::unity_engine::avatartarget::AvatarTarget,
        weight_mask: crate::unity_engine::matchtargetweightmask::MatchTargetWeightMask,
        start_normalized_time: f32,
    ) -> ();

    #[method(name = "MatchTarget", args = 6)]
    pub fn match_target_3(
        self,
        match_position: crate::unity_engine::vector3::Vector3,
        match_rotation: crate::unity_engine::quaternion::Quaternion,
        target_body_part: crate::unity_engine::avatartarget::AvatarTarget,
        weight_mask: crate::unity_engine::matchtargetweightmask::MatchTargetWeightMask,
        start_normalized_time: f32,
        target_normalized_time: f32,
    ) -> ();

    #[method(name = "MatchTarget", args = 7)]
    pub fn match_target_4(
        self,
        match_position: crate::unity_engine::vector3::Vector3,
        match_rotation: crate::unity_engine::quaternion::Quaternion,
        target_body_part: crate::unity_engine::avatartarget::AvatarTarget,
        weight_mask: crate::unity_engine::matchtargetweightmask::MatchTargetWeightMask,
        start_normalized_time: f32,
        target_normalized_time: f32,
        complete_match: bool,
    ) -> ();

    #[method(name = "InterruptMatchTarget", args = 0)]
    pub fn interrupt_match_target(self) -> ();

    #[method(name = "InterruptMatchTarget", args = 1)]
    pub fn interrupt_match_target_2(self, complete_match: bool) -> ();

    #[method(name = "get_isMatchingTarget", args = 0)]
    pub fn get_is_matching_target(self) -> bool;

    #[method(name = "get_speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "set_speed", args = 1)]
    pub fn set_speed(self, value: f32) -> ();

    #[method(name = "ForceStateNormalizedTime", args = 1)]
    pub fn force_state_normalized_time(self, normalized_time: f32) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 2)]
    pub fn cross_fade_in_fixed_time(
        self,
        state_name: ::unity2::Il2CppString,
        fixed_transition_duration: f32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 3)]
    pub fn cross_fade_in_fixed_time_2(
        self,
        state_name: ::unity2::Il2CppString,
        fixed_transition_duration: f32,
        layer: i32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 4)]
    pub fn cross_fade_in_fixed_time_3(
        self,
        state_name: ::unity2::Il2CppString,
        fixed_transition_duration: f32,
        layer: i32,
        fixed_time_offset: f32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 5)]
    pub fn cross_fade_in_fixed_time_4(
        self,
        state_name: ::unity2::Il2CppString,
        fixed_transition_duration: f32,
        layer: i32,
        fixed_time_offset: f32,
        normalized_transition_time: f32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 4)]
    pub fn cross_fade_in_fixed_time_5(
        self,
        state_hash_name: i32,
        fixed_transition_duration: f32,
        layer: i32,
        fixed_time_offset: f32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 3)]
    pub fn cross_fade_in_fixed_time_6(
        self,
        state_hash_name: i32,
        fixed_transition_duration: f32,
        layer: i32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 2)]
    pub fn cross_fade_in_fixed_time_7(
        self,
        state_hash_name: i32,
        fixed_transition_duration: f32,
    ) -> ();

    #[method(name = "CrossFadeInFixedTime", args = 5)]
    pub fn cross_fade_in_fixed_time_8(
        self,
        state_hash_name: i32,
        fixed_transition_duration: f32,
        layer: i32,
        fixed_time_offset: f32,
        normalized_transition_time: f32,
    ) -> ();

    #[method(name = "WriteDefaultValues", args = 0)]
    pub fn write_default_values(self) -> ();

    #[method(name = "CrossFade", args = 4)]
    pub fn cross_fade(
        self,
        state_name: ::unity2::Il2CppString,
        normalized_transition_duration: f32,
        layer: i32,
        normalized_time_offset: f32,
    ) -> ();

    #[method(name = "CrossFade", args = 3)]
    pub fn cross_fade_2(
        self,
        state_name: ::unity2::Il2CppString,
        normalized_transition_duration: f32,
        layer: i32,
    ) -> ();

    #[method(name = "CrossFade", args = 2)]
    pub fn cross_fade_3(
        self,
        state_name: ::unity2::Il2CppString,
        normalized_transition_duration: f32,
    ) -> ();

    #[method(name = "CrossFade", args = 5)]
    pub fn cross_fade_4(
        self,
        state_name: ::unity2::Il2CppString,
        normalized_transition_duration: f32,
        layer: i32,
        normalized_time_offset: f32,
        normalized_transition_time: f32,
    ) -> ();

    #[method(name = "CrossFade", args = 5)]
    pub fn cross_fade_5(
        self,
        state_hash_name: i32,
        normalized_transition_duration: f32,
        layer: i32,
        normalized_time_offset: f32,
        normalized_transition_time: f32,
    ) -> ();

    #[method(name = "CrossFade", args = 4)]
    pub fn cross_fade_6(
        self,
        state_hash_name: i32,
        normalized_transition_duration: f32,
        layer: i32,
        normalized_time_offset: f32,
    ) -> ();

    #[method(name = "CrossFade", args = 3)]
    pub fn cross_fade_7(
        self,
        state_hash_name: i32,
        normalized_transition_duration: f32,
        layer: i32,
    ) -> ();

    #[method(name = "CrossFade", args = 2)]
    pub fn cross_fade_8(self, state_hash_name: i32, normalized_transition_duration: f32) -> ();

    #[method(name = "PlayInFixedTime", args = 2)]
    pub fn play_in_fixed_time(self, state_name: ::unity2::Il2CppString, layer: i32) -> ();

    #[method(name = "PlayInFixedTime", args = 1)]
    pub fn play_in_fixed_time_2(self, state_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayInFixedTime", args = 3)]
    pub fn play_in_fixed_time_3(
        self,
        state_name: ::unity2::Il2CppString,
        layer: i32,
        fixed_time: f32,
    ) -> ();

    #[method(name = "PlayInFixedTime", args = 3)]
    pub fn play_in_fixed_time_4(self, state_name_hash: i32, layer: i32, fixed_time: f32) -> ();

    #[method(name = "PlayInFixedTime", args = 2)]
    pub fn play_in_fixed_time_5(self, state_name_hash: i32, layer: i32) -> ();

    #[method(name = "PlayInFixedTime", args = 1)]
    pub fn play_in_fixed_time_6(self, state_name_hash: i32) -> ();

    #[method(name = "Play", args = 2)]
    pub fn play(self, state_name: ::unity2::Il2CppString, layer: i32) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play_2(self, state_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Play", args = 3)]
    pub fn play_3(self, state_name: ::unity2::Il2CppString, layer: i32, normalized_time: f32)
        -> ();

    #[method(name = "Play", args = 3)]
    pub fn play_4(self, state_name_hash: i32, layer: i32, normalized_time: f32) -> ();

    #[method(name = "Play", args = 2)]
    pub fn play_5(self, state_name_hash: i32, layer: i32) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play_6(self, state_name_hash: i32) -> ();

    #[method(name = "SetTarget", args = 2)]
    pub fn set_target(
        self,
        target_index: crate::unity_engine::avatartarget::AvatarTarget,
        target_normalized_time: f32,
    ) -> ();

    #[method(name = "get_targetPosition", args = 0)]
    pub fn get_target_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_targetRotation", args = 0)]
    pub fn get_target_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "IsControlled", args = 1)]
    pub fn is_controlled(self, transform: crate::unity_engine::transform::Transform) -> bool;

    #[method(name = "IsBoneTransform", args = 1)]
    pub fn is_bone_transform(self, transform: crate::unity_engine::transform::Transform) -> bool;

    #[method(name = "get_avatarRoot", args = 0)]
    pub fn get_avatar_root(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetBoneTransform", args = 1)]
    pub fn get_bone_transform(
        self,
        human_bone_id: crate::unity_engine::humanbodybones::HumanBodyBones,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetBoneTransformInternal", args = 1)]
    pub fn get_bone_transform_internal(
        self,
        human_bone_id: i32,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_cullingMode", args = 0)]
    pub fn get_culling_mode(self) -> crate::unity_engine::animatorcullingmode::AnimatorCullingMode;

    #[method(name = "set_cullingMode", args = 1)]
    pub fn set_culling_mode(
        self,
        value: crate::unity_engine::animatorcullingmode::AnimatorCullingMode,
    ) -> ();

    #[method(name = "StartPlayback", args = 0)]
    pub fn start_playback(self) -> ();

    #[method(name = "StopPlayback", args = 0)]
    pub fn stop_playback(self) -> ();

    #[method(name = "get_playbackTime", args = 0)]
    pub fn get_playback_time(self) -> f32;

    #[method(name = "set_playbackTime", args = 1)]
    pub fn set_playback_time(self, value: f32) -> ();

    #[method(name = "StartRecording", args = 1)]
    pub fn start_recording(self, frame_count: i32) -> ();

    #[method(name = "StopRecording", args = 0)]
    pub fn stop_recording(self) -> ();

    #[method(name = "get_recorderStartTime", args = 0)]
    pub fn get_recorder_start_time(self) -> f32;

    #[method(name = "set_recorderStartTime", args = 1)]
    pub fn set_recorder_start_time(self, value: f32) -> ();

    #[method(name = "get_recorderStopTime", args = 0)]
    pub fn get_recorder_stop_time(self) -> f32;

    #[method(name = "set_recorderStopTime", args = 1)]
    pub fn set_recorder_stop_time(self, value: f32) -> ();

    #[method(name = "get_recorderMode", args = 0)]
    pub fn get_recorder_mode(
        self,
    ) -> crate::unity_engine::animatorrecordermode::AnimatorRecorderMode;

    #[method(name = "get_runtimeAnimatorController", args = 0)]
    pub fn get_runtime_animator_controller(
        self,
    ) -> crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController;

    #[method(name = "set_runtimeAnimatorController", args = 1)]
    pub fn set_runtime_animator_controller(
        self,
        value: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> ();

    #[method(name = "get_hasBoundPlayables", args = 0)]
    pub fn get_has_bound_playables(self) -> bool;

    #[method(name = "ClearInternalControllerPlayable", args = 0)]
    pub fn clear_internal_controller_playable(self) -> ();

    #[method(name = "HasState", args = 2)]
    pub fn has_state(self, layer_index: i32, state_id: i32) -> bool;

    #[method(name = "StringToHash", args = 1)]
    pub fn string_to_hash(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "get_avatar", args = 0)]
    pub fn get_avatar(self) -> crate::unity_engine::avatar::Avatar;

    #[method(name = "set_avatar", args = 1)]
    pub fn set_avatar(self, value: crate::unity_engine::avatar::Avatar) -> ();

    #[method(name = "GetStats", args = 0)]
    pub fn get_stats(self) -> ::unity2::Il2CppString;

    #[method(name = "get_playableGraph", args = 0)]
    pub fn get_playable_graph(self)
        -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "GetCurrentGraph", args = 1)]
    pub fn get_current_graph(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "CheckIfInIKPass", args = 0)]
    pub fn check_if_in_ik_pass(self) -> ();

    #[method(name = "IsInIKPass", args = 0)]
    pub fn is_in_ik_pass(self) -> bool;

    #[method(name = "SetFloatString", args = 2)]
    pub fn set_float_string(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetFloatID", args = 2)]
    pub fn set_float_id(self, id: i32, value: f32) -> ();

    #[method(name = "GetFloatString", args = 1)]
    pub fn get_float_string(self, name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetFloatID", args = 1)]
    pub fn get_float_id(self, id: i32) -> f32;

    #[method(name = "SetBoolString", args = 2)]
    pub fn set_bool_string(self, name: ::unity2::Il2CppString, value: bool) -> ();

    #[method(name = "SetBoolID", args = 2)]
    pub fn set_bool_id(self, id: i32, value: bool) -> ();

    #[method(name = "GetBoolString", args = 1)]
    pub fn get_bool_string(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetBoolID", args = 1)]
    pub fn get_bool_id(self, id: i32) -> bool;

    #[method(name = "SetIntegerString", args = 2)]
    pub fn set_integer_string(self, name: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "SetIntegerID", args = 2)]
    pub fn set_integer_id(self, id: i32, value: i32) -> ();

    #[method(name = "GetIntegerString", args = 1)]
    pub fn get_integer_string(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetIntegerID", args = 1)]
    pub fn get_integer_id(self, id: i32) -> i32;

    #[method(name = "SetTriggerString", args = 1)]
    pub fn set_trigger_string(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetTriggerID", args = 1)]
    pub fn set_trigger_id(self, id: i32) -> ();

    #[method(name = "ResetTriggerString", args = 1)]
    pub fn reset_trigger_string(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "ResetTriggerID", args = 1)]
    pub fn reset_trigger_id(self, id: i32) -> ();

    #[method(name = "IsParameterControlledByCurveString", args = 1)]
    pub fn is_parameter_controlled_by_curve_string(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsParameterControlledByCurveID", args = 1)]
    pub fn is_parameter_controlled_by_curve_id(self, id: i32) -> bool;

    #[method(name = "SetFloatStringDamp", args = 4)]
    pub fn set_float_string_damp(
        self,
        name: ::unity2::Il2CppString,
        value: f32,
        damp_time: f32,
        delta_time: f32,
    ) -> ();

    #[method(name = "SetFloatIDDamp", args = 4)]
    pub fn set_float_id_damp(self, id: i32, value: f32, damp_time: f32, delta_time: f32) -> ();

    #[method(name = "get_layersAffectMassCenter", args = 0)]
    pub fn get_layers_affect_mass_center(self) -> bool;

    #[method(name = "set_layersAffectMassCenter", args = 1)]
    pub fn set_layers_affect_mass_center(self, value: bool) -> ();

    #[method(name = "get_leftFeetBottomHeight", args = 0)]
    pub fn get_left_feet_bottom_height(self) -> f32;

    #[method(name = "get_rightFeetBottomHeight", args = 0)]
    pub fn get_right_feet_bottom_height(self) -> f32;

    #[method(name = "get_supportsOnAnimatorMove", args = 0)]
    pub fn get_supports_on_animator_move(self) -> bool;

    #[method(name = "OnUpdateModeChanged", args = 0)]
    pub fn on_update_mode_changed(self) -> ();

    #[method(name = "OnCullingModeChanged", args = 0)]
    pub fn on_culling_mode_changed(self) -> ();

    #[method(name = "WriteDefaultPose", args = 0)]
    pub fn write_default_pose(self) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, delta_time: f32) -> ();

    #[method(name = "Rebind", args = 0)]
    pub fn rebind(self) -> ();

    #[method(name = "Rebind", args = 1)]
    pub fn rebind_2(self, write_default_values: bool) -> ();

    #[method(name = "ApplyBuiltinRootMotion", args = 0)]
    pub fn apply_builtin_root_motion(self) -> ();

    #[method(name = "EvaluateController", args = 0)]
    pub fn evaluate_controller(self) -> ();

    #[method(name = "EvaluateController", args = 1)]
    pub fn evaluate_controller_2(self, delta_time: f32) -> ();

    #[method(name = "GetCurrentStateName", args = 1)]
    pub fn get_current_state_name(self, layer_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetNextStateName", args = 1)]
    pub fn get_next_state_name(self, layer_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetAnimatorStateName", args = 2)]
    pub fn get_animator_state_name(self, layer_index: i32, current: bool)
        -> ::unity2::Il2CppString;

    #[method(name = "ResolveHash", args = 1)]
    pub fn resolve_hash(self, hash: i32) -> ::unity2::Il2CppString;

    #[method(name = "get_logWarnings", args = 0)]
    pub fn get_log_warnings(self) -> bool;

    #[method(name = "set_logWarnings", args = 1)]
    pub fn set_log_warnings(self, value: bool) -> ();

    #[method(name = "get_fireEvents", args = 0)]
    pub fn get_fire_events(self) -> bool;

    #[method(name = "set_fireEvents", args = 1)]
    pub fn set_fire_events(self, value: bool) -> ();

    #[method(name = "get_keepAnimatorControllerStateOnDisable", args = 0)]
    pub fn get_keep_animator_controller_state_on_disable(self) -> bool;

    #[method(name = "set_keepAnimatorControllerStateOnDisable", args = 1)]
    pub fn set_keep_animator_controller_state_on_disable(self, value: bool) -> ();

    #[method(name = "GetVector", args = 1)]
    pub fn get_vector(self, name: ::unity2::Il2CppString) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetVector", args = 1)]
    pub fn get_vector_2(self, id: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector_2(self, id: i32, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetQuaternion", args = 1)]
    pub fn get_quaternion(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetQuaternion", args = 1)]
    pub fn get_quaternion_2(self, id: i32) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "SetQuaternion", args = 2)]
    pub fn set_quaternion(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SetQuaternion", args = 2)]
    pub fn set_quaternion_2(
        self,
        id: i32,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_deltaPosition_Injected", args = 1)]
    pub fn get_delta_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_deltaRotation_Injected", args = 1)]
    pub fn get_delta_rotation_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_velocity_Injected", args = 1)]
    pub fn get_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_angularVelocity_Injected", args = 1)]
    pub fn get_angular_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rootPosition_Injected", args = 1)]
    pub fn get_root_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_rootPosition_Injected", args = 1)]
    pub fn set_root_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rootRotation_Injected", args = 1)]
    pub fn get_root_rotation_injected(self, ret: crate::unity_engine::quaternion::Quaternion)
        -> ();

    #[method(name = "set_rootRotation_Injected", args = 1)]
    pub fn set_root_rotation_injected(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_bodyPositionInternal_Injected", args = 1)]
    pub fn get_body_position_internal_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "set_bodyPositionInternal_Injected", args = 1)]
    pub fn set_body_position_internal_injected(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_bodyRotationInternal_Injected", args = 1)]
    pub fn get_body_rotation_internal_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "set_bodyRotationInternal_Injected", args = 1)]
    pub fn set_body_rotation_internal_injected(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetGoalPosition_Injected", args = 2)]
    pub fn get_goal_position_injected(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetGoalPosition_Injected", args = 2)]
    pub fn set_goal_position_injected(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetGoalRotation_Injected", args = 2)]
    pub fn get_goal_rotation_injected(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SetGoalRotation_Injected", args = 2)]
    pub fn set_goal_rotation_injected(
        self,
        goal: crate::unity_engine::avatarikgoal::AvatarIKGoal,
        goal_rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "GetHintPosition_Injected", args = 2)]
    pub fn get_hint_position_injected(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetHintPosition_Injected", args = 2)]
    pub fn set_hint_position_injected(
        self,
        hint: crate::unity_engine::avatarikhint::AvatarIKHint,
        hint_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetLookAtPositionInternal_Injected", args = 1)]
    pub fn set_look_at_position_internal_injected(
        self,
        look_at_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetBoneLocalRotationInternal_Injected", args = 2)]
    pub fn set_bone_local_rotation_internal_injected(
        self,
        human_bone_id: i32,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_pivotPosition_Injected", args = 1)]
    pub fn get_pivot_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "MatchTarget_Injected", args = 7)]
    pub fn match_target_injected(
        self,
        match_position: crate::unity_engine::vector3::Vector3,
        match_rotation: crate::unity_engine::quaternion::Quaternion,
        target_body_part: i32,
        weight_mask: crate::unity_engine::matchtargetweightmask::MatchTargetWeightMask,
        start_normalized_time: f32,
        target_normalized_time: f32,
        complete_match: bool,
    ) -> ();

    #[method(name = "get_targetPosition_Injected", args = 1)]
    pub fn get_target_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_targetRotation_Injected", args = 1)]
    pub fn get_target_rotation_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();
}

#[cfg(feature = "unity_engine-animator")]
impl Animator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Animator),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimatorMethods>::ctor(this);
        this
    }
}
