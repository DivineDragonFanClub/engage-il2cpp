
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/imaterialmodifier/IMaterialModifier.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "IMaterialModifier")]
pub struct IMaterialModifier {}

#[cfg(feature = "unity_engine-ui-imaterialmodifier")]
#[::unity2::methods]
impl IMaterialModifier {
    #[method(name = "GetModifiedMaterial", args = 1)]
    pub fn get_modified_material(
        self,
        base_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;
}
