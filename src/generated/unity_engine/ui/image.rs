
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::graphic::Graphic;
use crate::unity_engine::ui::graphic::IGraphic;
use crate::unity_engine::ui::maskablegraphic::IMaskableGraphic;
use crate::unity_engine::ui::maskablegraphic::MaskableGraphic;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_Type {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_Type {
    pub fn simple() -> Self {
        Self { value: 0 }
    }

    pub fn sliced() -> Self {
        Self { value: 1 }
    }

    pub fn tiled() -> Self {
        Self { value: 2 }
    }

    pub fn filled() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Image")]
#[parent(crate::unity_engine::ui::maskablegraphic::MaskableGraphic)]
pub struct Image {
    #[static_field]
    #[rename(name = "s_ETC1DefaultUI")]
    pub s_etc1_default_ui: crate::unity_engine::material::Material,
    #[rename(name = "m_Sprite")]
    pub m_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_OverrideSprite")]
    pub m_override_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_Type")]
    pub m_type: crate::unity_engine::ui::image::Image_Type,
    #[rename(name = "m_PreserveAspect")]
    pub m_preserve_aspect: bool,
    #[rename(name = "m_FillCenter")]
    pub m_fill_center: bool,
    #[rename(name = "m_FillMethod")]
    pub m_fill_method: crate::unity_engine::ui::image::Image_FillMethod,
    #[rename(name = "m_FillAmount")]
    pub m_fill_amount: f32,
    #[rename(name = "m_FillClockwise")]
    pub m_fill_clockwise: bool,
    #[rename(name = "m_FillOrigin")]
    pub m_fill_origin: i32,
    #[rename(name = "m_AlphaHitTestMinimumThreshold")]
    pub m_alpha_hit_test_minimum_threshold: f32,
    #[rename(name = "m_Tracked")]
    pub m_tracked: bool,
    #[rename(name = "m_UseSpriteMesh")]
    pub m_use_sprite_mesh: bool,
    #[rename(name = "m_PixelsPerUnitMultiplier")]
    pub m_pixels_per_unit_multiplier: f32,
    #[rename(name = "m_CachedReferencePixelsPerUnit")]
    pub m_cached_reference_pixels_per_unit: f32,
    #[static_field]
    #[rename(name = "s_VertScratch")]
    pub s_vert_scratch: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    #[static_field]
    #[rename(name = "s_UVScratch")]
    pub s_uv_scratch: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    #[static_field]
    #[rename(name = "s_Xy")]
    pub s_xy: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "s_Uv")]
    pub s_uv: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "m_TrackedTexturelessImages")]
    pub m_tracked_textureless_images:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::ui::image::Image>,
    #[static_field]
    #[rename(name = "s_Initialized")]
    pub s_initialized: bool,
}

#[cfg(feature = "unity_engine-ui-image")]
#[::unity2::methods]
impl Image {
    #[method(name = "get_sprite", args = 0)]
    pub fn get_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_sprite", args = 1)]
    pub fn set_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "DisableSpriteOptimizations", args = 0)]
    pub fn disable_sprite_optimizations(self) -> ();

    #[method(name = "get_overrideSprite", args = 0)]
    pub fn get_override_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "set_overrideSprite", args = 1)]
    pub fn set_override_sprite(self, value: crate::unity_engine::sprite::Sprite) -> ();

    #[method(name = "get_activeSprite", args = 0)]
    pub fn get_active_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "get_type", args = 0)]
    pub fn get_type(self) -> crate::unity_engine::ui::image::Image_Type;

    #[method(name = "set_type", args = 1)]
    pub fn set_type(self, value: crate::unity_engine::ui::image::Image_Type) -> ();

    #[method(name = "get_preserveAspect", args = 0)]
    pub fn get_preserve_aspect(self) -> bool;

    #[method(name = "set_preserveAspect", args = 1)]
    pub fn set_preserve_aspect(self, value: bool) -> ();

    #[method(name = "get_fillCenter", args = 0)]
    pub fn get_fill_center(self) -> bool;

    #[method(name = "set_fillCenter", args = 1)]
    pub fn set_fill_center(self, value: bool) -> ();

    #[method(name = "get_fillMethod", args = 0)]
    pub fn get_fill_method(self) -> crate::unity_engine::ui::image::Image_FillMethod;

    #[method(name = "set_fillMethod", args = 1)]
    pub fn set_fill_method(self, value: crate::unity_engine::ui::image::Image_FillMethod) -> ();

    #[method(name = "get_fillAmount", args = 0)]
    pub fn get_fill_amount(self) -> f32;

    #[method(name = "set_fillAmount", args = 1)]
    pub fn set_fill_amount(self, value: f32) -> ();

    #[method(name = "get_fillClockwise", args = 0)]
    pub fn get_fill_clockwise(self) -> bool;

    #[method(name = "set_fillClockwise", args = 1)]
    pub fn set_fill_clockwise(self, value: bool) -> ();

    #[method(name = "get_fillOrigin", args = 0)]
    pub fn get_fill_origin(self) -> i32;

    #[method(name = "set_fillOrigin", args = 1)]
    pub fn set_fill_origin(self, value: i32) -> ();

    #[method(name = "get_eventAlphaThreshold", args = 0)]
    pub fn get_event_alpha_threshold(self) -> f32;

    #[method(name = "set_eventAlphaThreshold", args = 1)]
    pub fn set_event_alpha_threshold(self, value: f32) -> ();

    #[method(name = "get_alphaHitTestMinimumThreshold", args = 0)]
    pub fn get_alpha_hit_test_minimum_threshold(self) -> f32;

    #[method(name = "set_alphaHitTestMinimumThreshold", args = 1)]
    pub fn set_alpha_hit_test_minimum_threshold(self, value: f32) -> ();

    #[method(name = "get_useSpriteMesh", args = 0)]
    pub fn get_use_sprite_mesh(self) -> bool;

    #[method(name = "set_useSpriteMesh", args = 1)]
    pub fn set_use_sprite_mesh(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_defaultETC1GraphicMaterial", args = 0)]
    pub fn get_default_etc1_graphic_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "get_hasBorder", args = 0)]
    pub fn get_has_border(self) -> bool;

    #[method(name = "get_pixelsPerUnitMultiplier", args = 0)]
    pub fn get_pixels_per_unit_multiplier(self) -> f32;

    #[method(name = "set_pixelsPerUnitMultiplier", args = 1)]
    pub fn set_pixels_per_unit_multiplier(self, value: f32) -> ();

    #[method(name = "get_pixelsPerUnit", args = 0)]
    pub fn get_pixels_per_unit(self) -> f32;

    #[method(name = "get_multipliedPixelsPerUnit", args = 0)]
    pub fn get_multiplied_pixels_per_unit(self) -> f32;

    #[method(name = "get_material", args = 0)]
    pub fn get_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_material", args = 1)]
    pub fn set_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "OnBeforeSerialize", args = 0)]
    pub fn on_before_serialize(self) -> ();

    #[method(name = "OnAfterDeserialize", args = 0)]
    pub fn on_after_deserialize(self) -> ();

    #[method(name = "PreserveSpriteAspectRatio", args = 2)]
    pub fn preserve_sprite_aspect_ratio(
        self,
        rect: crate::unity_engine::rect::Rect,
        sprite_size: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "GetDrawingDimensions", args = 1)]
    pub fn get_drawing_dimensions(
        self,
        should_preserve_aspect: bool,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "SetNativeSize", args = 0)]
    pub fn set_native_size(self) -> ();

    #[method(name = "OnPopulateMesh", args = 1)]
    pub fn on_populate_mesh(
        self,
        to_fill: crate::unity_engine::ui::vertexhelper::VertexHelper,
    ) -> ();

    #[method(name = "TrackSprite", args = 0)]
    pub fn track_sprite(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "UpdateMaterial", args = 0)]
    pub fn update_material(self) -> ();

    #[method(name = "OnCanvasHierarchyChanged", args = 0)]
    pub fn on_canvas_hierarchy_changed(self) -> ();

    #[method(name = "GenerateSimpleSprite", args = 2)]
    pub fn generate_simple_sprite(
        self,
        vh: crate::unity_engine::ui::vertexhelper::VertexHelper,
        l_preserve_aspect: bool,
    ) -> ();

    #[method(name = "GenerateSprite", args = 2)]
    pub fn generate_sprite(
        self,
        vh: crate::unity_engine::ui::vertexhelper::VertexHelper,
        l_preserve_aspect: bool,
    ) -> ();

    #[method(name = "GenerateSlicedSprite", args = 1)]
    pub fn generate_sliced_sprite(
        self,
        to_fill: crate::unity_engine::ui::vertexhelper::VertexHelper,
    ) -> ();

    #[method(name = "GenerateTiledSprite", args = 1)]
    pub fn generate_tiled_sprite(
        self,
        to_fill: crate::unity_engine::ui::vertexhelper::VertexHelper,
    ) -> ();

    #[method(name = "AddQuad", args = 4)]
    pub fn add_quad(
        vertex_helper: crate::unity_engine::ui::vertexhelper::VertexHelper,
        quad_positions: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        color: crate::unity_engine::color32::Color32,
        quad_u_vs: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "AddQuad", args = 6)]
    pub fn add_quad_2(
        vertex_helper: crate::unity_engine::ui::vertexhelper::VertexHelper,
        pos_min: crate::unity_engine::vector2::Vector2,
        pos_max: crate::unity_engine::vector2::Vector2,
        color: crate::unity_engine::color32::Color32,
        uv_min: crate::unity_engine::vector2::Vector2,
        uv_max: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "GetAdjustedBorders", args = 2)]
    pub fn get_adjusted_borders(
        self,
        border: crate::unity_engine::vector4::Vector4,
        adjusted_rect: crate::unity_engine::rect::Rect,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GenerateFilledSprite", args = 2)]
    pub fn generate_filled_sprite(
        self,
        to_fill: crate::unity_engine::ui::vertexhelper::VertexHelper,
        preserve_aspect: bool,
    ) -> ();

    #[method(name = "RadialCut", args = 5)]
    pub fn radial_cut(
        xy: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        uv: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        fill: f32,
        invert: bool,
        corner: i32,
    ) -> bool;

    #[method(name = "RadialCut", args = 5)]
    pub fn radial_cut_2(
        xy: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        cos: f32,
        sin: f32,
        invert: bool,
        corner: i32,
    ) -> ();

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "get_minWidth", args = 0)]
    pub fn get_min_width(self) -> f32;

    #[method(name = "get_preferredWidth", args = 0)]
    pub fn get_preferred_width(self) -> f32;

    #[method(name = "get_flexibleWidth", args = 0)]
    pub fn get_flexible_width(self) -> f32;

    #[method(name = "get_minHeight", args = 0)]
    pub fn get_min_height(self) -> f32;

    #[method(name = "get_preferredHeight", args = 0)]
    pub fn get_preferred_height(self) -> f32;

    #[method(name = "get_flexibleHeight", args = 0)]
    pub fn get_flexible_height(self) -> f32;

    #[method(name = "get_layoutPriority", args = 0)]
    pub fn get_layout_priority(self) -> i32;

    #[method(name = "IsRaycastLocationValid", args = 2)]
    pub fn is_raycast_location_valid(
        self,
        screen_point: crate::unity_engine::vector2::Vector2,
        event_camera: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = "MapCoordinate", args = 2)]
    pub fn map_coordinate(
        self,
        local: crate::unity_engine::vector2::Vector2,
        rect: crate::unity_engine::rect::Rect,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "RebuildImage", args = 1)]
    pub fn rebuild_image(sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas) -> ();

    #[method(name = "TrackImage", args = 1)]
    pub fn track_image(g: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "UnTrackImage", args = 1)]
    pub fn un_track_image(g: crate::unity_engine::ui::image::Image) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-ui-image")]
impl Image {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Image),
                ::core::stringify!(new),
            )
        });
        <Self as IImageMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_Origin360.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_Origin360 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_Origin360 {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.Origin360";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_Origin360 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_Origin360 {
    pub fn bottom() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }

    pub fn top() -> Self {
        Self { value: 2 }
    }

    pub fn left() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_OriginHorizontal.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_OriginHorizontal {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_OriginHorizontal {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.OriginHorizontal";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_OriginHorizontal {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_OriginHorizontal {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_Origin180.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_Origin180 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_Origin180 {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.Origin180";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_Origin180 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_Origin180 {
    pub fn bottom() -> Self {
        Self { value: 0 }
    }

    pub fn left() -> Self {
        Self { value: 1 }
    }

    pub fn top() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_Origin90.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_Origin90 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_Origin90 {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.Origin90";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_Origin90 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_Origin90 {
    pub fn bottom_left() -> Self {
        Self { value: 0 }
    }

    pub fn top_left() -> Self {
        Self { value: 1 }
    }

    pub fn top_right() -> Self {
        Self { value: 2 }
    }

    pub fn bottom_right() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_OriginVertical.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_OriginVertical {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_OriginVertical {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.OriginVertical";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_OriginVertical {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_OriginVertical {
    pub fn bottom() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/image/Image_FillMethod.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Image_FillMethod {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Image_FillMethod {
    const NAMESPACE: &'static str = "UnityEngine.UI";

    const NAME: &'static str = "Image.FillMethod";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Image_FillMethod {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Image_FillMethod {
    pub fn horizontal() -> Self {
        Self { value: 0 }
    }

    pub fn vertical() -> Self {
        Self { value: 1 }
    }

    pub fn radial90() -> Self {
        Self { value: 2 }
    }

    pub fn radial180() -> Self {
        Self { value: 3 }
    }

    pub fn radial360() -> Self {
        Self { value: 4 }
    }
}
