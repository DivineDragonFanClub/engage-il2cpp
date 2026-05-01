
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rigidbody/Rigidbody.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Rigidbody")]
#[parent(crate::unity_engine::component::Component)]
pub struct Rigidbody {}

#[cfg(feature = "unity_engine-rigidbody")]
#[::unity2::methods]
impl Rigidbody {
    #[method(name = "get_velocity", args = 0)]
    pub fn get_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_velocity", args = 1)]
    pub fn set_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_angularVelocity", args = 0)]
    pub fn get_angular_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_angularVelocity", args = 1)]
    pub fn set_angular_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_drag", args = 0)]
    pub fn get_drag(self) -> f32;

    #[method(name = "set_drag", args = 1)]
    pub fn set_drag(self, value: f32) -> ();

    #[method(name = "get_angularDrag", args = 0)]
    pub fn get_angular_drag(self) -> f32;

    #[method(name = "set_angularDrag", args = 1)]
    pub fn set_angular_drag(self, value: f32) -> ();

    #[method(name = "get_mass", args = 0)]
    pub fn get_mass(self) -> f32;

    #[method(name = "set_mass", args = 1)]
    pub fn set_mass(self, value: f32) -> ();

    #[method(name = "SetDensity", args = 1)]
    pub fn set_density(self, density: f32) -> ();

    #[method(name = "get_useGravity", args = 0)]
    pub fn get_use_gravity(self) -> bool;

    #[method(name = "set_useGravity", args = 1)]
    pub fn set_use_gravity(self, value: bool) -> ();

    #[method(name = "get_maxDepenetrationVelocity", args = 0)]
    pub fn get_max_depenetration_velocity(self) -> f32;

    #[method(name = "set_maxDepenetrationVelocity", args = 1)]
    pub fn set_max_depenetration_velocity(self, value: f32) -> ();

    #[method(name = "get_isKinematic", args = 0)]
    pub fn get_is_kinematic(self) -> bool;

    #[method(name = "set_isKinematic", args = 1)]
    pub fn set_is_kinematic(self, value: bool) -> ();

    #[method(name = "get_freezeRotation", args = 0)]
    pub fn get_freeze_rotation(self) -> bool;

    #[method(name = "set_freezeRotation", args = 1)]
    pub fn set_freeze_rotation(self, value: bool) -> ();

    #[method(name = "get_constraints", args = 0)]
    pub fn get_constraints(self)
        -> crate::unity_engine::rigidbodyconstraints::RigidbodyConstraints;

    #[method(name = "set_constraints", args = 1)]
    pub fn set_constraints(
        self,
        value: crate::unity_engine::rigidbodyconstraints::RigidbodyConstraints,
    ) -> ();

    #[method(name = "get_collisionDetectionMode", args = 0)]
    pub fn get_collision_detection_mode(
        self,
    ) -> crate::unity_engine::collisiondetectionmode::CollisionDetectionMode;

    #[method(name = "set_collisionDetectionMode", args = 1)]
    pub fn set_collision_detection_mode(
        self,
        value: crate::unity_engine::collisiondetectionmode::CollisionDetectionMode,
    ) -> ();

    #[method(name = "get_centerOfMass", args = 0)]
    pub fn get_center_of_mass(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_centerOfMass", args = 1)]
    pub fn set_center_of_mass(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_worldCenterOfMass", args = 0)]
    pub fn get_world_center_of_mass(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_inertiaTensorRotation", args = 0)]
    pub fn get_inertia_tensor_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_inertiaTensorRotation", args = 1)]
    pub fn set_inertia_tensor_rotation(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_inertiaTensor", args = 0)]
    pub fn get_inertia_tensor(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_inertiaTensor", args = 1)]
    pub fn set_inertia_tensor(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_detectCollisions", args = 0)]
    pub fn get_detect_collisions(self) -> bool;

    #[method(name = "set_detectCollisions", args = 1)]
    pub fn set_detect_collisions(self, value: bool) -> ();

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_rotation", args = 1)]
    pub fn set_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_interpolation", args = 0)]
    pub fn get_interpolation(
        self,
    ) -> crate::unity_engine::rigidbodyinterpolation::RigidbodyInterpolation;

    #[method(name = "set_interpolation", args = 1)]
    pub fn set_interpolation(
        self,
        value: crate::unity_engine::rigidbodyinterpolation::RigidbodyInterpolation,
    ) -> ();

    #[method(name = "get_solverIterations", args = 0)]
    pub fn get_solver_iterations(self) -> i32;

    #[method(name = "set_solverIterations", args = 1)]
    pub fn set_solver_iterations(self, value: i32) -> ();

    #[method(name = "get_sleepThreshold", args = 0)]
    pub fn get_sleep_threshold(self) -> f32;

    #[method(name = "set_sleepThreshold", args = 1)]
    pub fn set_sleep_threshold(self, value: f32) -> ();

    #[method(name = "get_maxAngularVelocity", args = 0)]
    pub fn get_max_angular_velocity(self) -> f32;

    #[method(name = "set_maxAngularVelocity", args = 1)]
    pub fn set_max_angular_velocity(self, value: f32) -> ();

    #[method(name = "MovePosition", args = 1)]
    pub fn move_position(self, position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "MoveRotation", args = 1)]
    pub fn move_rotation(self, rot: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "Sleep", args = 0)]
    pub fn sleep(self) -> ();

    #[method(name = "IsSleeping", args = 0)]
    pub fn is_sleeping(self) -> bool;

    #[method(name = "WakeUp", args = 0)]
    pub fn wake_up(self) -> ();

    #[method(name = "ResetCenterOfMass", args = 0)]
    pub fn reset_center_of_mass(self) -> ();

    #[method(name = "ResetInertiaTensor", args = 0)]
    pub fn reset_inertia_tensor(self) -> ();

    #[method(name = "GetRelativePointVelocity", args = 1)]
    pub fn get_relative_point_velocity(
        self,
        relative_point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetPointVelocity", args = 1)]
    pub fn get_point_velocity(
        self,
        world_point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_solverVelocityIterations", args = 0)]
    pub fn get_solver_velocity_iterations(self) -> i32;

    #[method(name = "set_solverVelocityIterations", args = 1)]
    pub fn set_solver_velocity_iterations(self, value: i32) -> ();

    #[method(name = "get_sleepVelocity", args = 0)]
    pub fn get_sleep_velocity(self) -> f32;

    #[method(name = "set_sleepVelocity", args = 1)]
    pub fn set_sleep_velocity(self, value: f32) -> ();

    #[method(name = "get_sleepAngularVelocity", args = 0)]
    pub fn get_sleep_angular_velocity(self) -> f32;

    #[method(name = "set_sleepAngularVelocity", args = 1)]
    pub fn set_sleep_angular_velocity(self, value: f32) -> ();

    #[method(name = "get_useConeFriction", args = 0)]
    pub fn get_use_cone_friction(self) -> bool;

    #[method(name = "set_useConeFriction", args = 1)]
    pub fn set_use_cone_friction(self, value: bool) -> ();

    #[method(name = "get_solverIterationCount", args = 0)]
    pub fn get_solver_iteration_count(self) -> i32;

    #[method(name = "set_solverIterationCount", args = 1)]
    pub fn set_solver_iteration_count(self, value: i32) -> ();

    #[method(name = "get_solverVelocityIterationCount", args = 0)]
    pub fn get_solver_velocity_iteration_count(self) -> i32;

    #[method(name = "set_solverVelocityIterationCount", args = 1)]
    pub fn set_solver_velocity_iteration_count(self, value: i32) -> ();

    #[method(name = "AddForce", args = 2)]
    pub fn add_force(
        self,
        force: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddForce", args = 1)]
    pub fn add_force_2(self, force: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "AddForce", args = 4)]
    pub fn add_force_3(
        self,
        x: f32,
        y: f32,
        z: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddForce", args = 3)]
    pub fn add_force_4(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "AddRelativeForce", args = 2)]
    pub fn add_relative_force(
        self,
        force: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeForce", args = 1)]
    pub fn add_relative_force_2(self, force: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "AddRelativeForce", args = 4)]
    pub fn add_relative_force_3(
        self,
        x: f32,
        y: f32,
        z: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeForce", args = 3)]
    pub fn add_relative_force_4(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "AddTorque", args = 2)]
    pub fn add_torque(
        self,
        torque: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddTorque", args = 1)]
    pub fn add_torque_2(self, torque: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "AddTorque", args = 4)]
    pub fn add_torque_3(
        self,
        x: f32,
        y: f32,
        z: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddTorque", args = 3)]
    pub fn add_torque_4(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "AddRelativeTorque", args = 2)]
    pub fn add_relative_torque(
        self,
        torque: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeTorque", args = 1)]
    pub fn add_relative_torque_2(self, torque: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "AddRelativeTorque", args = 4)]
    pub fn add_relative_torque_3(
        self,
        x: f32,
        y: f32,
        z: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeTorque", args = 3)]
    pub fn add_relative_torque_4(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "AddForceAtPosition", args = 3)]
    pub fn add_force_at_position(
        self,
        force: crate::unity_engine::vector3::Vector3,
        position: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddForceAtPosition", args = 2)]
    pub fn add_force_at_position_2(
        self,
        force: crate::unity_engine::vector3::Vector3,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "AddExplosionForce", args = 5)]
    pub fn add_explosion_force(
        self,
        explosion_force: f32,
        explosion_position: crate::unity_engine::vector3::Vector3,
        explosion_radius: f32,
        upwards_modifier: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddExplosionForce", args = 4)]
    pub fn add_explosion_force_2(
        self,
        explosion_force: f32,
        explosion_position: crate::unity_engine::vector3::Vector3,
        explosion_radius: f32,
        upwards_modifier: f32,
    ) -> ();

    #[method(name = "AddExplosionForce", args = 3)]
    pub fn add_explosion_force_3(
        self,
        explosion_force: f32,
        explosion_position: crate::unity_engine::vector3::Vector3,
        explosion_radius: f32,
    ) -> ();

    #[method(name = "Internal_ClosestPointOnBounds", args = 3)]
    pub fn internal_closest_point_on_bounds(
        self,
        point: crate::unity_engine::vector3::Vector3,
        out_pos: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> ();

    #[method(name = "ClosestPointOnBounds", args = 1)]
    pub fn closest_point_on_bounds(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SweepTest", args = 4)]
    pub fn sweep_test(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
        has_hit: bool,
    ) -> crate::unity_engine::raycasthit::RaycastHit;

    #[method(name = "SweepTest", args = 4)]
    pub fn sweep_test_2(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> bool;

    #[method(name = "SweepTest", args = 3)]
    pub fn sweep_test_3(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        max_distance: f32,
    ) -> bool;

    #[method(name = "SweepTest", args = 2)]
    pub fn sweep_test_4(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
    ) -> bool;

    #[method(name = "Internal_SweepTestAll", args = 3)]
    pub fn internal_sweep_test_all(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "SweepTestAll", args = 3)]
    pub fn sweep_test_all(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "SweepTestAll", args = 2)]
    pub fn sweep_test_all_2(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = "SweepTestAll", args = 1)]
    pub fn sweep_test_all_3(
        self,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_velocity_Injected", args = 1)]
    pub fn get_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_velocity_Injected", args = 1)]
    pub fn set_velocity_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_angularVelocity_Injected", args = 1)]
    pub fn get_angular_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_angularVelocity_Injected", args = 1)]
    pub fn set_angular_velocity_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_centerOfMass_Injected", args = 1)]
    pub fn get_center_of_mass_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_centerOfMass_Injected", args = 1)]
    pub fn set_center_of_mass_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_worldCenterOfMass_Injected", args = 1)]
    pub fn get_world_center_of_mass_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_inertiaTensorRotation_Injected", args = 1)]
    pub fn get_inertia_tensor_rotation_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "set_inertiaTensorRotation_Injected", args = 1)]
    pub fn set_inertia_tensor_rotation_injected(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_inertiaTensor_Injected", args = 1)]
    pub fn get_inertia_tensor_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_inertiaTensor_Injected", args = 1)]
    pub fn set_inertia_tensor_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_position_Injected", args = 1)]
    pub fn get_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_position_Injected", args = 1)]
    pub fn set_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation_Injected", args = 1)]
    pub fn get_rotation_injected(self, ret: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "set_rotation_Injected", args = 1)]
    pub fn set_rotation_injected(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "MovePosition_Injected", args = 1)]
    pub fn move_position_injected(self, position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "MoveRotation_Injected", args = 1)]
    pub fn move_rotation_injected(self, rot: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "GetRelativePointVelocity_Injected", args = 2)]
    pub fn get_relative_point_velocity_injected(
        self,
        relative_point: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetPointVelocity_Injected", args = 2)]
    pub fn get_point_velocity_injected(
        self,
        world_point: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "AddForce_Injected", args = 2)]
    pub fn add_force_injected(
        self,
        force: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeForce_Injected", args = 2)]
    pub fn add_relative_force_injected(
        self,
        force: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddTorque_Injected", args = 2)]
    pub fn add_torque_injected(
        self,
        torque: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddRelativeTorque_Injected", args = 2)]
    pub fn add_relative_torque_injected(
        self,
        torque: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddForceAtPosition_Injected", args = 3)]
    pub fn add_force_at_position_injected(
        self,
        force: crate::unity_engine::vector3::Vector3,
        position: crate::unity_engine::vector3::Vector3,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "AddExplosionForce_Injected", args = 5)]
    pub fn add_explosion_force_injected(
        self,
        explosion_force: f32,
        explosion_position: crate::unity_engine::vector3::Vector3,
        explosion_radius: f32,
        upwards_modifier: f32,
        mode: crate::unity_engine::forcemode::ForceMode,
    ) -> ();

    #[method(name = "Internal_ClosestPointOnBounds_Injected", args = 3)]
    pub fn internal_closest_point_on_bounds_injected(
        self,
        point: crate::unity_engine::vector3::Vector3,
        out_pos: crate::unity_engine::vector3::Vector3,
        distance: f32,
    ) -> ();

    #[method(name = "SweepTest_Injected", args = 5)]
    pub fn sweep_test_injected(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
        has_hit: bool,
        ret: crate::unity_engine::raycasthit::RaycastHit,
    ) -> ();

    #[method(name = "Internal_SweepTestAll_Injected", args = 3)]
    pub fn internal_sweep_test_all_injected(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        query_trigger_interaction : crate :: unity_engine :: querytriggerinteraction :: QueryTriggerInteraction,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;
}

#[cfg(feature = "unity_engine-rigidbody")]
impl Rigidbody {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Rigidbody),
                ::core::stringify!(new),
            )
        });
        <Self as IRigidbodyMethods>::ctor(this);
        this
    }
}
