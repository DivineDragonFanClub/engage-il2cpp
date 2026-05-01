
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/itextelement/ITextElement.md")))]
#[::unity2::class(namespace = "TMPro", name = "ITextElement")]
pub struct ITextElement {}

#[cfg(feature = "tm_pro-itextelement")]
#[::unity2::methods]
impl ITextElement {
    #[method(name = "get_sharedMaterial", args = 0)]
    pub fn get_shared_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, update: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "GetInstanceID", args = 0)]
    pub fn get_instance_id(self) -> i32;
}
