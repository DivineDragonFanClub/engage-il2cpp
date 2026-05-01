
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/textgenerator/TextGenerator.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "TextGenerator")]
#[parent(crate::system::object::Object)]
pub struct TextGenerator {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_LastString")]
    pub m_last_string: ::unity2::Il2CppString,
    #[rename(name = "m_LastSettings")]
    pub m_last_settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    #[rename(name = "m_HasGenerated")]
    pub m_has_generated: bool,
    #[rename(name = "m_LastValid")]
    pub m_last_valid: crate::unity_engine::textgenerationerror::TextGenerationError,
    #[rename(name = "m_Verts")]
    pub m_verts: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::uivertex::UIVertex,
    >,
    #[rename(name = "m_Characters")]
    pub m_characters: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::uicharinfo::UICharInfo,
    >,
    #[rename(name = "m_Lines")]
    pub m_lines: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::uilineinfo::UILineInfo,
    >,
    #[rename(name = "m_CachedVerts")]
    pub m_cached_verts: bool,
    #[rename(name = "m_CachedCharacters")]
    pub m_cached_characters: bool,
    #[rename(name = "m_CachedLines")]
    pub m_cached_lines: bool,
}

#[cfg(feature = "unity_engine-textgenerator")]
#[::unity2::methods]
impl TextGenerator {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, initial_capacity: i32) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "System.IDisposable.Dispose", args = 0)]
    pub fn system_i_disposable_dispose(self) -> ();

    #[method(name = "get_characterCountVisible", args = 0)]
    pub fn get_character_count_visible(self) -> i32;

    #[method(name = "ValidatedSettings", args = 1)]
    pub fn validated_settings(
        self,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> crate::unity_engine::textgenerationsettings::TextGenerationSettings;

    #[method(name = "Invalidate", args = 0)]
    pub fn invalidate(self) -> ();

    #[method(name = "GetCharacters", args = 1)]
    pub fn get_characters(
        self,
        characters: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uicharinfo::UICharInfo,
        >,
    ) -> ();

    #[method(name = "GetLines", args = 1)]
    pub fn get_lines(
        self,
        lines: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uilineinfo::UILineInfo,
        >,
    ) -> ();

    #[method(name = "GetVertices", args = 1)]
    pub fn get_vertices(
        self,
        vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = "GetPreferredWidth", args = 2)]
    pub fn get_preferred_width(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> f32;

    #[method(name = "GetPreferredHeight", args = 2)]
    pub fn get_preferred_height(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> f32;

    #[method(name = "PopulateWithErrors", args = 3)]
    pub fn populate_with_errors(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
        context: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "Populate", args = 2)]
    pub fn populate(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> bool;

    #[method(name = "PopulateWithError", args = 2)]
    pub fn populate_with_error(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> crate::unity_engine::textgenerationerror::TextGenerationError;

    #[method(name = "PopulateAlways", args = 2)]
    pub fn populate_always(
        self,
        str: ::unity2::Il2CppString,
        settings: crate::unity_engine::textgenerationsettings::TextGenerationSettings,
    ) -> crate::unity_engine::textgenerationerror::TextGenerationError;

    #[method(name = "get_verts", args = 0)]
    pub fn get_verts(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::unity_engine::uivertex::UIVertex,
    >;

    #[method(name = "get_characters", args = 0)]
    pub fn get_characters_2(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::unity_engine::uicharinfo::UICharInfo,
    >;

    #[method(name = "get_lines", args = 0)]
    pub fn get_lines_2(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::unity_engine::uilineinfo::UILineInfo,
    >;

    #[method(name = "get_rectExtents", args = 0)]
    pub fn get_rect_extents(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "get_characterCount", args = 0)]
    pub fn get_character_count(self) -> i32;

    #[method(name = "get_lineCount", args = 0)]
    pub fn get_line_count(self) -> i32;

    #[method(name = "Internal_Create", args = 0)]
    pub fn internal_create() -> ::unity2::IntPtr;

    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Populate_Internal", args = 22)]
    pub fn populate_internal(
        self,
        str: ::unity2::Il2CppString,
        font: crate::unity_engine::font::Font,
        color: crate::unity_engine::color::Color,
        font_size: i32,
        scale_factor: f32,
        line_spacing: f32,
        style: crate::unity_engine::fontstyle::FontStyle,
        rich_text: bool,
        resize_text_for_best_fit: bool,
        resize_text_min_size: i32,
        resize_text_max_size: i32,
        vertical_over_flow: i32,
        horizontal_overflow: i32,
        update_bounds: bool,
        anchor: crate::unity_engine::textanchor::TextAnchor,
        extents_x: f32,
        extents_y: f32,
        pivot_x: f32,
        pivot_y: f32,
        generate_out_of_bounds: bool,
        align_by_geometry: bool,
        error: u32,
    ) -> bool;

    #[method(name = "Populate_Internal", args = 20)]
    pub fn populate_internal_2(
        self,
        str: ::unity2::Il2CppString,
        font: crate::unity_engine::font::Font,
        color: crate::unity_engine::color::Color,
        font_size: i32,
        scale_factor: f32,
        line_spacing: f32,
        style: crate::unity_engine::fontstyle::FontStyle,
        rich_text: bool,
        resize_text_for_best_fit: bool,
        resize_text_min_size: i32,
        resize_text_max_size: i32,
        vertical_over_flow: crate::unity_engine::verticalwrapmode::VerticalWrapMode,
        horizontal_overflow: crate::unity_engine::horizontalwrapmode::HorizontalWrapMode,
        update_bounds: bool,
        anchor: crate::unity_engine::textanchor::TextAnchor,
        extents: crate::unity_engine::vector2::Vector2,
        pivot: crate::unity_engine::vector2::Vector2,
        generate_out_of_bounds: bool,
        align_by_geometry: bool,
        error: crate::unity_engine::textgenerationerror::TextGenerationError,
    ) -> bool;

    #[method(name = "GetVerticesInternal", args = 1)]
    pub fn get_vertices_internal(self, vertices: crate::system::object::Object) -> ();

    #[method(name = "GetCharactersInternal", args = 1)]
    pub fn get_characters_internal(self, characters: crate::system::object::Object) -> ();

    #[method(name = "GetLinesInternal", args = 1)]
    pub fn get_lines_internal(self, lines: crate::system::object::Object) -> ();

    #[method(name = "get_rectExtents_Injected", args = 1)]
    pub fn get_rect_extents_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "Populate_Internal_Injected", args = 22)]
    pub fn populate_internal_injected(
        self,
        str: ::unity2::Il2CppString,
        font: crate::unity_engine::font::Font,
        color: crate::unity_engine::color::Color,
        font_size: i32,
        scale_factor: f32,
        line_spacing: f32,
        style: crate::unity_engine::fontstyle::FontStyle,
        rich_text: bool,
        resize_text_for_best_fit: bool,
        resize_text_min_size: i32,
        resize_text_max_size: i32,
        vertical_over_flow: i32,
        horizontal_overflow: i32,
        update_bounds: bool,
        anchor: crate::unity_engine::textanchor::TextAnchor,
        extents_x: f32,
        extents_y: f32,
        pivot_x: f32,
        pivot_y: f32,
        generate_out_of_bounds: bool,
        align_by_geometry: bool,
        error: u32,
    ) -> bool;
}

#[cfg(feature = "unity_engine-textgenerator")]
impl TextGenerator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextGenerator),
                ::core::stringify!(new),
            )
        });
        <Self as ITextGeneratorMethods>::ctor(this);
        this
    }

    pub fn new_2(initial_capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextGenerator),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITextGeneratorMethods>::ctor_2(this, initial_capacity);
        this
    }
}
