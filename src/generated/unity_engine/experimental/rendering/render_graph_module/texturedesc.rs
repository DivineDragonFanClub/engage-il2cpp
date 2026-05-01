
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/texturedesc/TextureDesc.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TextureDesc {
    pub size_mode: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturesizemode :: TextureSizeMode,
    pub width: i32,
    pub height: i32,
    pub slices: i32,
    pub scale: crate :: unity_engine :: vector2 :: Vector2,
    pub func: crate :: unity_engine :: rendering :: scalefunc :: ScaleFunc,
    pub depth_buffer_bits: crate :: unity_engine :: rendering :: depthbits :: DepthBits,
    pub color_format: crate :: unity_engine :: experimental :: rendering :: graphicsformat :: GraphicsFormat,
    pub filter_mode: crate :: unity_engine :: filtermode :: FilterMode,
    pub wrap_mode: crate :: unity_engine :: texturewrapmode :: TextureWrapMode,
    pub dimension: crate :: unity_engine :: rendering :: texturedimension :: TextureDimension,
    pub enable_random_write: bool,
    pub use_mip_map: bool,
    pub auto_generate_mips: bool,
    pub is_shadow_map: bool,
    pub aniso_level: i32,
    pub mip_map_bias: f32,
    pub enable_msaa: bool,
    pub msaa_samples: crate :: unity_engine :: rendering :: msaasamples :: MSAASamples,
    pub bind_texture_ms: bool,
    pub use_dynamic_scale: bool,
    pub memoryless: crate :: unity_engine :: rendertexturememoryless :: RenderTextureMemoryless,
    pub name: :: unity2 :: Il2CppString,
    pub fast_memory_desc: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: fastmemorydesc :: FastMemoryDesc,
    pub clear_buffer: bool,
    pub clear_color: crate :: unity_engine :: color :: Color,
}

impl ::unity2::ClassIdentity for TextureDesc {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "TextureDesc";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TextureDesc {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-texturedesc")]
#[::unity2::methods(value)]
impl TextureDesc {
    #[method(name = "InitDefaultValues", args = 2)]
    pub fn init_default_values(self, dynamic_resolution: bool, xr_ready: bool) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, width: i32, height: i32, dynamic_resolution: bool, xr_ready: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        scale: crate::unity_engine::vector2::Vector2,
        dynamic_resolution: bool,
        xr_ready: bool,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        func: crate::unity_engine::rendering::scalefunc::ScaleFunc,
        dynamic_resolution: bool,
        xr_ready: bool,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(
        self,
        input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
