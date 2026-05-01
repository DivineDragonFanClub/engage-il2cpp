
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::marker::IMarker;
use crate::unity_engine::timeline::marker::Marker;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/signalemitter/SignalEmitter.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "SignalEmitter")]
#[parent(crate::unity_engine::timeline::marker::Marker)]
pub struct SignalEmitter {
    #[rename(name = "m_Retroactive")]
    pub m_retroactive: bool,
    #[rename(name = "m_EmitOnce")]
    pub m_emit_once: bool,
    #[rename(name = "m_Asset")]
    pub m_asset: crate::unity_engine::timeline::signalasset::SignalAsset,
}

#[cfg(feature = "unity_engine-timeline-signalemitter")]
#[::unity2::methods]
impl SignalEmitter {
    #[method(name = "get_retroactive", args = 0)]
    pub fn get_retroactive(self) -> bool;

    #[method(name = "set_retroactive", args = 1)]
    pub fn set_retroactive(self, value: bool) -> ();

    #[method(name = "get_emitOnce", args = 0)]
    pub fn get_emit_once(self) -> bool;

    #[method(name = "set_emitOnce", args = 1)]
    pub fn set_emit_once(self, value: bool) -> ();

    #[method(name = "get_asset", args = 0)]
    pub fn get_asset(self) -> crate::unity_engine::timeline::signalasset::SignalAsset;

    #[method(name = "set_asset", args = 1)]
    pub fn set_asset(self, value: crate::unity_engine::timeline::signalasset::SignalAsset) -> ();

    #[method(name = "UnityEngine.Playables.INotification.get_id", args = 0)]
    pub fn unity_engine_playables_i_notification_get_id(
        self,
    ) -> crate::unity_engine::propertyname::PropertyName;

    #[method(
        name = "UnityEngine.Timeline.INotificationOptionProvider.get_flags",
        args = 0
    )]
    pub fn unity_engine_timeline_i_notification_option_provider_get_flags(
        self,
    ) -> crate::unity_engine::timeline::notificationflags::NotificationFlags;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-signalemitter")]
impl SignalEmitter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SignalEmitter),
                ::core::stringify!(new),
            )
        });
        <Self as ISignalEmitterMethods>::ctor(this);
        this
    }
}
