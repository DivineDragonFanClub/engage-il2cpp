
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/statemachinebehaviour/StateMachineBehaviour.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "StateMachineBehaviour")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct StateMachineBehaviour {}

#[cfg(feature = "unity_engine-statemachinebehaviour")]
#[::unity2::methods]
impl StateMachineBehaviour {
    #[method(name = "OnStateEnter", args = 3)]
    pub fn on_state_enter(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
    ) -> ();

    #[method(name = "OnStateUpdate", args = 3)]
    pub fn on_state_update(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
    ) -> ();

    #[method(name = "OnStateExit", args = 3)]
    pub fn on_state_exit(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
    ) -> ();

    #[method(name = "OnStateMove", args = 3)]
    pub fn on_state_move(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
    ) -> ();

    #[method(name = "OnStateIK", args = 3)]
    pub fn on_state_ik(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
    ) -> ();

    #[method(name = "OnStateMachineEnter", args = 2)]
    pub fn on_state_machine_enter(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_machine_path_hash: i32,
    ) -> ();

    #[method(name = "OnStateMachineExit", args = 2)]
    pub fn on_state_machine_exit(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_machine_path_hash: i32,
    ) -> ();

    #[method(name = "OnStateEnter", args = 4)]
    pub fn on_state_enter_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateUpdate", args = 4)]
    pub fn on_state_update_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateExit", args = 4)]
    pub fn on_state_exit_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateMove", args = 4)]
    pub fn on_state_move_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateIK", args = 4)]
    pub fn on_state_ik_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        layer_index: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateMachineEnter", args = 3)]
    pub fn on_state_machine_enter_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_machine_path_hash: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = "OnStateMachineExit", args = 3)]
    pub fn on_state_machine_exit_2(
        self,
        animator: crate::unity_engine::animator::Animator,
        state_machine_path_hash: i32,
        controller : crate :: unity_engine :: animations :: animatorcontrollerplayable :: AnimatorControllerPlayable,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-statemachinebehaviour")]
impl StateMachineBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StateMachineBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as IStateMachineBehaviourMethods>::ctor(this);
        this
    }
}
