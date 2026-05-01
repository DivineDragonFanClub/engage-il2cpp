
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/delegateutility/DelegateUtility.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "DelegateUtility")]
#[parent(crate::system::object::Object)]
pub struct DelegateUtility {}

#[cfg(feature = "unity_engine-rendering-delegateutility")]
#[::unity2::methods]
impl DelegateUtility {
    #[method(name = "Cast", args = 2)]
    pub fn cast(
        source: crate::system::delegate::Delegate,
        r#type: ::unity2::SystemType,
    ) -> crate::system::delegate::Delegate;
}
