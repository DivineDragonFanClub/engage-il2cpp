
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playablegraph/PlayableGraph.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PlayableGraph {
    pub m_handle: ::unity2::IntPtr,
    pub m_version: u32,
}

impl ::unity2::ClassIdentity for PlayableGraph {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "PlayableGraph";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PlayableGraph {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-playablegraph")]
#[::unity2::methods(value)]
impl PlayableGraph {
    #[method(name = "Evaluate", args = 0)]
    pub fn evaluate(self) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "Create", args = 1)]
    pub fn create_2(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::playables::playablegraph::PlayableGraph;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "Play", args = 0)]
    pub fn play(self) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate_2(self, delta_time: f32) -> ();

    #[method(name = "SetTimeUpdateMode", args = 1)]
    pub fn set_time_update_mode(
        self,
        value: crate::unity_engine::playables::directorupdatemode::DirectorUpdateMode,
    ) -> ();

    #[method(name = "GetResolver", args = 0)]
    pub fn get_resolver(self) -> crate::unity_engine::iexposedpropertytable::IExposedPropertyTable;

    #[method(name = "GetPlayableCount", args = 0)]
    pub fn get_playable_count(self) -> i32;

    #[method(name = "CreatePlayableHandle", args = 0)]
    pub fn create_playable_handle(
        self,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "CreateScriptOutputInternal", args = 2)]
    pub fn create_script_output_internal(
        self,
        name: ::unity2::Il2CppString,
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "ConnectInternal", args = 4)]
    pub fn connect_internal(
        self,
        source: crate::unity_engine::playables::playablehandle::PlayableHandle,
        source_output_port: i32,
        destination: crate::unity_engine::playables::playablehandle::PlayableHandle,
        destination_input_port: i32,
    ) -> bool;

    #[method(name = "DisconnectInternal", args = 2)]
    pub fn disconnect_internal(
        self,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
        input_port: i32,
    ) -> ();

    #[method(name = "DestroyPlayableInternal", args = 1)]
    pub fn destroy_playable_internal(
        self,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "DestroySubgraphInternal", args = 1)]
    pub fn destroy_subgraph_internal(
        self,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "Create_Injected", args = 2)]
    pub fn create_injected(
        name: ::unity2::Il2CppString,
        ret: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "Destroy_Injected", args = 1)]
    pub fn destroy_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "IsValid_Injected", args = 1)]
    pub fn is_valid_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> bool;

    #[method(name = "Play_Injected", args = 1)]
    pub fn play_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "Stop_Injected", args = 1)]
    pub fn stop_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> ();

    #[method(name = "Evaluate_Injected", args = 2)]
    pub fn evaluate_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        delta_time: f32,
    ) -> ();

    #[method(name = "SetTimeUpdateMode_Injected", args = 2)]
    pub fn set_time_update_mode_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        value: crate::unity_engine::playables::directorupdatemode::DirectorUpdateMode,
    ) -> ();

    #[method(name = "GetResolver_Injected", args = 1)]
    pub fn get_resolver_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> crate::unity_engine::iexposedpropertytable::IExposedPropertyTable;

    #[method(name = "GetPlayableCount_Injected", args = 1)]
    pub fn get_playable_count_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
    ) -> i32;

    #[method(name = "CreatePlayableHandle_Injected", args = 2)]
    pub fn create_playable_handle_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        ret: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "CreateScriptOutputInternal_Injected", args = 3)]
    pub fn create_script_output_internal_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        name: ::unity2::Il2CppString,
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "ConnectInternal_Injected", args = 5)]
    pub fn connect_internal_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        source: crate::unity_engine::playables::playablehandle::PlayableHandle,
        source_output_port: i32,
        destination: crate::unity_engine::playables::playablehandle::PlayableHandle,
        destination_input_port: i32,
    ) -> bool;

    #[method(name = "DisconnectInternal_Injected", args = 3)]
    pub fn disconnect_internal_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
        input_port: i32,
    ) -> ();

    #[method(name = "DestroyPlayableInternal_Injected", args = 2)]
    pub fn destroy_playable_internal_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "DestroySubgraphInternal_Injected", args = 2)]
    pub fn destroy_subgraph_internal_injected(
        unity_self: crate::unity_engine::playables::playablegraph::PlayableGraph,
        playable: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();
}
