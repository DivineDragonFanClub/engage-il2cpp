
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::joint::IJoint;
use crate::unity_engine::joint::Joint;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/configurablejoint/ConfigurableJoint.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ConfigurableJoint")]
#[parent(crate::unity_engine::joint::Joint)]
pub struct ConfigurableJoint {}

#[cfg(feature = "unity_engine-configurablejoint")]
#[::unity2::methods]
impl ConfigurableJoint {
    #[method(name = "get_secondaryAxis", args = 0)]
    pub fn get_secondary_axis(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_secondaryAxis", args = 1)]
    pub fn set_secondary_axis(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_xMotion", args = 0)]
    pub fn get_x_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_xMotion", args = 1)]
    pub fn set_x_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_yMotion", args = 0)]
    pub fn get_y_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_yMotion", args = 1)]
    pub fn set_y_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_zMotion", args = 0)]
    pub fn get_z_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_zMotion", args = 1)]
    pub fn set_z_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_angularXMotion", args = 0)]
    pub fn get_angular_x_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_angularXMotion", args = 1)]
    pub fn set_angular_x_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_angularYMotion", args = 0)]
    pub fn get_angular_y_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_angularYMotion", args = 1)]
    pub fn set_angular_y_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_angularZMotion", args = 0)]
    pub fn get_angular_z_motion(
        self,
    ) -> crate::unity_engine::configurablejointmotion::ConfigurableJointMotion;

    #[method(name = "set_angularZMotion", args = 1)]
    pub fn set_angular_z_motion(
        self,
        value: crate::unity_engine::configurablejointmotion::ConfigurableJointMotion,
    ) -> ();

    #[method(name = "get_linearLimitSpring", args = 0)]
    pub fn get_linear_limit_spring(
        self,
    ) -> crate::unity_engine::softjointlimitspring::SoftJointLimitSpring;

    #[method(name = "set_linearLimitSpring", args = 1)]
    pub fn set_linear_limit_spring(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_angularXLimitSpring", args = 0)]
    pub fn get_angular_x_limit_spring(
        self,
    ) -> crate::unity_engine::softjointlimitspring::SoftJointLimitSpring;

    #[method(name = "set_angularXLimitSpring", args = 1)]
    pub fn set_angular_x_limit_spring(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_angularYZLimitSpring", args = 0)]
    pub fn get_angular_yz_limit_spring(
        self,
    ) -> crate::unity_engine::softjointlimitspring::SoftJointLimitSpring;

    #[method(name = "set_angularYZLimitSpring", args = 1)]
    pub fn set_angular_yz_limit_spring(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_linearLimit", args = 0)]
    pub fn get_linear_limit(self) -> crate::unity_engine::softjointlimit::SoftJointLimit;

    #[method(name = "set_linearLimit", args = 1)]
    pub fn set_linear_limit(self, value: crate::unity_engine::softjointlimit::SoftJointLimit)
        -> ();

    #[method(name = "get_lowAngularXLimit", args = 0)]
    pub fn get_low_angular_x_limit(self) -> crate::unity_engine::softjointlimit::SoftJointLimit;

    #[method(name = "set_lowAngularXLimit", args = 1)]
    pub fn set_low_angular_x_limit(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_highAngularXLimit", args = 0)]
    pub fn get_high_angular_x_limit(self) -> crate::unity_engine::softjointlimit::SoftJointLimit;

    #[method(name = "set_highAngularXLimit", args = 1)]
    pub fn set_high_angular_x_limit(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_angularYLimit", args = 0)]
    pub fn get_angular_y_limit(self) -> crate::unity_engine::softjointlimit::SoftJointLimit;

    #[method(name = "set_angularYLimit", args = 1)]
    pub fn set_angular_y_limit(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_angularZLimit", args = 0)]
    pub fn get_angular_z_limit(self) -> crate::unity_engine::softjointlimit::SoftJointLimit;

    #[method(name = "set_angularZLimit", args = 1)]
    pub fn set_angular_z_limit(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_targetPosition", args = 0)]
    pub fn get_target_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_targetPosition", args = 1)]
    pub fn set_target_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_targetVelocity", args = 0)]
    pub fn get_target_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_targetVelocity", args = 1)]
    pub fn set_target_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_xDrive", args = 0)]
    pub fn get_x_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_xDrive", args = 1)]
    pub fn set_x_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_yDrive", args = 0)]
    pub fn get_y_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_yDrive", args = 1)]
    pub fn set_y_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_zDrive", args = 0)]
    pub fn get_z_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_zDrive", args = 1)]
    pub fn set_z_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_targetRotation", args = 0)]
    pub fn get_target_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_targetRotation", args = 1)]
    pub fn set_target_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_targetAngularVelocity", args = 0)]
    pub fn get_target_angular_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_targetAngularVelocity", args = 1)]
    pub fn set_target_angular_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotationDriveMode", args = 0)]
    pub fn get_rotation_drive_mode(
        self,
    ) -> crate::unity_engine::rotationdrivemode::RotationDriveMode;

    #[method(name = "set_rotationDriveMode", args = 1)]
    pub fn set_rotation_drive_mode(
        self,
        value: crate::unity_engine::rotationdrivemode::RotationDriveMode,
    ) -> ();

    #[method(name = "get_angularXDrive", args = 0)]
    pub fn get_angular_x_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_angularXDrive", args = 1)]
    pub fn set_angular_x_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_angularYZDrive", args = 0)]
    pub fn get_angular_yz_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_angularYZDrive", args = 1)]
    pub fn set_angular_yz_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_slerpDrive", args = 0)]
    pub fn get_slerp_drive(self) -> crate::unity_engine::jointdrive::JointDrive;

    #[method(name = "set_slerpDrive", args = 1)]
    pub fn set_slerp_drive(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_projectionMode", args = 0)]
    pub fn get_projection_mode(
        self,
    ) -> crate::unity_engine::jointprojectionmode::JointProjectionMode;

    #[method(name = "set_projectionMode", args = 1)]
    pub fn set_projection_mode(
        self,
        value: crate::unity_engine::jointprojectionmode::JointProjectionMode,
    ) -> ();

    #[method(name = "get_projectionDistance", args = 0)]
    pub fn get_projection_distance(self) -> f32;

    #[method(name = "set_projectionDistance", args = 1)]
    pub fn set_projection_distance(self, value: f32) -> ();

    #[method(name = "get_projectionAngle", args = 0)]
    pub fn get_projection_angle(self) -> f32;

    #[method(name = "set_projectionAngle", args = 1)]
    pub fn set_projection_angle(self, value: f32) -> ();

    #[method(name = "get_configuredInWorldSpace", args = 0)]
    pub fn get_configured_in_world_space(self) -> bool;

    #[method(name = "set_configuredInWorldSpace", args = 1)]
    pub fn set_configured_in_world_space(self, value: bool) -> ();

    #[method(name = "get_swapBodies", args = 0)]
    pub fn get_swap_bodies(self) -> bool;

    #[method(name = "set_swapBodies", args = 1)]
    pub fn set_swap_bodies(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_secondaryAxis_Injected", args = 1)]
    pub fn get_secondary_axis_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_secondaryAxis_Injected", args = 1)]
    pub fn set_secondary_axis_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_linearLimitSpring_Injected", args = 1)]
    pub fn get_linear_limit_spring_injected(
        self,
        ret: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "set_linearLimitSpring_Injected", args = 1)]
    pub fn set_linear_limit_spring_injected(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_angularXLimitSpring_Injected", args = 1)]
    pub fn get_angular_x_limit_spring_injected(
        self,
        ret: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "set_angularXLimitSpring_Injected", args = 1)]
    pub fn set_angular_x_limit_spring_injected(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_angularYZLimitSpring_Injected", args = 1)]
    pub fn get_angular_yz_limit_spring_injected(
        self,
        ret: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "set_angularYZLimitSpring_Injected", args = 1)]
    pub fn set_angular_yz_limit_spring_injected(
        self,
        value: crate::unity_engine::softjointlimitspring::SoftJointLimitSpring,
    ) -> ();

    #[method(name = "get_linearLimit_Injected", args = 1)]
    pub fn get_linear_limit_injected(
        self,
        ret: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "set_linearLimit_Injected", args = 1)]
    pub fn set_linear_limit_injected(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_lowAngularXLimit_Injected", args = 1)]
    pub fn get_low_angular_x_limit_injected(
        self,
        ret: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "set_lowAngularXLimit_Injected", args = 1)]
    pub fn set_low_angular_x_limit_injected(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_highAngularXLimit_Injected", args = 1)]
    pub fn get_high_angular_x_limit_injected(
        self,
        ret: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "set_highAngularXLimit_Injected", args = 1)]
    pub fn set_high_angular_x_limit_injected(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_angularYLimit_Injected", args = 1)]
    pub fn get_angular_y_limit_injected(
        self,
        ret: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "set_angularYLimit_Injected", args = 1)]
    pub fn set_angular_y_limit_injected(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_angularZLimit_Injected", args = 1)]
    pub fn get_angular_z_limit_injected(
        self,
        ret: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "set_angularZLimit_Injected", args = 1)]
    pub fn set_angular_z_limit_injected(
        self,
        value: crate::unity_engine::softjointlimit::SoftJointLimit,
    ) -> ();

    #[method(name = "get_targetPosition_Injected", args = 1)]
    pub fn get_target_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_targetPosition_Injected", args = 1)]
    pub fn set_target_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_targetVelocity_Injected", args = 1)]
    pub fn get_target_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_targetVelocity_Injected", args = 1)]
    pub fn set_target_velocity_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_xDrive_Injected", args = 1)]
    pub fn get_x_drive_injected(self, ret: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "set_xDrive_Injected", args = 1)]
    pub fn set_x_drive_injected(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_yDrive_Injected", args = 1)]
    pub fn get_y_drive_injected(self, ret: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "set_yDrive_Injected", args = 1)]
    pub fn set_y_drive_injected(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_zDrive_Injected", args = 1)]
    pub fn get_z_drive_injected(self, ret: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "set_zDrive_Injected", args = 1)]
    pub fn set_z_drive_injected(self, value: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "get_targetRotation_Injected", args = 1)]
    pub fn get_target_rotation_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "set_targetRotation_Injected", args = 1)]
    pub fn set_target_rotation_injected(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_targetAngularVelocity_Injected", args = 1)]
    pub fn get_target_angular_velocity_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "set_targetAngularVelocity_Injected", args = 1)]
    pub fn set_target_angular_velocity_injected(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_angularXDrive_Injected", args = 1)]
    pub fn get_angular_x_drive_injected(
        self,
        ret: crate::unity_engine::jointdrive::JointDrive,
    ) -> ();

    #[method(name = "set_angularXDrive_Injected", args = 1)]
    pub fn set_angular_x_drive_injected(
        self,
        value: crate::unity_engine::jointdrive::JointDrive,
    ) -> ();

    #[method(name = "get_angularYZDrive_Injected", args = 1)]
    pub fn get_angular_yz_drive_injected(
        self,
        ret: crate::unity_engine::jointdrive::JointDrive,
    ) -> ();

    #[method(name = "set_angularYZDrive_Injected", args = 1)]
    pub fn set_angular_yz_drive_injected(
        self,
        value: crate::unity_engine::jointdrive::JointDrive,
    ) -> ();

    #[method(name = "get_slerpDrive_Injected", args = 1)]
    pub fn get_slerp_drive_injected(self, ret: crate::unity_engine::jointdrive::JointDrive) -> ();

    #[method(name = "set_slerpDrive_Injected", args = 1)]
    pub fn set_slerp_drive_injected(self, value: crate::unity_engine::jointdrive::JointDrive)
        -> ();
}

#[cfg(feature = "unity_engine-configurablejoint")]
impl ConfigurableJoint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConfigurableJoint),
                ::core::stringify!(new),
            )
        });
        <Self as IConfigurableJointMethods>::ctor(this);
        this
    }
}
