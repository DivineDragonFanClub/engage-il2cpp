
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/faceinfo_legacy/FaceInfo_Legacy.md")))]
#[::unity2::class(namespace = "TMPro", name = "FaceInfo_Legacy")]
#[parent(crate::system::object::Object)]
pub struct FaceInfo_Legacy {
    #[rename(name = "Name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "PointSize")]
    pub point_size: f32,
    #[rename(name = "Scale")]
    pub scale: f32,
    #[rename(name = "CharacterCount")]
    pub character_count: i32,
    #[rename(name = "LineHeight")]
    pub line_height: f32,
    #[rename(name = "Baseline")]
    pub baseline: f32,
    #[rename(name = "Ascender")]
    pub ascender: f32,
    #[rename(name = "CapHeight")]
    pub cap_height: f32,
    #[rename(name = "Descender")]
    pub descender: f32,
    #[rename(name = "CenterLine")]
    pub center_line: f32,
    #[rename(name = "SuperscriptOffset")]
    pub superscript_offset: f32,
    #[rename(name = "SubscriptOffset")]
    pub subscript_offset: f32,
    #[rename(name = "SubSize")]
    pub sub_size: f32,
    #[rename(name = "Underline")]
    pub underline: f32,
    #[rename(name = "UnderlineThickness")]
    pub underline_thickness: f32,
    #[rename(name = "strikethrough")]
    pub strikethrough: f32,
    #[rename(name = "strikethroughThickness")]
    pub strikethrough_thickness: f32,
    #[rename(name = "TabWidth")]
    pub tab_width: f32,
    #[rename(name = "Padding")]
    pub padding: f32,
    #[rename(name = "AtlasWidth")]
    pub atlas_width: f32,
    #[rename(name = "AtlasHeight")]
    pub atlas_height: f32,
}

#[cfg(feature = "tm_pro-faceinfo_legacy")]
#[::unity2::methods]
impl FaceInfo_Legacy {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-faceinfo_legacy")]
impl FaceInfo_Legacy {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FaceInfo_Legacy),
                ::core::stringify!(new),
            )
        });
        <Self as IFaceInfo_LegacyMethods>::ctor(this);
        this
    }
}
