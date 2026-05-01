
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/imeshmodifier/IMeshModifier.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "IMeshModifier")]
pub struct IMeshModifier {}

#[cfg(feature = "unity_engine-ui-imeshmodifier")]
#[::unity2::methods]
impl IMeshModifier {
    #[method(name = "ModifyMesh", args = 1)]
    pub fn modify_mesh(self, mesh: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "ModifyMesh", args = 1)]
    pub fn modify_mesh_2(self, verts: crate::unity_engine::ui::vertexhelper::VertexHelper) -> ();
}
