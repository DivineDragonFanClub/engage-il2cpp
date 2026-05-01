
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/tess/Tess_ActiveRegion.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Tess.ActiveRegion"
)]
#[parent(crate::system::object::Object)]
pub struct Tess_ActiveRegion {
# [rename (name = "_eUp")] pub e_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ,
# [rename (name = "_nodeUp")] pub node_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion > ,
# [rename (name = "_windingNumber")] pub winding_number : i32 ,
# [rename (name = "_inside")] pub inside : bool ,
# [rename (name = "_sentinel")] pub sentinel : bool ,
# [rename (name = "_dirty")] pub dirty : bool ,
# [rename (name = "_fixUpperEdge")] pub fix_upper_edge : bool ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-tess")]
#[::unity2::methods]
impl Tess_ActiveRegion {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-tess")]
impl Tess_ActiveRegion {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Tess_ActiveRegion),
                ::core::stringify!(new),
            )
        });
        <Self as ITess_ActiveRegionMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/tess/Tess.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Tess"
)]
#[parent(crate::system::object::Object)]
pub struct Tess {
# [rename (name = "_mesh")] pub mesh : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: mesh_2 :: Mesh_2 ,
# [rename (name = "_sUnit")] pub s_unit : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: vec3 :: Vec3 ,
# [rename (name = "_tUnit")] pub t_unit : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: vec3 :: Vec3 ,
# [rename (name = "_bminX")] pub bmin_x : f32 ,
# [rename (name = "_bminY")] pub bmin_y : f32 ,
# [rename (name = "_bmaxX")] pub bmax_x : f32 ,
# [rename (name = "_bmaxY")] pub bmax_y : f32 ,
# [rename (name = "_windingRule")] pub winding_rule : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: windingrule :: WindingRule ,
# [rename (name = "_dict")] pub dict : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1 < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion > ,
# [rename (name = "_pq")] pub pq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: priorityqueue_1 :: PriorityQueue_1 < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex > ,
# [rename (name = "_event")] pub event : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex ,
# [rename (name = "_combineCallback")] pub combine_callback : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: combinecallback :: CombineCallback ,
# [rename (name = "_vertices")] pub vertices : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourvertex :: ContourVertex > ,
# [rename (name = "_vertexCount")] pub vertex_count : i32 ,
# [rename (name = "_elements")] pub elements : :: unity2 :: Array < i32 > ,
# [rename (name = "_elementCount")] pub element_count : i32 ,
# [rename (name = "SUnitX")] pub s_unit_x : f32 ,
# [rename (name = "SUnitY")] pub s_unit_y : f32 ,
# [rename (name = "SentinelCoord")] pub sentinel_coord : f32 ,
# [rename (name = "NoEmptyPolygons")] pub no_empty_polygons : bool ,
# [rename (name = "UsePooling")] pub use_pooling : bool ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-tess")]
#[::unity2::methods]
impl Tess {
    #[method(name = "RegionBelow", args = 1)]
    pub fn region_below (self , reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion ;

    #[method(name = "RegionAbove", args = 1)]
    pub fn region_above (self , reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion ;

    #[method(name = "EdgeLeq", args = 2)]
    pub fn edge_leq(
        self,
        reg1 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
        reg2 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> bool;

    #[method(name = "DeleteRegion", args = 1)]
    pub fn delete_region(
        self,
        reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> ();

    #[method(name = "FixUpperEdge", args = 2)]
    pub fn fix_upper_edge(
        self,
        reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
        new_edge : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "TopLeftRegion", args = 1)]
    pub fn top_left_region (self , reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion ;

    #[method(name = "TopRightRegion", args = 1)]
    pub fn top_right_region (self , reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion ;

    #[method(name = "AddRegionBelow", args = 2)]
    pub fn add_region_below (self , reg_above : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion , e_new_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion ;

    #[method(name = "ComputeWinding", args = 1)]
    pub fn compute_winding(
        self,
        reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> ();

    #[method(name = "FinishRegion", args = 1)]
    pub fn finish_region(
        self,
        reg : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> ();

    #[method(name = "FinishLeftRegions", args = 2)]
    pub fn finish_left_regions (self , reg_first : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion , reg_last : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge ;

    #[method(name = "AddRightEdges", args = 5)]
    pub fn add_right_edges(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
        e_first : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        e_last : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        e_top_left : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        clean_up: bool,
    ) -> ();

    #[method(name = "SpliceMergeVertices", args = 2)]
    pub fn splice_merge_vertices(
        self,
        e1 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
        e2 : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "VertexWeights", args = 5)]
    pub fn vertex_weights(
        self,
        isect : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        org : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        dst : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        w0: f32,
        w1: f32,
    ) -> ();

    #[method(name = "GetIntersectData", args = 5)]
    pub fn get_intersect_data(
        self,
        isect : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        org_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        dst_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        org_lo : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
        dst_lo : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();

    #[method(name = "CheckForRightSplice", args = 1)]
    pub fn check_for_right_splice(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> bool;

    #[method(name = "CheckForLeftSplice", args = 1)]
    pub fn check_for_left_splice(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> bool;

    #[method(name = "CheckForIntersect", args = 1)]
    pub fn check_for_intersect(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> bool;

    #[method(name = "WalkDirtyRegions", args = 1)]
    pub fn walk_dirty_regions(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
    ) -> ();

    #[method(name = "ConnectRightVertex", args = 2)]
    pub fn connect_right_vertex(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
        e_bottom_left : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> ();

    #[method(name = "ConnectLeftDegenerate", args = 2)]
    pub fn connect_left_degenerate(
        self,
        reg_up : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: tess :: Tess_ActiveRegion,
        v_event : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();

    #[method(name = "ConnectLeftVertex", args = 1)]
    pub fn connect_left_vertex(
        self,
        v_event : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();

    #[method(name = "SweepEvent", args = 1)]
    pub fn sweep_event(
        self,
        v_event : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Vertex,
    ) -> ();

    #[method(name = "AddSentinel", args = 3)]
    pub fn add_sentinel(self, smin: f32, smax: f32, t: f32) -> ();

    #[method(name = "InitEdgeDict", args = 0)]
    pub fn init_edge_dict(self) -> ();

    #[method(name = "DoneEdgeDict", args = 0)]
    pub fn done_edge_dict(self) -> ();

    #[method(name = "RemoveDegenerateEdges", args = 0)]
    pub fn remove_degenerate_edges(self) -> ();

    #[method(name = "InitPriorityQ", args = 0)]
    pub fn init_priority_q(self) -> ();

    #[method(name = "DonePriorityQ", args = 0)]
    pub fn done_priority_q(self) -> ();

    #[method(name = "RemoveDegenerateFaces", args = 0)]
    pub fn remove_degenerate_faces(self) -> ();

    #[method(name = "ComputeInterior", args = 0)]
    pub fn compute_interior(self) -> ();

    #[method(name = "get_Normal", args = 0)]
    pub fn get_normal(
        self,
    ) -> crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3;

    #[method(name = "set_Normal", args = 1)]
    pub fn set_normal(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: vec3 :: Vec3,
    ) -> ();

    #[method(name = "get_Vertices", args = 0)]
    pub fn get_vertices (self ,) -> :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourvertex :: ContourVertex > ;

    #[method(name = "get_VertexCount", args = 0)]
    pub fn get_vertex_count(self) -> i32;

    #[method(name = "get_Elements", args = 0)]
    pub fn get_elements(self) -> ::unity2::Array<i32>;

    #[method(name = "get_ElementCount", args = 0)]
    pub fn get_element_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ComputeNormal", args = 1)]
    pub fn compute_normal(
        self,
        norm: crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::vec3::Vec3,
    ) -> ();

    #[method(name = "CheckOrientation", args = 0)]
    pub fn check_orientation(self) -> ();

    #[method(name = "ProjectPolygon", args = 0)]
    pub fn project_polygon(self) -> ();

    #[method(name = "TessellateMonoRegion", args = 1)]
    pub fn tessellate_mono_region(
        self,
        face : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Face,
    ) -> ();

    #[method(name = "TessellateInterior", args = 0)]
    pub fn tessellate_interior(self) -> ();

    #[method(name = "DiscardExterior", args = 0)]
    pub fn discard_exterior(self) -> ();

    #[method(name = "SetWindingNumber", args = 2)]
    pub fn set_winding_number(self, value: i32, keep_only_boundary: bool) -> ();

    #[method(name = "GetNeighbourFace", args = 1)]
    pub fn get_neighbour_face(
        self,
        edge : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: meshutils :: MeshUtils_Edge,
    ) -> i32;

    #[method(name = "OutputPolymesh", args = 2)]
    pub fn output_polymesh(
        self,
        element_type : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: elementtype :: ElementType,
        poly_size: i32,
    ) -> ();

    #[method(name = "OutputContours", args = 0)]
    pub fn output_contours(self) -> ();

    #[method(name = "SignedArea", args = 1)]
    pub fn signed_area(
        self,
        vertices : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourvertex :: ContourVertex >,
    ) -> f32;

    #[method(name = "AddContour", args = 1)]
    pub fn add_contour(
        self,
        vertices : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourvertex :: ContourVertex >,
    ) -> ();

    #[method(name = "AddContour", args = 2)]
    pub fn add_contour_2(
        self,
        vertices : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourvertex :: ContourVertex >,
        force_orientation : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: contourorientation :: ContourOrientation,
    ) -> ();

    #[method(name = "Tessellate", args = 3)]
    pub fn tessellate(
        self,
        winding_rule : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: windingrule :: WindingRule,
        element_type : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: elementtype :: ElementType,
        poly_size: i32,
    ) -> ();

    #[method(name = "Tessellate", args = 4)]
    pub fn tessellate_2(
        self,
        winding_rule : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: windingrule :: WindingRule,
        element_type : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: elementtype :: ElementType,
        poly_size: i32,
        combine_callback : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: combinecallback :: CombineCallback,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-tess")]
impl Tess {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Tess),
                ::core::stringify!(new),
            )
        });
        <Self as ITessMethods>::ctor(this);
        this
    }
}
