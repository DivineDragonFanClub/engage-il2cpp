
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/trackasset/TrackAsset.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TrackAsset")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct TrackAsset {
    #[static_field]
    #[rename(name = "k_LatestVersion")]
    pub k_latest_version: i32,
    #[rename(name = "m_Version")]
    pub m_version: i32,
    #[rename(name = "m_AnimClip")]
    pub m_anim_clip: crate::unity_engine::animationclip::AnimationClip,
    #[static_field]
    #[rename(name = "s_BuildData")]
    pub s_build_data: crate::unity_engine::timeline::trackasset::TrackAsset_TransientBuildData,
    #[static_field]
    #[rename(name = "kDefaultCurvesName")]
    pub k_default_curves_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "OnClipPlayableCreate")]
    pub on_clip_playable_create: crate::system::action_3::Action_3<
        crate::unity_engine::timeline::timelineclip::TimelineClip,
        crate::unity_engine::gameobject::GameObject,
        crate::unity_engine::playables::playable::Playable,
    >,
    #[static_field]
    #[rename(name = "OnTrackAnimationPlayableCreate")]
    pub on_track_animation_playable_create: crate::system::action_3::Action_3<
        crate::unity_engine::timeline::trackasset::TrackAsset,
        crate::unity_engine::gameobject::GameObject,
        crate::unity_engine::playables::playable::Playable,
    >,
    #[rename(name = "m_Locked")]
    pub m_locked: bool,
    #[rename(name = "m_Muted")]
    pub m_muted: bool,
    #[rename(name = "m_CustomPlayableFullTypename")]
    pub m_custom_playable_full_typename: ::unity2::Il2CppString,
    #[rename(name = "m_Curves")]
    pub m_curves: crate::unity_engine::animationclip::AnimationClip,
    #[rename(name = "m_Parent")]
    pub m_parent: crate::unity_engine::playables::playableasset::PlayableAsset,
    #[rename(name = "m_Children")]
    pub m_children: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >,
    #[rename(name = "m_ItemsHash")]
    pub m_items_hash: i32,
    #[rename(name = "m_ClipsCache")]
    pub m_clips_cache: ::unity2::Array<crate::unity_engine::timeline::timelineclip::TimelineClip>,
    #[rename(name = "m_Start")]
    pub m_start: crate::unity_engine::timeline::discretetime::DiscreteTime,
    #[rename(name = "m_End")]
    pub m_end: crate::unity_engine::timeline::discretetime::DiscreteTime,
    #[rename(name = "m_CacheSorted")]
    pub m_cache_sorted: bool,
    #[static_field]
    #[rename(name = "s_EmptyCache")]
    pub s_empty_cache: ::unity2::Array<crate::unity_engine::timeline::trackasset::TrackAsset>,
    #[rename(name = "m_ChildTrackCache")]
    pub m_child_track_cache: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >,
    #[static_field]
    #[rename(name = "s_TrackBindingTypeAttributeCache")]
    pub s_track_binding_type_attribute_cache:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::SystemType,
            crate::unity_engine::timeline::trackbindingtypeattribute::TrackBindingTypeAttribute,
        >,
    #[rename(name = "m_Clips")]
    pub m_clips: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::timelineclip::TimelineClip,
    >,
    #[rename(name = "m_Markers")]
    pub m_markers: crate::unity_engine::timeline::markerlist::MarkerList,
}

#[cfg(feature = "unity_engine-timeline-trackasset")]
#[::unity2::methods]
impl TrackAsset {
    #[method(name = "OnBeforeTrackSerialize", args = 0)]
    pub fn on_before_track_serialize(self) -> ();

    #[method(name = "OnAfterTrackDeserialize", args = 0)]
    pub fn on_after_track_deserialize(self) -> ();

    #[method(name = "OnUpgradeFromVersion", args = 1)]
    pub fn on_upgrade_from_version(self, old_version: i32) -> ();

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

    #[method(name = "UpgradeToLatestVersion", args = 0)]
    pub fn upgrade_to_latest_version(self) -> ();

    #[method(name = "add_OnClipPlayableCreate", args = 1)]
    pub fn add_on_clip_playable_create(
        value: crate::system::action_3::Action_3<
            crate::unity_engine::timeline::timelineclip::TimelineClip,
            crate::unity_engine::gameobject::GameObject,
            crate::unity_engine::playables::playable::Playable,
        >,
    ) -> ();

    #[method(name = "remove_OnClipPlayableCreate", args = 1)]
    pub fn remove_on_clip_playable_create(
        value: crate::system::action_3::Action_3<
            crate::unity_engine::timeline::timelineclip::TimelineClip,
            crate::unity_engine::gameobject::GameObject,
            crate::unity_engine::playables::playable::Playable,
        >,
    ) -> ();

    #[method(name = "add_OnTrackAnimationPlayableCreate", args = 1)]
    pub fn add_on_track_animation_playable_create(
        value: crate::system::action_3::Action_3<
            crate::unity_engine::timeline::trackasset::TrackAsset,
            crate::unity_engine::gameobject::GameObject,
            crate::unity_engine::playables::playable::Playable,
        >,
    ) -> ();

    #[method(name = "remove_OnTrackAnimationPlayableCreate", args = 1)]
    pub fn remove_on_track_animation_playable_create(
        value: crate::system::action_3::Action_3<
            crate::unity_engine::timeline::trackasset::TrackAsset,
            crate::unity_engine::gameobject::GameObject,
            crate::unity_engine::playables::playable::Playable,
        >,
    ) -> ();

    #[method(name = "get_start", args = 0)]
    pub fn get_start(self) -> f64;

    #[method(name = "get_end", args = 0)]
    pub fn get_end(self) -> f64;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f64;

    #[method(name = "get_muted", args = 0)]
    pub fn get_muted(self) -> bool;

    #[method(name = "set_muted", args = 1)]
    pub fn set_muted(self, value: bool) -> ();

    #[method(name = "get_mutedInHierarchy", args = 0)]
    pub fn get_muted_in_hierarchy(self) -> bool;

    #[method(name = "get_timelineAsset", args = 0)]
    pub fn get_timeline_asset(self) -> crate::unity_engine::timeline::timelineasset::TimelineAsset;

    #[method(name = "get_parent", args = 0)]
    pub fn get_parent(self) -> crate::unity_engine::playables::playableasset::PlayableAsset;

    #[method(name = "set_parent", args = 1)]
    pub fn set_parent(
        self,
        value: crate::unity_engine::playables::playableasset::PlayableAsset,
    ) -> ();

    #[method(name = "GetClips", args = 0)]
    pub fn get_clips(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::timelineclip::TimelineClip,
    >;

    #[method(name = "get_isEmpty", args = 0)]
    pub fn get_is_empty(self) -> bool;

    #[method(name = "get_hasClips", args = 0)]
    pub fn get_has_clips(self) -> bool;

    #[method(name = "get_hasCurves", args = 0)]
    pub fn get_has_curves(self) -> bool;

    #[method(name = "get_isSubTrack", args = 0)]
    pub fn get_is_sub_track(self) -> bool;

    #[method(name = "get_outputs", args = 0)]
    pub fn get_outputs(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::playables::playablebinding::PlayableBinding,
    >;

    #[method(name = "GetChildTracks", args = 0)]
    pub fn get_child_tracks(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >;

    #[method(name = "get_customPlayableTypename", args = 0)]
    pub fn get_custom_playable_typename(self) -> ::unity2::Il2CppString;

    #[method(name = "set_customPlayableTypename", args = 1)]
    pub fn set_custom_playable_typename(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_curves", args = 0)]
    pub fn get_curves(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_curves", args = 1)]
    pub fn set_curves(self, value: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(
        name = "UnityEngine.Timeline.ICurvesOwner.get_defaultCurvesName",
        args = 0
    )]
    pub fn unity_engine_timeline_i_curves_owner_get_default_curves_name(
        self,
    ) -> ::unity2::Il2CppString;

    #[method(name = "UnityEngine.Timeline.ICurvesOwner.get_asset", args = 0)]
    pub fn unity_engine_timeline_i_curves_owner_get_asset(
        self,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "UnityEngine.Timeline.ICurvesOwner.get_assetOwner", args = 0)]
    pub fn unity_engine_timeline_i_curves_owner_get_asset_owner(
        self,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "UnityEngine.Timeline.ICurvesOwner.get_targetTrack", args = 0)]
    pub fn unity_engine_timeline_i_curves_owner_get_target_track(
        self,
    ) -> crate::unity_engine::timeline::trackasset::TrackAsset;

    #[method(name = "get_subTracksObjects", args = 0)]
    pub fn get_sub_tracks_objects(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >;

    #[method(name = "get_locked", args = 0)]
    pub fn get_locked(self) -> bool;

    #[method(name = "set_locked", args = 1)]
    pub fn set_locked(self, value: bool) -> ();

    #[method(name = "get_lockedInHierarchy", args = 0)]
    pub fn get_locked_in_hierarchy(self) -> bool;

    #[method(name = "get_supportsNotifications", args = 0)]
    pub fn get_supports_notifications(self) -> bool;

    #[method(name = "__internalAwake", args = 0)]
    pub fn internal_awake(self) -> ();

    #[method(name = "CreateCurves", args = 1)]
    pub fn create_curves(self, curves_clip_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateTrackMixer", args = 3)]
    pub fn create_track_mixer(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        input_count: i32,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "CreatePlayable", args = 2)]
    pub fn create_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "CreateDefaultClip", args = 0)]
    pub fn create_default_clip(self) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "DeleteClip", args = 1)]
    pub fn delete_clip(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> bool;

    #[method(name = "CreateMarker", args = 2)]
    pub fn create_marker(
        self,
        r#type: ::unity2::SystemType,
        time: f64,
    ) -> crate::unity_engine::timeline::imarker_interface::IMarker_Interface;

    #[method(name = "DeleteMarker", args = 1)]
    pub fn delete_marker(
        self,
        marker: crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    ) -> bool;

    #[method(name = "GetMarkers", args = 0)]
    pub fn get_markers(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    >;

    #[method(name = "GetMarkerCount", args = 0)]
    pub fn get_marker_count(self) -> i32;

    #[method(name = "GetMarker", args = 1)]
    pub fn get_marker(
        self,
        idx: i32,
    ) -> crate::unity_engine::timeline::imarker_interface::IMarker_Interface;

    #[method(name = "CreateClip", args = 1)]
    pub fn create_clip(
        self,
        requested_type: ::unity2::SystemType,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "CreateAndAddNewClipOfType", args = 1)]
    pub fn create_and_add_new_clip_of_type(
        self,
        requested_type: ::unity2::SystemType,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "CreateClipOfType", args = 1)]
    pub fn create_clip_of_type(
        self,
        requested_type: ::unity2::SystemType,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "CreateClipFromPlayableAsset", args = 1)]
    pub fn create_clip_from_playable_asset(
        self,
        asset: crate::unity_engine::playables::iplayableasset_interface::IPlayableAsset_Interface,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "CreateClipFromAsset", args = 1)]
    pub fn create_clip_from_asset(
        self,
        playable_asset: crate::unity_engine::scriptableobject::ScriptableObject,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "GetMarkersRaw", args = 0)]
    pub fn get_markers_raw(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >;

    #[method(name = "ClearMarkers", args = 0)]
    pub fn clear_markers(self) -> ();

    #[method(name = "AddMarker", args = 1)]
    pub fn add_marker(self, e: crate::unity_engine::scriptableobject::ScriptableObject) -> ();

    #[method(name = "DeleteMarkerRaw", args = 1)]
    pub fn delete_marker_raw(
        self,
        marker: crate::unity_engine::scriptableobject::ScriptableObject,
    ) -> bool;

    #[method(name = "GetTimeRangeHash", args = 0)]
    pub fn get_time_range_hash(self) -> i32;

    #[method(name = "AddClip", args = 1)]
    pub fn add_clip(
        self,
        new_clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> ();

    #[method(name = "CreateNotificationsPlayable", args = 4)]
    pub fn create_notifications_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        mixer_playable: crate::unity_engine::playables::playable::Playable,
        go: crate::unity_engine::gameobject::GameObject,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "CreatePlayableGraph", args = 4)]
    pub fn create_playable_graph(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
        timeline_playable: crate::unity_engine::playables::playable::Playable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "CompileClips", args = 4)]
    pub fn compile_clips(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        timeline_clips: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::unity_engine::timeline::timelineclip::TimelineClip,
        >,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "GatherCompilableTracks", args = 1)]
    pub fn gather_compilable_tracks(
        self,
        tracks: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::unity_engine::timeline::trackasset::TrackAsset,
        >,
    ) -> ();

    #[method(name = "GatherNotificiations", args = 1)]
    pub fn gather_notificiations(
        self,
        markers: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
        >,
    ) -> ();

    #[method(name = "OnCreateClipPlayableGraph", args = 3)]
    pub fn on_create_clip_playable_graph(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "ConfigureTrackAnimation", args = 3)]
    pub fn configure_track_animation(
        self,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
        go: crate::unity_engine::gameobject::GameObject,
        blend: crate::unity_engine::playables::playable::Playable,
    ) -> ();

    #[method(name = "SortClips", args = 0)]
    pub fn sort_clips(self) -> ();

    #[method(name = "ClearClipsInternal", args = 0)]
    pub fn clear_clips_internal(self) -> ();

    #[method(name = "ClearSubTracksInternal", args = 0)]
    pub fn clear_sub_tracks_internal(self) -> ();

    #[method(name = "OnClipMove", args = 0)]
    pub fn on_clip_move(self) -> ();

    #[method(name = "CreateNewClipContainerInternal", args = 0)]
    pub fn create_new_clip_container_internal(
        self,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "AddChild", args = 1)]
    pub fn add_child(self, child: crate::unity_engine::timeline::trackasset::TrackAsset) -> ();

    #[method(name = "MoveLastTrackBefore", args = 1)]
    pub fn move_last_track_before(
        self,
        asset: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> ();

    #[method(name = "RemoveSubTrack", args = 1)]
    pub fn remove_sub_track(
        self,
        child: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> bool;

    #[method(name = "RemoveClip", args = 1)]
    pub fn remove_clip(self, clip: crate::unity_engine::timeline::timelineclip::TimelineClip)
        -> ();

    #[method(name = "GetEvaluationTime", args = 2)]
    pub fn get_evaluation_time(self, out_start: f64, out_duration: f64) -> ();

    #[method(name = "GetSequenceTime", args = 2)]
    pub fn get_sequence_time(self, out_start: f64, out_duration: f64) -> ();

    #[method(name = "GatherProperties", args = 2)]
    pub fn gather_properties(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
        driver: crate::unity_engine::timeline::ipropertycollector::IPropertyCollector,
    ) -> ();

    #[method(name = "GetGameObjectBinding", args = 1)]
    pub fn get_game_object_binding(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "ValidateClipType", args = 1)]
    pub fn validate_clip_type(self, clip_type: ::unity2::SystemType) -> bool;

    #[method(name = "OnCreateClip", args = 1)]
    pub fn on_create_clip(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> ();

    #[method(name = "UpdateDuration", args = 0)]
    pub fn update_duration(self) -> ();

    #[method(name = "CalculateItemsHash", args = 0)]
    pub fn calculate_items_hash(self) -> i32;

    #[method(name = "CreatePlayable", args = 3)]
    pub fn create_playable_2(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        game_object: crate::unity_engine::gameobject::GameObject,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "Invalidate", args = 0)]
    pub fn invalidate(self) -> ();

    #[method(name = "GetNotificationDuration", args = 0)]
    pub fn get_notification_duration(self) -> f64;

    #[method(name = "CanCompileClips", args = 0)]
    pub fn can_compile_clips(self) -> bool;

    #[method(name = "IsCompilable", args = 0)]
    pub fn is_compilable(self) -> bool;

    #[method(name = "UpdateChildTrackCache", args = 0)]
    pub fn update_child_track_cache(self) -> ();

    #[method(name = "Hash", args = 0)]
    pub fn hash(self) -> i32;

    #[method(name = "GetClipsHash", args = 0)]
    pub fn get_clips_hash(self) -> i32;

    #[method(name = "GetAnimationClipHash", args = 1)]
    pub fn get_animation_clip_hash(clip: crate::unity_engine::animationclip::AnimationClip) -> i32;

    #[method(name = "HasNotifications", args = 0)]
    pub fn has_notifications(self) -> bool;

    #[method(name = "CanCompileNotifications", args = 0)]
    pub fn can_compile_notifications(self) -> bool;

    #[method(name = "CanCompileClipsRecursive", args = 0)]
    pub fn can_compile_clips_recursive(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-timeline-trackasset")]
impl TrackAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TrackAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ITrackAssetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/trackasset/TrackAsset_TransientBuildData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TrackAsset_TransientBuildData {
    pub track_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::trackasset::TrackAsset,
    >,
    pub clip_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::timelineclip::TimelineClip,
    >,
    pub marker_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    >,
}

impl ::unity2::ClassIdentity for TrackAsset_TransientBuildData {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "TrackAsset.TransientBuildData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TrackAsset_TransientBuildData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-timeline-trackasset")]
#[::unity2::methods(value)]
impl TrackAsset_TransientBuildData {
    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::unity_engine::timeline::trackasset::TrackAsset_TransientBuildData;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}
