
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/geom/Geom.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Geom"
)]
#[parent(crate::system::object::Object)]
pub struct Geom {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-geom")]
#[::unity2::methods]
impl Geom {
    #[method(name = "IsWindingInside", args = 2)]
    pub fn is_winding_inside(
        rule : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: windingrule :: WindingRule,
        n: i32,
    ) -> bool;

    #[method(name = "VertCCW", args = 3)]
    pub fn vert_ccw(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> bool;

    #[method(name = "VertEq", args = 2)]
    pub fn vert_eq(
        lhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        rhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> bool;

    #[method(name = "VertLeq", args = 2)]
    pub fn vert_leq(
        lhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        rhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> bool;

    #[method(name = "EdgeEval", args = 3)]
    pub fn edge_eval(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> f32;

    #[method(name = "EdgeSign", args = 3)]
    pub fn edge_sign(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> f32;

    #[method(name = "TransLeq", args = 2)]
    pub fn trans_leq(
        lhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        rhs : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> bool;

    #[method(name = "TransEval", args = 3)]
    pub fn trans_eval(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> f32;

    #[method(name = "TransSign", args = 3)]
    pub fn trans_sign(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> f32;

    #[method(name = "EdgeGoesLeft", args = 1)]
    pub fn edge_goes_left(
        e : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> bool;

    #[method(name = "EdgeGoesRight", args = 1)]
    pub fn edge_goes_right(
        e : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> bool;

    #[method(name = "VertL1dist", args = 2)]
    pub fn vert_l1dist(
        u : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> f32;

    #[method(name = "AddWinding", args = 2)]
    pub fn add_winding(
        e_dst : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        e_src : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "Interpolate", args = 4)]
    pub fn interpolate(a: f32, x: f32, b: f32, y: f32) -> f32;

    #[method(name = "Swap", args = 2)]
    pub fn swap(
        a : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        b : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();

    #[method(name = "EdgeIntersect", args = 5)]
    pub fn edge_intersect(
        o1 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        d1 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        o2 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        d2 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        v : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();
}
