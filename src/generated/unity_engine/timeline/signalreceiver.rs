
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/signalreceiver/SignalReceiver.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "SignalReceiver")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SignalReceiver {
    #[rename(name = "m_Events")]
    pub m_events: crate::unity_engine::timeline::signalreceiver::SignalReceiver_EventKeyValue,
}

#[cfg(feature = "unity_engine-timeline-signalreceiver")]
#[::unity2::methods]
impl SignalReceiver {
    #[method(name = "OnNotify", args = 3)]
    pub fn on_notify(
        self,
        origin: crate::unity_engine::playables::playable::Playable,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = "AddReaction", args = 2)]
    pub fn add_reaction(
        self,
        asset: crate::unity_engine::timeline::signalasset::SignalAsset,
        reaction: crate::unity_engine::events::unityevent::UnityEvent,
    ) -> ();

    #[method(name = "AddEmptyReaction", args = 1)]
    pub fn add_empty_reaction(
        self,
        reaction: crate::unity_engine::events::unityevent::UnityEvent,
    ) -> i32;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, asset: crate::unity_engine::timeline::signalasset::SignalAsset) -> ();

    #[method(name = "GetRegisteredSignals", args = 0)]
    pub fn get_registered_signals(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::unity_engine::timeline::signalasset::SignalAsset,
    >;

    #[method(name = "GetReaction", args = 1)]
    pub fn get_reaction(
        self,
        key: crate::unity_engine::timeline::signalasset::SignalAsset,
    ) -> crate::unity_engine::events::unityevent::UnityEvent;

    #[method(name = "Count", args = 0)]
    pub fn count(self) -> i32;

    #[method(name = "ChangeSignalAtIndex", args = 2)]
    pub fn change_signal_at_index(
        self,
        idx: i32,
        new_key: crate::unity_engine::timeline::signalasset::SignalAsset,
    ) -> ();

    #[method(name = "RemoveAtIndex", args = 1)]
    pub fn remove_at_index(self, idx: i32) -> ();

    #[method(name = "ChangeReactionAtIndex", args = 2)]
    pub fn change_reaction_at_index(
        self,
        idx: i32,
        reaction: crate::unity_engine::events::unityevent::UnityEvent,
    ) -> ();

    #[method(name = "GetReactionAtIndex", args = 1)]
    pub fn get_reaction_at_index(
        self,
        idx: i32,
    ) -> crate::unity_engine::events::unityevent::UnityEvent;

    #[method(name = "GetSignalAssetAtIndex", args = 1)]
    pub fn get_signal_asset_at_index(
        self,
        idx: i32,
    ) -> crate::unity_engine::timeline::signalasset::SignalAsset;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-signalreceiver")]
impl SignalReceiver {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SignalReceiver),
                ::core::stringify!(new),
            )
        });
        <Self as ISignalReceiverMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/signalreceiver/SignalReceiver_EventKeyValue.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "SignalReceiver.EventKeyValue"
)]
#[parent(crate::system::object::Object)]
pub struct SignalReceiver_EventKeyValue {
    #[rename(name = "m_Signals")]
    pub m_signals: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::signalasset::SignalAsset,
    >,
    #[rename(name = "m_Events")]
    pub m_events: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::unityevent::UnityEvent,
    >,
}

#[cfg(feature = "unity_engine-timeline-signalreceiver")]
#[::unity2::methods]
impl SignalReceiver_EventKeyValue {
    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(
        self,
        key: crate::unity_engine::timeline::signalasset::SignalAsset,
        value: crate::unity_engine::events::unityevent::UnityEvent,
    ) -> bool;

    #[method(name = "Append", args = 2)]
    pub fn append(
        self,
        key: crate::unity_engine::timeline::signalasset::SignalAsset,
        value: crate::unity_engine::events::unityevent::UnityEvent,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, idx: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove_2(self, key: crate::unity_engine::timeline::signalasset::SignalAsset) -> ();

    #[method(name = "get_signals", args = 0)]
    pub fn get_signals(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::timeline::signalasset::SignalAsset,
    >;

    #[method(name = "get_events", args = 0)]
    pub fn get_events(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::unityevent::UnityEvent,
    >;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-signalreceiver")]
impl SignalReceiver_EventKeyValue {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SignalReceiver_EventKeyValue),
                ::core::stringify!(new),
            )
        });
        <Self as ISignalReceiver_EventKeyValueMethods>::ctor(this);
        this
    }
}
