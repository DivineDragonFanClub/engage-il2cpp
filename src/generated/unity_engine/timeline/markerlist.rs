
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/markerlist/MarkerList.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MarkerList {
    pub m_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >,
    pub m_cache: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    >,
    pub m_cache_dirty: bool,
    pub m_has_notifications: bool,
}

impl ::unity2::ClassIdentity for MarkerList {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "MarkerList";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MarkerList {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-timeline-markerlist")]
#[::unity2::methods(value)]
impl MarkerList {
    #[method(name = "get_markers", args = 0)]
    pub fn get_markers(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    >;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: crate::unity_engine::scriptableobject::ScriptableObject) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(
        self,
        item: crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
    ) -> bool;

    #[method(name = "Remove", args = 3)]
    pub fn remove_2(
        self,
        item: crate::unity_engine::scriptableobject::ScriptableObject,
        timeline_asset: crate::unity_engine::timeline::timelineasset::TimelineAsset,
        thing_to_dirty: crate::unity_engine::playables::playableasset::PlayableAsset,
    ) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        idx: i32,
    ) -> crate::unity_engine::timeline::imarker_interface::IMarker_Interface;

    #[method(name = "GetRawMarkerList", args = 0)]
    pub fn get_raw_marker_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::scriptableobject::ScriptableObject,
    >;

    #[method(name = "CreateMarker", args = 3)]
    pub fn create_marker(
        self,
        r#type: ::unity2::SystemType,
        time: f64,
        owner: crate::unity_engine::timeline::trackasset::TrackAsset,
    ) -> crate::unity_engine::timeline::imarker_interface::IMarker_Interface;

    #[method(name = "HasNotifications", args = 0)]
    pub fn has_notifications(self) -> bool;

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

    #[method(name = "BuildCache", args = 0)]
    pub fn build_cache(self) -> ();
}
