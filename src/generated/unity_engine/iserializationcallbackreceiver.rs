
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/iserializationcallbackreceiver/ISerializationCallbackReceiver.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ISerializationCallbackReceiver")]
pub struct ISerializationCallbackReceiver {}

#[cfg(feature = "unity_engine-iserializationcallbackreceiver")]
#[::unity2::methods]
impl ISerializationCallbackReceiver {
    #[method(name = "OnBeforeSerialize", args = 0)]
    pub fn on_before_serialize(self) -> ();

    #[method(name = "OnAfterDeserialize", args = 0)]
    pub fn on_after_deserialize(self) -> ();
}
