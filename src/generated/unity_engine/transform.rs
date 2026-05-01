
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/transform/Transform_Enumerator.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Transform.Enumerator")]
#[parent(crate::system::object::Object)]
pub struct Transform_Enumerator {
    #[rename(name = "outer")]
    pub outer: crate::unity_engine::transform::Transform,
    #[rename(name = "currentIndex")]
    pub current_index: i32,
}

#[cfg(feature = "unity_engine-transform")]
#[::unity2::methods]
impl Transform_Enumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, outer: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "unity_engine-transform")]
impl Transform_Enumerator {
    pub fn new(outer: crate::unity_engine::transform::Transform) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Transform_Enumerator),
                ::core::stringify!(new),
            )
        });
        <Self as ITransform_EnumeratorMethods>::ctor(this, outer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/transform/Transform.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Transform")]
#[parent(crate::unity_engine::component::Component)]
pub struct Transform {}

#[cfg(feature = "unity_engine-transform")]
#[::unity2::methods]
impl Transform {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_localPosition", args = 0)]
    pub fn get_local_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_localPosition", args = 1)]
    pub fn set_local_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetLocalEulerAngles", args = 1)]
    pub fn get_local_euler_angles(
        self,
        order: crate::unity_engine::rotationorder::RotationOrder,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetLocalEulerAngles", args = 2)]
    pub fn set_local_euler_angles(
        self,
        euler: crate::unity_engine::vector3::Vector3,
        order: crate::unity_engine::rotationorder::RotationOrder,
    ) -> ();

    #[method(name = "SetLocalEulerHint", args = 1)]
    pub fn set_local_euler_hint(self, euler: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_eulerAngles", args = 0)]
    pub fn get_euler_angles(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_eulerAngles", args = 1)]
    pub fn set_euler_angles(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_localEulerAngles", args = 0)]
    pub fn get_local_euler_angles_2(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_localEulerAngles", args = 1)]
    pub fn set_local_euler_angles_2(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_right", args = 0)]
    pub fn get_right(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_right", args = 1)]
    pub fn set_right(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_up", args = 0)]
    pub fn get_up(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_up", args = 1)]
    pub fn set_up(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_forward", args = 0)]
    pub fn get_forward(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_forward", args = 1)]
    pub fn set_forward(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_rotation", args = 1)]
    pub fn set_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_localRotation", args = 0)]
    pub fn get_local_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_localRotation", args = 1)]
    pub fn set_local_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_rotationOrder", args = 0)]
    pub fn get_rotation_order(self) -> crate::unity_engine::rotationorder::RotationOrder;

    #[method(name = "set_rotationOrder", args = 1)]
    pub fn set_rotation_order(self, value: crate::unity_engine::rotationorder::RotationOrder)
        -> ();

    #[method(name = "GetRotationOrderInternal", args = 0)]
    pub fn get_rotation_order_internal(self) -> i32;

    #[method(name = "SetRotationOrderInternal", args = 1)]
    pub fn set_rotation_order_internal(
        self,
        rotation_order: crate::unity_engine::rotationorder::RotationOrder,
    ) -> ();

    #[method(name = "get_localScale", args = 0)]
    pub fn get_local_scale(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_localScale", args = 1)]
    pub fn set_local_scale(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_parent", args = 0)]
    pub fn get_parent(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_parent", args = 1)]
    pub fn set_parent(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_parentInternal", args = 0)]
    pub fn get_parent_internal(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_parentInternal", args = 1)]
    pub fn set_parent_internal(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "SetParent", args = 2)]
    pub fn set_parent_2(
        self,
        parent: crate::unity_engine::transform::Transform,
        world_position_stays: bool,
    ) -> ();

    #[method(name = "get_worldToLocalMatrix", args = 0)]
    pub fn get_world_to_local_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_localToWorldMatrix", args = 0)]
    pub fn get_local_to_world_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "SetPositionAndRotation", args = 2)]
    pub fn set_position_and_rotation(
        self,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Translate", args = 2)]
    pub fn translate(
        self,
        translation: crate::unity_engine::vector3::Vector3,
        relative_to: crate::unity_engine::space::Space,
    ) -> ();

    #[method(name = "Translate", args = 1)]
    pub fn translate_2(self, translation: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Translate", args = 4)]
    pub fn translate_3(
        self,
        x: f32,
        y: f32,
        z: f32,
        relative_to: crate::unity_engine::space::Space,
    ) -> ();

    #[method(name = "Translate", args = 3)]
    pub fn translate_4(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "Translate", args = 2)]
    pub fn translate_5(
        self,
        translation: crate::unity_engine::vector3::Vector3,
        relative_to: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "Translate", args = 4)]
    pub fn translate_6(
        self,
        x: f32,
        y: f32,
        z: f32,
        relative_to: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "Rotate", args = 2)]
    pub fn rotate(
        self,
        eulers: crate::unity_engine::vector3::Vector3,
        relative_to: crate::unity_engine::space::Space,
    ) -> ();

    #[method(name = "Rotate", args = 1)]
    pub fn rotate_2(self, eulers: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Rotate", args = 4)]
    pub fn rotate_3(
        self,
        x_angle: f32,
        y_angle: f32,
        z_angle: f32,
        relative_to: crate::unity_engine::space::Space,
    ) -> ();

    #[method(name = "Rotate", args = 3)]
    pub fn rotate_4(self, x_angle: f32, y_angle: f32, z_angle: f32) -> ();

    #[method(name = "RotateAroundInternal", args = 2)]
    pub fn rotate_around_internal(
        self,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "Rotate", args = 3)]
    pub fn rotate_5(
        self,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
        relative_to: crate::unity_engine::space::Space,
    ) -> ();

    #[method(name = "Rotate", args = 2)]
    pub fn rotate_6(self, axis: crate::unity_engine::vector3::Vector3, angle: f32) -> ();

    #[method(name = "RotateAround", args = 3)]
    pub fn rotate_around(
        self,
        point: crate::unity_engine::vector3::Vector3,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "LookAt", args = 2)]
    pub fn look_at(
        self,
        target: crate::unity_engine::transform::Transform,
        world_up: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "LookAt", args = 1)]
    pub fn look_at_2(self, target: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "LookAt", args = 2)]
    pub fn look_at_3(
        self,
        world_position: crate::unity_engine::vector3::Vector3,
        world_up: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "LookAt", args = 1)]
    pub fn look_at_4(self, world_position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Internal_LookAt", args = 2)]
    pub fn internal_look_at(
        self,
        world_position: crate::unity_engine::vector3::Vector3,
        world_up: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "TransformDirection", args = 1)]
    pub fn transform_direction(
        self,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TransformDirection", args = 3)]
    pub fn transform_direction_2(
        self,
        x: f32,
        y: f32,
        z: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformDirection", args = 1)]
    pub fn inverse_transform_direction(
        self,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformDirection", args = 3)]
    pub fn inverse_transform_direction_2(
        self,
        x: f32,
        y: f32,
        z: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TransformVector", args = 1)]
    pub fn transform_vector(
        self,
        vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TransformVector", args = 3)]
    pub fn transform_vector_2(
        self,
        x: f32,
        y: f32,
        z: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformVector", args = 1)]
    pub fn inverse_transform_vector(
        self,
        vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformVector", args = 3)]
    pub fn inverse_transform_vector_2(
        self,
        x: f32,
        y: f32,
        z: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TransformPoint", args = 1)]
    pub fn transform_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "TransformPoint", args = 3)]
    pub fn transform_point_2(self, x: f32, y: f32, z: f32)
        -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformPoint", args = 1)]
    pub fn inverse_transform_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InverseTransformPoint", args = 3)]
    pub fn inverse_transform_point_2(
        self,
        x: f32,
        y: f32,
        z: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_root", args = 0)]
    pub fn get_root(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_childCount", args = 0)]
    pub fn get_child_count(self) -> i32;

    #[method(name = "DetachChildren", args = 0)]
    pub fn detach_children(self) -> ();

    #[method(name = "SetAsFirstSibling", args = 0)]
    pub fn set_as_first_sibling(self) -> ();

    #[method(name = "SetAsLastSibling", args = 0)]
    pub fn set_as_last_sibling(self) -> ();

    #[method(name = "SetSiblingIndex", args = 1)]
    pub fn set_sibling_index(self, index: i32) -> ();

    #[method(name = "MoveAfterSibling", args = 2)]
    pub fn move_after_sibling(
        self,
        transform: crate::unity_engine::transform::Transform,
        notify_editor_and_mark_dirty: bool,
    ) -> ();

    #[method(name = "GetSiblingIndex", args = 0)]
    pub fn get_sibling_index(self) -> i32;

    #[method(name = "FindRelativeTransformWithPath", args = 3)]
    pub fn find_relative_transform_with_path(
        transform: crate::unity_engine::transform::Transform,
        path: ::unity2::Il2CppString,
        is_active_only: bool,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "Find", args = 1)]
    pub fn find(self, n: ::unity2::Il2CppString) -> crate::unity_engine::transform::Transform;

    #[method(name = "SendTransformChangedScale", args = 0)]
    pub fn send_transform_changed_scale(self) -> ();

    #[method(name = "get_lossyScale", args = 0)]
    pub fn get_lossy_scale(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "IsChildOf", args = 1)]
    pub fn is_child_of(self, parent: crate::unity_engine::transform::Transform) -> bool;

    #[method(name = "get_hasChanged", args = 0)]
    pub fn get_has_changed(self) -> bool;

    #[method(name = "set_hasChanged", args = 1)]
    pub fn set_has_changed(self, value: bool) -> ();

    #[method(name = "FindChild", args = 1)]
    pub fn find_child(self, n: ::unity2::Il2CppString)
        -> crate::unity_engine::transform::Transform;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "RotateAround", args = 2)]
    pub fn rotate_around_2(self, axis: crate::unity_engine::vector3::Vector3, angle: f32) -> ();

    #[method(name = "RotateAroundLocal", args = 2)]
    pub fn rotate_around_local(self, axis: crate::unity_engine::vector3::Vector3, angle: f32)
        -> ();

    #[method(name = "GetChild", args = 1)]
    pub fn get_child(self, index: i32) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_hierarchyCapacity", args = 0)]
    pub fn get_hierarchy_capacity(self) -> i32;

    #[method(name = "set_hierarchyCapacity", args = 1)]
    pub fn set_hierarchy_capacity(self, value: i32) -> ();

    #[method(name = "internal_getHierarchyCapacity", args = 0)]
    pub fn internal_get_hierarchy_capacity(self) -> i32;

    #[method(name = "internal_setHierarchyCapacity", args = 1)]
    pub fn internal_set_hierarchy_capacity(self, value: i32) -> ();

    #[method(name = "get_hierarchyCount", args = 0)]
    pub fn get_hierarchy_count(self) -> i32;

    #[method(name = "internal_getHierarchyCount", args = 0)]
    pub fn internal_get_hierarchy_count(self) -> i32;

    #[method(name = "IsNonUniformScaleTransform", args = 0)]
    pub fn is_non_uniform_scale_transform(self) -> bool;

    #[method(name = "get_position_Injected", args = 1)]
    pub fn get_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_position_Injected", args = 1)]
    pub fn set_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_localPosition_Injected", args = 1)]
    pub fn get_local_position_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_localPosition_Injected", args = 1)]
    pub fn set_local_position_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetLocalEulerAngles_Injected", args = 2)]
    pub fn get_local_euler_angles_injected(
        self,
        order: crate::unity_engine::rotationorder::RotationOrder,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetLocalEulerAngles_Injected", args = 2)]
    pub fn set_local_euler_angles_injected(
        self,
        euler: crate::unity_engine::vector3::Vector3,
        order: crate::unity_engine::rotationorder::RotationOrder,
    ) -> ();

    #[method(name = "SetLocalEulerHint_Injected", args = 1)]
    pub fn set_local_euler_hint_injected(self, euler: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation_Injected", args = 1)]
    pub fn get_rotation_injected(self, ret: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "set_rotation_Injected", args = 1)]
    pub fn set_rotation_injected(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_localRotation_Injected", args = 1)]
    pub fn get_local_rotation_injected(
        self,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "set_localRotation_Injected", args = 1)]
    pub fn set_local_rotation_injected(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_localScale_Injected", args = 1)]
    pub fn get_local_scale_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_localScale_Injected", args = 1)]
    pub fn set_local_scale_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_worldToLocalMatrix_Injected", args = 1)]
    pub fn get_world_to_local_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_localToWorldMatrix_Injected", args = 1)]
    pub fn get_local_to_world_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetPositionAndRotation_Injected", args = 2)]
    pub fn set_position_and_rotation_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "RotateAroundInternal_Injected", args = 2)]
    pub fn rotate_around_internal_injected(
        self,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "Internal_LookAt_Injected", args = 2)]
    pub fn internal_look_at_injected(
        self,
        world_position: crate::unity_engine::vector3::Vector3,
        world_up: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "TransformDirection_Injected", args = 2)]
    pub fn transform_direction_injected(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "InverseTransformDirection_Injected", args = 2)]
    pub fn inverse_transform_direction_injected(
        self,
        direction: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "TransformVector_Injected", args = 2)]
    pub fn transform_vector_injected(
        self,
        vector: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "InverseTransformVector_Injected", args = 2)]
    pub fn inverse_transform_vector_injected(
        self,
        vector: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "TransformPoint_Injected", args = 2)]
    pub fn transform_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "InverseTransformPoint_Injected", args = 2)]
    pub fn inverse_transform_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_lossyScale_Injected", args = 1)]
    pub fn get_lossy_scale_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "RotateAround_Injected", args = 2)]
    pub fn rotate_around_injected(
        self,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();

    #[method(name = "RotateAroundLocal_Injected", args = 2)]
    pub fn rotate_around_local_injected(
        self,
        axis: crate::unity_engine::vector3::Vector3,
        angle: f32,
    ) -> ();
}

#[cfg(feature = "unity_engine-transform")]
impl Transform {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Transform),
                ::core::stringify!(new),
            )
        });
        <Self as ITransformMethods>::ctor(this);
        this
    }
}
