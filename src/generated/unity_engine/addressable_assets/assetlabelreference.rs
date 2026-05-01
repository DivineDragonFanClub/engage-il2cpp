
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/assetlabelreference/AssetLabelReference.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AssetLabelReference"
)]
#[parent(crate::system::object::Object)]
pub struct AssetLabelReference {
    #[rename(name = "m_LabelString")]
    pub m_label_string: ::unity2::Il2CppString,
}

#[cfg(feature = "unity_engine-addressable_assets-assetlabelreference")]
#[::unity2::methods]
impl AssetLabelReference {
    #[method(name = "get_labelString", args = 0)]
    pub fn get_label_string(self) -> ::unity2::Il2CppString;

    #[method(name = "set_labelString", args = 1)]
    pub fn set_label_string(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RuntimeKey", args = 0)]
    pub fn get_runtime_key(self) -> crate::system::object::Object;

    #[method(name = "RuntimeKeyIsValid", args = 0)]
    pub fn runtime_key_is_valid(self) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-assetlabelreference")]
impl AssetLabelReference {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetLabelReference),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetLabelReferenceMethods>::ctor(this);
        this
    }
}
