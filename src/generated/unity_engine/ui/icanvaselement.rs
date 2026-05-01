
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/icanvaselement/ICanvasElement.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ICanvasElement")]
pub struct ICanvasElement {}

#[cfg(feature = "unity_engine-ui-icanvaselement")]
#[::unity2::methods]
impl ICanvasElement {
    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, executing: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "LayoutComplete", args = 0)]
    pub fn layout_complete(self) -> ();

    #[method(name = "GraphicUpdateComplete", args = 0)]
    pub fn graphic_update_complete(self) -> ();

    #[method(name = "IsDestroyed", args = 0)]
    pub fn is_destroyed(self) -> bool;
}
