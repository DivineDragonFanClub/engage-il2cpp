
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::meshutils::IMeshUtils_Pooled_1;
use crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::meshutils::MeshUtils_Pooled_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/mesh_2/Mesh_2.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Mesh"
)]
# [parent (crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Pooled_1 < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: mesh_2 :: Mesh_2 >)]
pub struct Mesh_2 {
# [rename (name = "_vHead")] pub v_head : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex ,
# [rename (name = "_fHead")] pub f_head : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Face ,
# [rename (name = "_eHead")] pub e_head : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ,
# [rename (name = "_eHeadSym")] pub e_head_sym : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-mesh_2")]
#[::unity2::methods]
impl Mesh_2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "OnFree", args = 0)]
    pub fn on_free(self) -> ();

    #[method(name = "MakeEdge", args = 0)]
    pub fn make_edge (self ,) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ;

    #[method(name = "Splice", args = 2)]
    pub fn splice(
        self,
        e_org : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        e_dst : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete(
        self,
        e_del : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "AddEdgeVertex", args = 1)]
    pub fn add_edge_vertex (self , e_org : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ;

    #[method(name = "SplitEdge", args = 1)]
    pub fn split_edge (self , e_org : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ;

    #[method(name = "Connect", args = 2)]
    pub fn connect (self , e_org : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge , e_dst : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ;

    #[method(name = "ZapFace", args = 1)]
    pub fn zap_face(
        self,
        f_zap : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Face,
    ) -> ();

    #[method(name = "MergeConvexFaces", args = 1)]
    pub fn merge_convex_faces(self, max_verts_per_face: i32) -> ();

    #[method(name = "Check", args = 0)]
    pub fn check(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-mesh_2")]
impl Mesh_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Mesh_2),
                ::core::stringify!(new),
            )
        });
        <Self as IMesh_2Methods>::ctor(this);
        this
    }
}
