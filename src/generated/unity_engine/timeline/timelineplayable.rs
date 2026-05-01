
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::playables::playablebehaviour::IPlayableBehaviour;
use crate::unity_engine::playables::playablebehaviour::PlayableBehaviour;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelineplayable/TimelinePlayable.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimelinePlayable")]
#[parent(crate::unity_engine::playables::playablebehaviour::PlayableBehaviour)]
pub struct TimelinePlayable {
    #[rename(name = "m_IntervalTree")]
    pub m_interval_tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
        crate::unity_engine::timeline::runtimeelement::RuntimeElement,
    >,
    #[rename(name = "m_ActiveClips")]
    pub m_active_clips: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::runtimeelement::RuntimeElement,
    >,
    #[rename(name = "m_CurrentListOfActiveClips")]
    pub m_current_list_of_active_clips: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::runtimeelement::RuntimeElement,
    >,
    #[rename(name = "m_ActiveBit")]
    pub m_active_bit: i32,
    #[rename(name = "m_EvaluateCallbacks")]
    pub m_evaluate_callbacks: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::itimelineevaluatecallback::ITimelineEvaluateCallback,
    >,
    #[rename(name = "m_PlayableCache")]
    pub m_playable_cache: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::timeline::trackasset::TrackAsset,
        crate::unity_engine::playables::playable::Playable,
    >,
    #[static_field]
    #[rename(name = "muteAudioScrubbing")]
    pub mute_audio_scrubbing: bool,
}

#[cfg(feature = "unity_engine-timeline-timelineplayable")]
#[::unity2::methods]
impl TimelinePlayable {
    #[method(name = "Create", args = 5)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        tracks: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::unity_engine::timeline::trackasset::TrackAsset,
        >,
        go: crate::unity_engine::gameobject::GameObject,
        auto_rebalance: bool,
        create_outputs: bool,
    ) -> crate::unity_engine::playables::scriptplayable_1::ScriptPlayable_1<
        crate::unity_engine::timeline::timelineplayable::TimelinePlayable,
    >;

    #[method(name = "Compile", args = 6)]
    pub fn compile(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
        tracks: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::unity_engine::timeline::trackasset::TrackAsset,
        >,
        go: crate::unity_engine::gameobject::GameObject,
        auto_rebalance: bool,
        create_outputs: bool,
    ) -> ();

    #[method(name = "CompileTrackList", args = 5)]
    pub fn compile_track_list(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
        tracks: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::unity_engine::timeline::trackasset::TrackAsset,
        >,
        go: crate::unity_engine::gameobject::GameObject,
        create_outputs: bool,
    ) -> ();

    #[method(name = "CreateTrackOutput", args = 5)]
    pub fn create_track_output(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        go: crate::unity_engine::gameobject::GameObject,
        playable: crate::unity_engine::playables::playable::Playable,
        port: i32,
    ) -> ();

    #[method(name = "EvaluateWeightsForAnimationPlayableOutput", args = 2)]
    pub fn evaluate_weights_for_animation_playable_output(
        self,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        anim_output : crate :: unity_engine :: animations :: animationplayableoutput :: AnimationPlayableOutput,
    ) -> ();

    #[method(name = "CreatePlayableGraph", args = 5)]
    pub fn create_playable_graph(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        asset: crate::unity_engine::timeline::trackasset::TrackAsset,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "CreateTrackPlayable", args = 5)]
    pub fn create_track_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        go: crate::unity_engine::gameobject::GameObject,
        create_outputs: bool,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "PrepareFrame", args = 2)]
    pub fn prepare_frame(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "Evaluate", args = 2)]
    pub fn evaluate(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        frame_data: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "CacheTrack", args = 4)]
    pub fn cache_track(
        self,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        playable: crate::unity_engine::playables::playable::Playable,
        port: i32,
        parent: crate::unity_engine::playables::playable::Playable,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-timeline-timelineplayable")]
impl TimelinePlayable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TimelinePlayable),
                ::core::stringify!(new),
            )
        });
        <Self as ITimelinePlayableMethods>::ctor(this);
        this
    }
}
