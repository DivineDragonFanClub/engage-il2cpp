
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/humanposehandler/HumanPoseHandler.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "HumanPoseHandler")]
#[parent(crate::system::object::Object)]
pub struct HumanPoseHandler {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-humanposehandler")]
#[::unity2::methods]
impl HumanPoseHandler {
    #[method(name = "Internal_CreateFromRoot", args = 2)]
    pub fn internal_create_from_root(
        avatar: crate::unity_engine::avatar::Avatar,
        root: crate::unity_engine::transform::Transform,
    ) -> ::unity2::IntPtr;

    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "GetHumanPose", args = 3)]
    pub fn get_human_pose(
        self,
        body_position: crate::unity_engine::vector3::Vector3,
        body_rotation: crate::unity_engine::quaternion::Quaternion,
        muscles: ::unity2::Array<f32>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        avatar: crate::unity_engine::avatar::Avatar,
        root: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "GetHumanPose", args = 1)]
    pub fn get_human_pose_2(self, human_pose: crate::unity_engine::humanpose::HumanPose) -> ();
}

#[cfg(feature = "unity_engine-humanposehandler")]
impl HumanPoseHandler {
    pub fn new(
        avatar: crate::unity_engine::avatar::Avatar,
        root: crate::unity_engine::transform::Transform,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HumanPoseHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IHumanPoseHandlerMethods>::ctor(this, avatar, root);
        this
    }
}
