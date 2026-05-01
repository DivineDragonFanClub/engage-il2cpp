
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playabledirector/PlayableDirector.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "PlayableDirector")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct PlayableDirector {
    #[rename(name = "played")]
    pub played: crate::system::action_1::Action_1<
        crate::unity_engine::playables::playabledirector::PlayableDirector,
    >,
    #[rename(name = "paused")]
    pub paused: crate::system::action_1::Action_1<
        crate::unity_engine::playables::playabledirector::PlayableDirector,
    >,
    #[rename(name = "stopped")]
    pub stopped: crate::system::action_1::Action_1<
        crate::unity_engine::playables::playabledirector::PlayableDirector,
    >,
}

#[cfg(feature = "unity_engine-playables-playabledirector")]
#[::unity2::methods]
impl PlayableDirector {
    #[method(name = "get_state", args = 0)]
    pub fn get_state(self) -> crate::unity_engine::playables::playstate::PlayState;

    #[method(name = "set_extrapolationMode", args = 1)]
    pub fn set_extrapolation_mode(
        self,
        value: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> ();

    #[method(name = "get_extrapolationMode", args = 0)]
    pub fn get_extrapolation_mode(
        self,
    ) -> crate::unity_engine::playables::directorwrapmode::DirectorWrapMode;

    #[method(name = "get_playableAsset", args = 0)]
    pub fn get_playable_asset(self)
        -> crate::unity_engine::playables::playableasset::PlayableAsset;

    #[method(name = "set_playableAsset", args = 1)]
    pub fn set_playable_asset(
        self,
        value: crate::unity_engine::playables::playableasset::PlayableAsset,
    ) -> ();

    #[method(name = "get_playableGraph", args = 0)]
    pub fn get_playable_graph(self)
        -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "get_playOnAwake", args = 0)]
    pub fn get_play_on_awake(self) -> bool;

    #[method(name = "set_playOnAwake", args = 1)]
    pub fn set_play_on_awake(self, value: bool) -> ();

    #[method(name = "DeferredEvaluate", args = 0)]
    pub fn deferred_evaluate(self) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play(self, asset: crate::unity_engine::playables::playableasset::PlayableAsset) -> ();

    #[method(name = "Play", args = 2)]
    pub fn play_2(
        self,
        asset: crate::unity_engine::playables::playableasset::PlayableAsset,
        mode: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> ();

    #[method(name = "SetGenericBinding", args = 2)]
    pub fn set_generic_binding(
        self,
        key: crate::unity_engine::object_2::Object_2,
        value: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "set_timeUpdateMode", args = 1)]
    pub fn set_time_update_mode(
        self,
        value: crate::unity_engine::playables::directorupdatemode::DirectorUpdateMode,
    ) -> ();

    #[method(name = "get_timeUpdateMode", args = 0)]
    pub fn get_time_update_mode(
        self,
    ) -> crate::unity_engine::playables::directorupdatemode::DirectorUpdateMode;

    #[method(name = "set_time", args = 1)]
    pub fn set_time(self, value: f64) -> ();

    #[method(name = "get_time", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "set_initialTime", args = 1)]
    pub fn set_initial_time(self, value: f64) -> ();

    #[method(name = "get_initialTime", args = 0)]
    pub fn get_initial_time(self) -> f64;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f64;

    #[method(name = "Evaluate", args = 0)]
    pub fn evaluate(self) -> ();

    #[method(name = "Play", args = 0)]
    pub fn play_3(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Pause", args = 0)]
    pub fn pause(self) -> ();

    #[method(name = "Resume", args = 0)]
    pub fn resume(self) -> ();

    #[method(name = "RebuildGraph", args = 0)]
    pub fn rebuild_graph(self) -> ();

    #[method(name = "ClearReferenceValue", args = 1)]
    pub fn clear_reference_value(self, id: crate::unity_engine::propertyname::PropertyName) -> ();

    #[method(name = "SetReferenceValue", args = 2)]
    pub fn set_reference_value(
        self,
        id: crate::unity_engine::propertyname::PropertyName,
        value: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "GetReferenceValue", args = 2)]
    pub fn get_reference_value(
        self,
        id: crate::unity_engine::propertyname::PropertyName,
        id_valid: bool,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "GetGenericBinding", args = 1)]
    pub fn get_generic_binding(
        self,
        key: crate::unity_engine::object_2::Object_2,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "ClearGenericBinding", args = 1)]
    pub fn clear_generic_binding(self, key: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "RebindPlayableGraphOutputs", args = 0)]
    pub fn rebind_playable_graph_outputs(self) -> ();

    #[method(name = "ProcessPendingGraphChanges", args = 0)]
    pub fn process_pending_graph_changes(self) -> ();

    #[method(name = "HasGenericBinding", args = 1)]
    pub fn has_generic_binding(self, key: crate::unity_engine::object_2::Object_2) -> bool;

    #[method(name = "GetPlayState", args = 0)]
    pub fn get_play_state(self) -> crate::unity_engine::playables::playstate::PlayState;

    #[method(name = "SetWrapMode", args = 1)]
    pub fn set_wrap_mode(
        self,
        mode: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> ();

    #[method(name = "GetWrapMode", args = 0)]
    pub fn get_wrap_mode(
        self,
    ) -> crate::unity_engine::playables::directorwrapmode::DirectorWrapMode;

    #[method(name = "EvaluateNextFrame", args = 0)]
    pub fn evaluate_next_frame(self) -> ();

    #[method(name = "GetGraphHandle", args = 0)]
    pub fn get_graph_handle(self) -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "Internal_SetGenericBinding", args = 2)]
    pub fn internal_set_generic_binding(
        self,
        key: crate::unity_engine::object_2::Object_2,
        value: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "SetPlayableAsset", args = 1)]
    pub fn set_playable_asset_2(
        self,
        asset: crate::unity_engine::scriptableobject::ScriptableObject,
    ) -> ();

    #[method(name = "Internal_GetPlayableAsset", args = 0)]
    pub fn internal_get_playable_asset(
        self,
    ) -> crate::unity_engine::scriptableobject::ScriptableObject;

    #[method(name = "add_played", args = 1)]
    pub fn add_played(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "remove_played", args = 1)]
    pub fn remove_played(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "add_paused", args = 1)]
    pub fn add_paused(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "remove_paused", args = 1)]
    pub fn remove_paused(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "add_stopped", args = 1)]
    pub fn add_stopped(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "remove_stopped", args = 1)]
    pub fn remove_stopped(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::playables::playabledirector::PlayableDirector,
        >,
    ) -> ();

    #[method(name = "ResetFrameTiming", args = 0)]
    pub fn reset_frame_timing() -> ();

    #[method(name = "SendOnPlayableDirectorPlay", args = 0)]
    pub fn send_on_playable_director_play(self) -> ();

    #[method(name = "SendOnPlayableDirectorPause", args = 0)]
    pub fn send_on_playable_director_pause(self) -> ();

    #[method(name = "SendOnPlayableDirectorStop", args = 0)]
    pub fn send_on_playable_director_stop(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ClearReferenceValue_Injected", args = 1)]
    pub fn clear_reference_value_injected(
        self,
        id: crate::unity_engine::propertyname::PropertyName,
    ) -> ();

    #[method(name = "SetReferenceValue_Injected", args = 2)]
    pub fn set_reference_value_injected(
        self,
        id: crate::unity_engine::propertyname::PropertyName,
        value: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "GetReferenceValue_Injected", args = 2)]
    pub fn get_reference_value_injected(
        self,
        id: crate::unity_engine::propertyname::PropertyName,
        id_valid: bool,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "GetGraphHandle_Injected", args = 1)]
    pub fn get_graph_handle_injected(
        self,
        ret: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();
}

#[cfg(feature = "unity_engine-playables-playabledirector")]
impl PlayableDirector {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayableDirector),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayableDirectorMethods>::ctor(this);
        this
    }
}
