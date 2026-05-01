
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphdefaultresources/RenderGraphDefaultResources.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphDefaultResources"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphDefaultResources {
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
    #[rename(name = "m_BlackTexture2D")]
    pub m_black_texture2_d: crate::unity_engine::rendering::rthandle::RTHandle,
    #[rename(name = "m_WhiteTexture2D")]
    pub m_white_texture2_d: crate::unity_engine::rendering::rthandle::RTHandle,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphdefaultresources")]
#[::unity2::methods]
impl RenderGraphDefaultResources {
    #[method(name = "get_blackTexture", args = 0)]
    pub fn get_black_texture (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_blackTexture", args = 1)]
    pub fn set_black_texture(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_whiteTexture", args = 0)]
    pub fn get_white_texture (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_whiteTexture", args = 1)]
    pub fn set_white_texture(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_clearTextureXR", args = 0)]
    pub fn get_clear_texture_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_clearTextureXR", args = 1)]
    pub fn set_clear_texture_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_magentaTextureXR", args = 0)]
    pub fn get_magenta_texture_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_magentaTextureXR", args = 1)]
    pub fn set_magenta_texture_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_blackTextureXR", args = 0)]
    pub fn get_black_texture_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_blackTextureXR", args = 1)]
    pub fn set_black_texture_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_blackTextureArrayXR", args = 0)]
    pub fn get_black_texture_array_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_blackTextureArrayXR", args = 1)]
    pub fn set_black_texture_array_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_blackUIntTextureXR", args = 0)]
    pub fn get_black_u_int_texture_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_blackUIntTextureXR", args = 1)]
    pub fn set_black_u_int_texture_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_blackTexture3DXR", args = 0)]
    pub fn get_black_texture3_dxr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_blackTexture3DXR", args = 1)]
    pub fn set_black_texture3_dxr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_whiteTextureXR", args = 0)]
    pub fn get_white_texture_xr (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_whiteTextureXR", args = 1)]
    pub fn set_white_texture_xr(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "InitializeForRendering", args = 1)]
    pub fn initialize_for_rendering(
        self,
        render_graph : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphdefaultresources")]
impl RenderGraphDefaultResources {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphDefaultResources),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphDefaultResourcesMethods>::ctor(this);
        this
    }
}
