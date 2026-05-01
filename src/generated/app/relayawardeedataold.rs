
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relayawardeedataold/RelayAwardeeDataOld.md")))]
#[::unity2::class(namespace = "App", name = "RelayAwardeeDataOld")]
#[parent(crate::system::object::Object)]
pub struct RelayAwardeeDataOld {}

#[cfg(feature = "app-relayawardeedataold")]
#[::unity2::methods]
impl RelayAwardeeDataOld {
    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();
}
