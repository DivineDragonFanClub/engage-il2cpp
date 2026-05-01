
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/iplayereditorconnectionnative/IPlayerEditorConnectionNative.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "IPlayerEditorConnectionNative")]
pub struct IPlayerEditorConnectionNative {}

#[cfg(feature = "unity_engine-iplayereditorconnectionnative")]
#[::unity2::methods]
impl IPlayerEditorConnectionNative {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "DisconnectAll", args = 0)]
    pub fn disconnect_all(self) -> ();

    #[method(name = "Poll", args = 0)]
    pub fn poll(self) -> ();

    #[method(name = "IsConnected", args = 0)]
    pub fn is_connected(self) -> bool;
}
