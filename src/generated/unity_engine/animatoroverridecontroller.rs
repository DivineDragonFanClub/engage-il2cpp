
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::runtimeanimatorcontroller::IRuntimeAnimatorController;
use crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatoroverridecontroller/AnimatorOverrideController_OnOverrideControllerDirtyCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine",
    name = "AnimatorOverrideController.OnOverrideControllerDirtyCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AnimatorOverrideController_OnOverrideControllerDirtyCallback {}

#[cfg(feature = "unity_engine-animatoroverridecontroller")]
#[::unity2::methods]
impl AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-animatoroverridecontroller")]
impl AnimatorOverrideController_OnOverrideControllerDirtyCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimatorOverrideController_OnOverrideControllerDirtyCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimatorOverrideController_OnOverrideControllerDirtyCallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatoroverridecontroller/AnimatorOverrideController.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AnimatorOverrideController")]
#[parent(crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController)]
pub struct AnimatorOverrideController {
# [rename (name = "OnOverrideControllerDirty")] pub on_override_controller_dirty : crate :: unity_engine :: animatoroverridecontroller :: AnimatorOverrideController_OnOverrideControllerDirtyCallback ,
}

#[cfg(feature = "unity_engine-animatoroverridecontroller")]
#[::unity2::methods]
impl AnimatorOverrideController {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        controller: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> ();

    #[method(name = "Internal_Create", args = 2)]
    pub fn internal_create(
        self_: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
        controller: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> ();

    #[method(name = "get_runtimeAnimatorController", args = 0)]
    pub fn get_runtime_animator_controller(
        self,
    ) -> crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController;

    #[method(name = "set_runtimeAnimatorController", args = 1)]
    pub fn set_runtime_animator_controller(
        self,
        value: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "Internal_GetClipByName", args = 2)]
    pub fn internal_get_clip_by_name(
        self,
        name: ::unity2::Il2CppString,
        return_effective_clip: bool,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "Internal_SetClipByName", args = 2)]
    pub fn internal_set_clip_by_name(
        self,
        name: ::unity2::Il2CppString,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item_2(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item_2(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
        value: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "GetClip", args = 2)]
    pub fn get_clip(
        self,
        original_clip: crate::unity_engine::animationclip::AnimationClip,
        return_effective_clip: bool,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "SetClip", args = 3)]
    pub fn set_clip(
        self,
        original_clip: crate::unity_engine::animationclip::AnimationClip,
        override_clip: crate::unity_engine::animationclip::AnimationClip,
        notify: bool,
    ) -> ();

    #[method(name = "SendNotification", args = 0)]
    pub fn send_notification(self) -> ();

    #[method(name = "GetOriginalClip", args = 1)]
    pub fn get_original_clip(self, index: i32)
        -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "GetOverrideClip", args = 1)]
    pub fn get_override_clip(
        self,
        original_clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "get_overridesCount", args = 0)]
    pub fn get_overrides_count(self) -> i32;

    #[method(name = "GetOverrides", args = 1)]
    pub fn get_overrides(
        self,
        overrides: crate::system::collections::generic::list_1::List_1<
            crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<
                crate::unity_engine::animationclip::AnimationClip,
                crate::unity_engine::animationclip::AnimationClip,
            >,
        >,
    ) -> ();

    #[method(name = "ApplyOverrides", args = 1)]
    pub fn apply_overrides(
        self,
        overrides: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<
                crate::unity_engine::animationclip::AnimationClip,
                crate::unity_engine::animationclip::AnimationClip,
            >,
        >,
    ) -> ();

    #[method(name = "get_clips", args = 0)]
    pub fn get_clips(
        self,
    ) -> ::unity2::Array<crate::unity_engine::animationclippair::AnimationClipPair>;

    #[method(name = "set_clips", args = 1)]
    pub fn set_clips(
        self,
        value: ::unity2::Array<crate::unity_engine::animationclippair::AnimationClipPair>,
    ) -> ();

    #[method(name = "PerformOverrideClipListCleanup", args = 0)]
    pub fn perform_override_clip_list_cleanup(self) -> ();

    #[method(name = "OnInvalidateOverrideController", args = 1)]
    pub fn on_invalidate_override_controller(
        controller: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    ) -> ();
}

#[cfg(feature = "unity_engine-animatoroverridecontroller")]
impl AnimatorOverrideController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimatorOverrideController),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimatorOverrideControllerMethods>::ctor(this);
        this
    }

    pub fn new_2(
        controller: crate::unity_engine::runtimeanimatorcontroller::RuntimeAnimatorController,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimatorOverrideController),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAnimatorOverrideControllerMethods>::ctor_2(this, controller);
        this
    }
}
