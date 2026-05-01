
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelineasset/TimelineAsset.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimelineAsset")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct TimelineAsset {
    #[static_field]
    #[rename(name = "k_LatestVersion")]
    pub k_latest_version: i32,
    #[rename(name = "m_Version")]
    pub m_version: i32,
    #[rename(name = "m_Tracks")]
    pub m_tracks: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >,
    #[rename(name = "m_FixedDuration")]
    pub m_fixed_duration: f64,
    #[rename(name = "m_CacheOutputTracks")]
    pub m_cache_output_tracks:
        ::unity2::Array<crate::unity_engine::timeline::trackasset::TrackAsset>,
    #[rename(name = "m_CacheRootTracks")]
    pub m_cache_root_tracks: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >,
    #[rename(name = "m_CacheFlattenedTracks")]
    pub m_cache_flattened_tracks: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >,
    #[rename(name = "m_EditorSettings")]
    pub m_editor_settings:
        crate::unity_engine::timeline::timelineasset::TimelineAsset_EditorSettings,
    #[rename(name = "m_DurationMode")]
    pub m_duration_mode: crate::unity_engine::timeline::timelineasset::TimelineAsset_DurationMode,
    #[rename(name = "m_MarkerTrack")]
    pub m_marker_track: crate::unity_engine::timeline::markertrack::MarkerTrack,
}

#[cfg(feature = "unity_engine-timeline-timelineasset")]
#[::unity2::methods]
impl TimelineAsset {
    #[method(name = "UpgradeToLatestVersion", args = 0)]
    pub fn upgrade_to_latest_version(self) -> ();

    #[method(name = "get_editorSettings", args = 0)]
    pub fn get_editor_settings(
        self,
    ) -> crate::unity_engine::timeline::timelineasset::TimelineAsset_EditorSettings;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f64;

    #[method(name = "get_fixedDuration", args = 0)]
    pub fn get_fixed_duration(self) -> f64;

    #[method(name = "set_fixedDuration", args = 1)]
    pub fn set_fixed_duration(self, value: f64) -> ();

    #[method(name = "get_durationMode", args = 0)]
    pub fn get_duration_mode(
        self,
    ) -> crate::unity_engine::timeline::timelineasset::TimelineAsset_DurationMode;

    #[method(name = "set_durationMode", args = 1)]
    pub fn set_duration_mode(
        self,
        value: crate::unity_engine::timeline::timelineasset::TimelineAsset_DurationMode,
    ) -> ();

    #[method(name = "get_outputs", args = 0)]
    pub fn get_outputs(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::playables::playablebinding::PlayableBinding,
    >;

    #[method(name = "get_clipCaps", args = 0)]
    pub fn get_clip_caps(self) -> crate::unity_engine::timeline::clipcaps::ClipCaps;

    #[method(name = "get_outputTrackCount", args = 0)]
    pub fn get_output_track_count(self) -> i32;

    #[method(name = "get_rootTrackCount", args = 0)]
    pub fn get_root_track_count(self) -> i32;

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "GetValidFramerate", args = 1)]
    pub fn get_valid_framerate(framerate: f32) -> f32;

    #[method(name = "GetRootTrack", args = 1)]
    pub fn get_root_track(
        self,
        index: i32,
    ) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "GetRootTracks", args = 0)]
    pub fn get_root_tracks(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >;

    #[method(name = "GetOutputTrack", args = 1)]
    pub fn get_output_track(
        self,
        index: i32,
    ) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "GetOutputTracks", args = 0)]
    pub fn get_output_tracks(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >;

    #[method(name = "UpdateRootTrackCache", args = 0)]
    pub fn update_root_track_cache(self) -> ();

    #[method(name = "UpdateOutputTrackCache", args = 0)]
    pub fn update_output_track_cache(self) -> ();

    #[method(name = "get_flattenedTracks", args = 0)]
    pub fn get_flattened_tracks(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >;

    #[method(name = "get_markerTrack", args = 0)]
    pub fn get_marker_track(self) -> crate::unity_engine::timeline::markertrack::MarkerTrack;

    #[method(name = "get_trackObjects", args = 0)]
    pub fn get_track_objects(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >;

    #[method(name = "AddTrackInternal", args = 1)]
    pub fn add_track_internal(
        self,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "RemoveTrack", args = 1)]
    pub fn remove_track(self, track: crate::unity_engine::timeline::trackasset::TrackAsset) -> ();

    #[method(name = "CreatePlayable", args = 2)]
    pub fn create_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_before_serialize(self) -> ();

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_after_deserialize(self) -> ();

    #[method(name = "__internalAwake", args = 0)]
    pub fn internal_awake(self) -> ();

    #[method(name = "GatherProperties", args = 2)]
    pub fn gather_properties(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
        driver: crate::unity_engine::timeline::ipropertycollector::IPropertyCollector,
    ) -> ();

    #[method(name = "CreateMarkerTrack", args = 0)]
    pub fn create_marker_track(self) -> ();

    #[method(name = "Invalidate", args = 0)]
    pub fn invalidate(self) -> ();

    #[method(name = "UpdateFixedDurationWithItemsDuration", args = 0)]
    pub fn update_fixed_duration_with_items_duration(self) -> ();

    #[method(name = "CalculateItemsDuration", args = 0)]
    pub fn calculate_items_duration(
        self,
    ) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "AddSubTracksRecursive", args = 2)]
    pub fn add_sub_tracks_recursive(
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
        all_tracks: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::timeline::trackasset::TrackAsset,
        >,
    ) -> ();

    #[method(name = "CreateTrack", args = 3)]
    pub fn create_track(
        self,
        r#type: ::unity2::SystemType,
        parent: crate::unity_engine::timeline::trackasset::TrackAsset,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "DeleteClip", args = 1)]
    pub fn delete_clip(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> bool;

    #[method(name = "DeleteTrack", args = 1)]
    pub fn delete_track(self, track: crate::unity_engine::timeline::trackasset::TrackAsset)
        -> bool;

    #[method(name = "MoveLastTrackBefore", args = 1)]
    pub fn move_last_track_before(
        self,
        asset: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "AllocateTrack", args = 3)]
    pub fn allocate_track(
        self,
        track_asset_parent: crate::unity_engine::timeline::trackasset::TrackAsset,
        track_name: ::unity2::Il2CppString,
        track_type: ::unity2::SystemType,
    ) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "DeleteRecordedAnimation", args = 1)]
    pub fn delete_recorded_animation(
        self,
        track: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "DeleteRecordedAnimation", args = 1)]
    pub fn delete_recorded_animation_2(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-timelineasset")]
impl TimelineAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TimelineAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ITimelineAssetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelineasset/TimelineAsset_DurationMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TimelineAsset_DurationMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TimelineAsset_DurationMode {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "TimelineAsset.DurationMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TimelineAsset_DurationMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TimelineAsset_DurationMode {
    pub fn based_on_clips() -> Self {
        Self { value: 0 }
    }

    pub fn fixed_length() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelineasset/TimelineAsset_EditorSettings.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "TimelineAsset.EditorSettings"
)]
#[parent(crate::system::object::Object)]
pub struct TimelineAsset_EditorSettings {
    #[static_field]
    #[rename(name = "kMinFps")]
    pub k_min_fps: f32,
    #[static_field]
    #[rename(name = "kMaxFps")]
    pub k_max_fps: f32,
    #[static_field]
    #[rename(name = "kDefaultFps")]
    pub k_default_fps: f32,
    #[rename(name = "m_Framerate")]
    pub m_framerate: f32,
    #[rename(name = "m_ScenePreview")]
    pub m_scene_preview: bool,
}

#[cfg(feature = "unity_engine-timeline-timelineasset")]
#[::unity2::methods]
impl TimelineAsset_EditorSettings {
    #[method(name = "get_fps", args = 0)]
    pub fn get_fps(self) -> f32;

    #[method(name = "set_fps", args = 1)]
    pub fn set_fps(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-timeline-timelineasset")]
impl TimelineAsset_EditorSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TimelineAsset_EditorSettings),
                ::core::stringify!(new),
            )
        });
        <Self as ITimelineAsset_EditorSettingsMethods>::ctor(this);
        this
    }
}
