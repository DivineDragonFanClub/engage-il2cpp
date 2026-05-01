
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::trackasset::ITrackAsset;
use crate::unity_engine::timeline::trackasset::TrackAsset;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/animationtrack/AnimationTrack_AnimationTrackUpgrade.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "AnimationTrack.AnimationTrackUpgrade"
)]
#[parent(crate::system::object::Object)]
pub struct AnimationTrack_AnimationTrackUpgrade {}

#[cfg(feature = "unity_engine-timeline-animationtrack")]
#[::unity2::methods]
impl AnimationTrack_AnimationTrackUpgrade {
    #[method(name = "ConvertRotationsToEuler", args = 1)]
    pub fn convert_rotations_to_euler(
        track: crate::unity_engine::timeline::animationtrack::AnimationTrack,
    ) -> ();

    #[method(name = "ConvertRootMotion", args = 1)]
    pub fn convert_root_motion(
        track: crate::unity_engine::timeline::animationtrack::AnimationTrack,
    ) -> ();

    #[method(name = "ConvertInfiniteTrack", args = 1)]
    pub fn convert_infinite_track(
        track: crate::unity_engine::timeline::animationtrack::AnimationTrack,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/animationtrack/AnimationTrack.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "AnimationTrack")]
#[parent(crate::unity_engine::timeline::trackasset::TrackAsset)]
pub struct AnimationTrack {
    #[static_field]
    #[rename(name = "k_DefaultInfiniteClipName")]
    pub k_default_infinite_clip_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "k_DefaultRecordableClipName")]
    pub k_default_recordable_clip_name: ::unity2::Il2CppString,
    #[rename(name = "m_InfiniteClipPreExtrapolation")]
    pub m_infinite_clip_pre_extrapolation:
        crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    #[rename(name = "m_InfiniteClipPostExtrapolation")]
    pub m_infinite_clip_post_extrapolation:
        crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    #[rename(name = "m_InfiniteClipOffsetPosition")]
    pub m_infinite_clip_offset_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_InfiniteClipOffsetEulerAngles")]
    pub m_infinite_clip_offset_euler_angles: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_InfiniteClipTimeOffset")]
    pub m_infinite_clip_time_offset: f64,
    #[rename(name = "m_InfiniteClipRemoveOffset")]
    pub m_infinite_clip_remove_offset: bool,
    #[rename(name = "m_InfiniteClipApplyFootIK")]
    pub m_infinite_clip_apply_foot_ik: bool,
    #[rename(name = "mInfiniteClipLoop")]
    pub m_infinite_clip_loop:
        crate::unity_engine::timeline::animationplayableasset::AnimationPlayableAsset_LoopMode,
    #[rename(name = "m_MatchTargetFields")]
    pub m_match_target_fields: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
    #[rename(name = "m_Position")]
    pub m_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_EulerAngles")]
    pub m_euler_angles: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_AvatarMask")]
    pub m_avatar_mask: crate::unity_engine::avatarmask::AvatarMask,
    #[rename(name = "m_ApplyAvatarMask")]
    pub m_apply_avatar_mask: bool,
    #[rename(name = "m_TrackOffset")]
    pub m_track_offset: crate::unity_engine::timeline::trackoffset::TrackOffset,
    #[rename(name = "m_InfiniteClip")]
    pub m_infinite_clip: crate::unity_engine::animationclip::AnimationClip,
    #[static_field]
    #[rename(name = "s_CachedQueue")]
    pub s_cached_queue: crate::system::collections::generic::queue_1::Queue_1<
        crate::unity_engine::transform::Transform,
    >,
    #[rename(name = "m_OpenClipOffsetRotation")]
    pub m_open_clip_offset_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_Rotation")]
    pub m_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_ApplyOffsets")]
    pub m_apply_offsets: bool,
}

#[cfg(feature = "unity_engine-timeline-animationtrack")]
#[::unity2::methods]
impl AnimationTrack {
    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_rotation", args = 1)]
    pub fn set_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "get_eulerAngles", args = 0)]
    pub fn get_euler_angles(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_eulerAngles", args = 1)]
    pub fn set_euler_angles(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_applyOffsets", args = 0)]
    pub fn get_apply_offsets(self) -> bool;

    #[method(name = "set_applyOffsets", args = 1)]
    pub fn set_apply_offsets(self, value: bool) -> ();

    #[method(name = "get_trackOffset", args = 0)]
    pub fn get_track_offset(self) -> crate::unity_engine::timeline::trackoffset::TrackOffset;

    #[method(name = "set_trackOffset", args = 1)]
    pub fn set_track_offset(
        self,
        value: crate::unity_engine::timeline::trackoffset::TrackOffset,
    ) -> ();

    #[method(name = "get_matchTargetFields", args = 0)]
    pub fn get_match_target_fields(
        self,
    ) -> crate::unity_engine::timeline::matchtargetfields::MatchTargetFields;

    #[method(name = "set_matchTargetFields", args = 1)]
    pub fn set_match_target_fields(
        self,
        value: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
    ) -> ();

    #[method(name = "get_infiniteClip", args = 0)]
    pub fn get_infinite_clip(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "set_infiniteClip", args = 1)]
    pub fn set_infinite_clip(self, value: crate::unity_engine::animationclip::AnimationClip) -> ();

    #[method(name = "get_infiniteClipRemoveOffset", args = 0)]
    pub fn get_infinite_clip_remove_offset(self) -> bool;

    #[method(name = "set_infiniteClipRemoveOffset", args = 1)]
    pub fn set_infinite_clip_remove_offset(self, value: bool) -> ();

    #[method(name = "get_avatarMask", args = 0)]
    pub fn get_avatar_mask(self) -> crate::unity_engine::avatarmask::AvatarMask;

    #[method(name = "set_avatarMask", args = 1)]
    pub fn set_avatar_mask(self, value: crate::unity_engine::avatarmask::AvatarMask) -> ();

    #[method(name = "get_applyAvatarMask", args = 0)]
    pub fn get_apply_avatar_mask(self) -> bool;

    #[method(name = "set_applyAvatarMask", args = 1)]
    pub fn set_apply_avatar_mask(self, value: bool) -> ();

    #[method(name = "CanCompileClips", args = 0)]
    pub fn can_compile_clips(self) -> bool;

    #[method(name = "get_outputs", args = 0)]
    pub fn get_outputs(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::playables::playablebinding::PlayableBinding,
    >;

    #[method(name = "get_inClipMode", args = 0)]
    pub fn get_in_clip_mode(self) -> bool;

    #[method(name = "get_infiniteClipOffsetPosition", args = 0)]
    pub fn get_infinite_clip_offset_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_infiniteClipOffsetPosition", args = 1)]
    pub fn set_infinite_clip_offset_position(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_infiniteClipOffsetRotation", args = 0)]
    pub fn get_infinite_clip_offset_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_infiniteClipOffsetRotation", args = 1)]
    pub fn set_infinite_clip_offset_rotation(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_infiniteClipOffsetEulerAngles", args = 0)]
    pub fn get_infinite_clip_offset_euler_angles(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_infiniteClipOffsetEulerAngles", args = 1)]
    pub fn set_infinite_clip_offset_euler_angles(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_infiniteClipApplyFootIK", args = 0)]
    pub fn get_infinite_clip_apply_foot_ik(self) -> bool;

    #[method(name = "set_infiniteClipApplyFootIK", args = 1)]
    pub fn set_infinite_clip_apply_foot_ik(self, value: bool) -> ();

    #[method(name = "get_infiniteClipTimeOffset", args = 0)]
    pub fn get_infinite_clip_time_offset(self) -> f64;

    #[method(name = "set_infiniteClipTimeOffset", args = 1)]
    pub fn set_infinite_clip_time_offset(self, value: f64) -> ();

    #[method(name = "get_infiniteClipPreExtrapolation", args = 0)]
    pub fn get_infinite_clip_pre_extrapolation(
        self,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation;

    #[method(name = "set_infiniteClipPreExtrapolation", args = 1)]
    pub fn set_infinite_clip_pre_extrapolation(
        self,
        value: crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    ) -> ();

    #[method(name = "get_infiniteClipPostExtrapolation", args = 0)]
    pub fn get_infinite_clip_post_extrapolation(
        self,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation;

    #[method(name = "set_infiniteClipPostExtrapolation", args = 1)]
    pub fn set_infinite_clip_post_extrapolation(
        self,
        value: crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    ) -> ();

    #[method(name = "get_infiniteClipLoop", args = 0)]
    pub fn get_infinite_clip_loop(
        self,
    ) -> crate::unity_engine::timeline::animationplayableasset::AnimationPlayableAsset_LoopMode;

    #[method(name = "set_infiniteClipLoop", args = 1)]
    pub fn set_infinite_clip_loop(
        self,
        value : crate :: unity_engine :: timeline :: animationplayableasset :: AnimationPlayableAsset_LoopMode,
    ) -> ();

    #[method(name = "ResetOffsets", args = 0)]
    pub fn reset_offsets(self) -> ();

    #[method(name = "CreateClip", args = 1)]
    pub fn create_clip(
        self,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "CreateInfiniteClip", args = 1)]
    pub fn create_infinite_clip(self, infinite_clip_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateRecordableClip", args = 1)]
    pub fn create_recordable_clip(
        self,
        anim_clip_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip;

    #[method(name = "OnCreateClip", args = 1)]
    pub fn on_create_clip(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
    ) -> ();

    #[method(name = "CalculateItemsHash", args = 0)]
    pub fn calculate_items_hash(self) -> i32;

    #[method(name = "UpdateClipOffsets", args = 0)]
    pub fn update_clip_offsets(self) -> ();

    #[method(name = "CompileTrackPlayable", args = 5)]
    pub fn compile_track_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        track: crate::unity_engine::timeline::animationtrack::AnimationTrack,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
        mode: crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "UnityEngine.Timeline.ILayerable.CreateLayerMixer", args = 3)]
    pub fn unity_engine_timeline_i_layerable_create_layer_mixer(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        input_count: i32,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "OnCreateClipPlayableGraph", args = 3)]
    pub fn on_create_clip_playable_graph(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "GetDefaultBlendCount", args = 0)]
    pub fn get_default_blend_count(self) -> i32;

    #[method(name = "AttachDefaultBlend", args = 3)]
    pub fn attach_default_blend(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        mixer : crate :: unity_engine :: animations :: animationlayermixerplayable :: AnimationLayerMixerPlayable,
        require_offset: bool,
    ) -> ();

    #[method(name = "AttachOffsetPlayable", args = 4)]
    pub fn attach_offset_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        playable: crate::unity_engine::playables::playable::Playable,
        pos: crate::unity_engine::vector3::Vector3,
        rot: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "RequiresMotionXPlayable", args = 2)]
    pub fn requires_motion_x_playable(
        self,
        mode: crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "UsesAbsoluteMotion", args = 1)]
    pub fn uses_absolute_motion(
        mode: crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode,
    ) -> bool;

    #[method(name = "HasController", args = 1)]
    pub fn has_controller(self, game_object: crate::unity_engine::gameobject::GameObject) -> bool;

    #[method(name = "GetBinding", args = 1)]
    pub fn get_binding(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
    ) -> crate::unity_engine::animator::Animator;

    #[method(name = "CreateGroupMixer", args = 3)]
    pub fn create_group_mixer(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        input_count: i32,
    ) -> crate::unity_engine::animations::animationlayermixerplayable::AnimationLayerMixerPlayable;

    #[method(name = "CreateInfiniteTrackPlayable", args = 4)]
    pub fn create_infinite_track_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        tree: crate::unity_engine::timeline::intervaltree_1::IntervalTree_1<
            crate::unity_engine::timeline::runtimeelement::RuntimeElement,
        >,
        mode: crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "ApplyTrackOffset", args = 4)]
    pub fn apply_track_offset(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        root: crate::unity_engine::playables::playable::Playable,
        go: crate::unity_engine::gameobject::GameObject,
        mode: crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "GetEvaluationTime", args = 2)]
    pub fn get_evaluation_time(self, out_start: f64, out_duration: f64) -> ();

    #[method(name = "GetSequenceTime", args = 2)]
    pub fn get_sequence_time(self, out_start: f64, out_duration: f64) -> ();

    #[method(name = "AssignAnimationClip", args = 2)]
    pub fn assign_animation_clip(
        self,
        clip: crate::unity_engine::timeline::timelineclip::TimelineClip,
        anim_clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> ();

    #[method(name = "GatherProperties", args = 2)]
    pub fn gather_properties(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
        driver: crate::unity_engine::timeline::ipropertycollector::IPropertyCollector,
    ) -> ();

    #[method(name = "GetAnimationClips", args = 1)]
    pub fn get_animation_clips(
        self,
        anim_clips: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animationclip::AnimationClip,
        >,
    ) -> ();

    #[method(name = "GetOffsetMode", args = 2)]
    pub fn get_offset_mode(
        self,
        go: crate::unity_engine::gameobject::GameObject,
        animates_root_transform: bool,
    ) -> crate::unity_engine::timeline::appliedoffsetmode::AppliedOffsetMode;

    #[method(name = "IsRootTransformDisabledByMask", args = 2)]
    pub fn is_root_transform_disabled_by_mask(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
        generic_root_node: crate::unity_engine::transform::Transform,
    ) -> bool;

    #[method(name = "GetGenericRootNode", args = 1)]
    pub fn get_generic_root_node(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "AnimatesRootTransform", args = 0)]
    pub fn animates_root_transform(self) -> bool;

    #[method(name = "FindInHierarchyBreadthFirst", args = 2)]
    pub fn find_in_hierarchy_breadth_first(
        t: crate::unity_engine::transform::Transform,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_openClipOffsetPosition", args = 0)]
    pub fn get_open_clip_offset_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_openClipOffsetPosition", args = 1)]
    pub fn set_open_clip_offset_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_openClipOffsetRotation", args = 0)]
    pub fn get_open_clip_offset_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_openClipOffsetRotation", args = 1)]
    pub fn set_open_clip_offset_rotation(
        self,
        value: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "get_openClipOffsetEulerAngles", args = 0)]
    pub fn get_open_clip_offset_euler_angles(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_openClipOffsetEulerAngles", args = 1)]
    pub fn set_open_clip_offset_euler_angles(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_openClipPreExtrapolation", args = 0)]
    pub fn get_open_clip_pre_extrapolation(
        self,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation;

    #[method(name = "set_openClipPreExtrapolation", args = 1)]
    pub fn set_open_clip_pre_extrapolation(
        self,
        value: crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    ) -> ();

    #[method(name = "get_openClipPostExtrapolation", args = 0)]
    pub fn get_open_clip_post_extrapolation(
        self,
    ) -> crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation;

    #[method(name = "set_openClipPostExtrapolation", args = 1)]
    pub fn set_open_clip_post_extrapolation(
        self,
        value: crate::unity_engine::timeline::timelineclip::TimelineClip_ClipExtrapolation,
    ) -> ();

    #[method(name = "OnUpgradeFromVersion", args = 1)]
    pub fn on_upgrade_from_version(self, old_version: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-timeline-animationtrack")]
impl AnimationTrack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimationTrack),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimationTrackMethods>::ctor(this);
        this
    }
}
