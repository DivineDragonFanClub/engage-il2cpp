
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playablehandle/PlayableHandle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PlayableHandle {
    pub m_handle: ::unity2::IntPtr,
    pub m_version: u32,
}

impl ::unity2::ClassIdentity for PlayableHandle {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "PlayableHandle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PlayableHandle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-playablehandle")]
#[::unity2::methods(value)]
impl PlayableHandle {
    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "GetInput", args = 1)]
    pub fn get_input(self, input_port: i32) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "SetInputWeight", args = 2)]
    pub fn set_input_weight(self, input_index: i32, weight: f32) -> bool;

    #[method(name = "GetInputWeight", args = 1)]
    pub fn get_input_weight(self, input_index: i32) -> f32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        x: crate::unity_engine::playables::playablehandle::PlayableHandle,
        y: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        x: crate::unity_engine::playables::playablehandle::PlayableHandle,
        y: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, p: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "CompareVersion", args = 2)]
    pub fn compare_version(
        lhs: crate::unity_engine::playables::playablehandle::PlayableHandle,
        rhs: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "CheckInputBounds", args = 1)]
    pub fn check_input_bounds(self, input_index: i32) -> bool;

    #[method(name = "CheckInputBounds", args = 2)]
    pub fn check_input_bounds_2(self, input_index: i32, accept_any: bool) -> bool;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "GetPlayableType", args = 0)]
    pub fn get_playable_type(self) -> ::unity2::SystemType;

    #[method(name = "SetScriptInstance", args = 1)]
    pub fn set_script_instance(self, script_instance: crate::system::object::Object) -> ();

    #[method(name = "GetPlayState", args = 0)]
    pub fn get_play_state(self) -> crate::unity_engine::playables::playstate::PlayState;

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> ();

    #[method(name = "Pause", args = 0)]
    pub fn pause(self) -> ();

    #[method(name = "GetSpeed", args = 0)]
    pub fn get_speed(self) -> f64;

    #[method(name = "SetSpeed", args = 1)]
    pub fn set_speed(self, value: f64) -> ();

    #[method(name = "GetTime", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "SetTime", args = 1)]
    pub fn set_time(self, value: f64) -> ();

    #[method(name = "IsDone", args = 0)]
    pub fn is_done(self) -> bool;

    #[method(name = "SetDone", args = 1)]
    pub fn set_done(self, value: bool) -> ();

    #[method(name = "GetDuration", args = 0)]
    pub fn get_duration(self) -> f64;

    #[method(name = "SetDuration", args = 1)]
    pub fn set_duration(self, value: f64) -> ();

    #[method(name = "SetPropagateSetTime", args = 1)]
    pub fn set_propagate_set_time(self, value: bool) -> ();

    #[method(name = "GetGraph", args = 0)]
    pub fn get_graph(self) -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "GetInputCount", args = 0)]
    pub fn get_input_count(self) -> i32;

    #[method(name = "SetInputCount", args = 1)]
    pub fn set_input_count(self, value: i32) -> ();

    #[method(name = "SetInputWeight", args = 2)]
    pub fn set_input_weight_2(
        self,
        input: crate::unity_engine::playables::playablehandle::PlayableHandle,
        weight: f32,
    ) -> ();

    #[method(name = "GetPreviousTime", args = 0)]
    pub fn get_previous_time(self) -> f64;

    #[method(name = "SetTraversalMode", args = 1)]
    pub fn set_traversal_mode(
        self,
        mode: crate::unity_engine::playables::playabletraversalmode::PlayableTraversalMode,
    ) -> ();

    #[method(name = "GetTimeWrapMode", args = 0)]
    pub fn get_time_wrap_mode(
        self,
    ) -> crate::unity_engine::playables::directorwrapmode::DirectorWrapMode;

    #[method(name = "SetTimeWrapMode", args = 1)]
    pub fn set_time_wrap_mode(
        self,
        mode: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> ();

    #[method(name = "GetScriptInstance", args = 0)]
    pub fn get_script_instance(self) -> crate::system::object::Object;

    #[method(name = "GetInputHandle", args = 1)]
    pub fn get_input_handle(
        self,
        index: i32,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "SetInputWeightFromIndex", args = 2)]
    pub fn set_input_weight_from_index(self, index: i32, weight: f32) -> ();

    #[method(name = "GetInputWeightFromIndex", args = 1)]
    pub fn get_input_weight_from_index(self, index: i32) -> f32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "IsValid_Injected", args = 1)]
    pub fn is_valid_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "GetPlayableType_Injected", args = 1)]
    pub fn get_playable_type_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ::unity2::SystemType;

    #[method(name = "SetScriptInstance_Injected", args = 2)]
    pub fn set_script_instance_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        script_instance: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetPlayState_Injected", args = 1)]
    pub fn get_play_state_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> crate::unity_engine::playables::playstate::PlayState;

    #[method(name = "Play_Injected", args = 1)]
    pub fn play_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "Pause_Injected", args = 1)]
    pub fn pause_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "GetSpeed_Injected", args = 1)]
    pub fn get_speed_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> f64;

    #[method(name = "SetSpeed_Injected", args = 2)]
    pub fn set_speed_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: f64,
    ) -> ();

    #[method(name = "GetTime_Injected", args = 1)]
    pub fn get_time_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> f64;

    #[method(name = "SetTime_Injected", args = 2)]
    pub fn set_time_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: f64,
    ) -> ();

    #[method(name = "IsDone_Injected", args = 1)]
    pub fn is_done_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "SetDone_Injected", args = 2)]
    pub fn set_done_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "GetDuration_Injected", args = 1)]
    pub fn get_duration_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> f64;

    #[method(name = "SetDuration_Injected", args = 2)]
    pub fn set_duration_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: f64,
    ) -> ();

    #[method(name = "SetPropagateSetTime_Injected", args = 2)]
    pub fn set_propagate_set_time_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "GetGraph_Injected", args = 2)]
    pub fn get_graph_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        ret: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "GetInputCount_Injected", args = 1)]
    pub fn get_input_count_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> i32;

    #[method(name = "SetInputCount_Injected", args = 2)]
    pub fn set_input_count_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: i32,
    ) -> ();

    #[method(name = "SetInputWeight_Injected", args = 3)]
    pub fn set_input_weight_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        input: crate::unity_engine::playables::playablehandle::PlayableHandle,
        weight: f32,
    ) -> ();

    #[method(name = "GetPreviousTime_Injected", args = 1)]
    pub fn get_previous_time_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> f64;

    #[method(name = "SetTraversalMode_Injected", args = 2)]
    pub fn set_traversal_mode_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        mode: crate::unity_engine::playables::playabletraversalmode::PlayableTraversalMode,
    ) -> ();

    #[method(name = "GetTimeWrapMode_Injected", args = 1)]
    pub fn get_time_wrap_mode_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> crate::unity_engine::playables::directorwrapmode::DirectorWrapMode;

    #[method(name = "SetTimeWrapMode_Injected", args = 2)]
    pub fn set_time_wrap_mode_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        mode: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> ();

    #[method(name = "GetScriptInstance_Injected", args = 1)]
    pub fn get_script_instance_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> crate::system::object::Object;

    #[method(name = "GetInputHandle_Injected", args = 3)]
    pub fn get_input_handle_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        index: i32,
        ret: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "SetInputWeightFromIndex_Injected", args = 3)]
    pub fn set_input_weight_from_index_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        index: i32,
        weight: f32,
    ) -> ();

    #[method(name = "GetInputWeightFromIndex_Injected", args = 2)]
    pub fn get_input_weight_from_index_injected(
        unity_self: crate::unity_engine::playables::playablehandle::PlayableHandle,
        index: i32,
    ) -> f32;
}
