
use crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase;
use crate::nintendo::message_studio::lib::binlibmsfilebase::IBinLibmsFileBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/binprjfile/BinPrjFile.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "BinPrjFile")]
#[parent(crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase)]
pub struct BinPrjFile {}

#[cfg(feature = "nintendo-message_studio-lib-binprjfile")]
#[::unity2::methods]
impl BinPrjFile {
    #[method(name = "InitObject", args = 1)]
    pub fn init_object(self, resource_ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "CloseObject", args = 1)]
    pub fn close_object(self, object_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "SearchProjectBlock", args = 1)]
    pub fn search_project_block(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetColorIndex", args = 1)]
    pub fn get_color_index(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetColor", args = 1)]
    pub fn get_color(self, index: i32) -> crate::nintendo::message_studio::lib::lmscolor::LMSColor;

    #[method(name = "GetColor", args = 1)]
    pub fn get_color_2(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::nintendo::message_studio::lib::lmscolor::LMSColor;

    #[method(name = "GetColorNum", args = 0)]
    pub fn get_color_num(self) -> i32;

    #[method(name = "GetContentsNum", args = 0)]
    pub fn get_contents_num(self) -> i32;

    #[method(name = "GetContentPath", args = 1)]
    pub fn get_content_path(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetAttrInfoIndex", args = 1)]
    pub fn get_attr_info_index(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetAttrType", args = 1)]
    pub fn get_attr_type(
        self,
        index: i32,
    ) -> crate::nintendo::message_studio::lib::libmstype::LibmsType;

    #[method(name = "GetAttrType", args = 1)]
    pub fn get_attr_type_2(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::nintendo::message_studio::lib::libmstype::LibmsType;

    #[method(name = "GetAttrOffset", args = 1)]
    pub fn get_attr_offset(self, index: i32) -> i32;

    #[method(name = "GetAttrOffset", args = 1)]
    pub fn get_attr_offset_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetAttrListItemName", args = 2)]
    pub fn get_attr_list_item_name(
        self,
        attr_index: i32,
        item_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetAttrListItemName", args = 2)]
    pub fn get_attr_list_item_name_2(
        self,
        attr_name: ::unity2::Il2CppString,
        item_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetAttrNum", args = 0)]
    pub fn get_attr_num(self) -> i32;

    #[method(name = "GetAttrListItemNum", args = 1)]
    pub fn get_attr_list_item_num(self, attr_index: i32) -> i32;

    #[method(name = "GetTagGroupName", args = 1)]
    pub fn get_tag_group_name(self, group_id: u16) -> ::unity2::Il2CppString;

    #[method(name = "GetTagName", args = 2)]
    pub fn get_tag_name(self, group_id: u16, tag_id: u16) -> ::unity2::Il2CppString;

    #[method(name = "GetTagParamName", args = 3)]
    pub fn get_tag_param_name(
        self,
        group_id: u16,
        tag_id: u16,
        param_id: u16,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTagParamType", args = 3)]
    pub fn get_tag_param_type(
        self,
        group_id: u16,
        tag_id: u16,
        param_id: u16,
    ) -> crate::nintendo::message_studio::lib::libmstype::LibmsType;

    #[method(name = "GetTagListItemName", args = 4)]
    pub fn get_tag_list_item_name(
        self,
        group_id: u16,
        tag_id: u16,
        param_id: u16,
        item_index: u16,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTagGroupNum", args = 0)]
    pub fn get_tag_group_num(self) -> i32;

    #[method(name = "GetTagNum", args = 1)]
    pub fn get_tag_num(self, group_id: u16) -> i32;

    #[method(name = "GetTagParamNum", args = 2)]
    pub fn get_tag_param_num(self, group_id: u16, tag_id: u16) -> i32;

    #[method(name = "GetTagListItemNum", args = 3)]
    pub fn get_tag_list_item_num(self, group_id: u16, tag_id: u16, param_id: u16) -> i32;

    #[method(name = "GetStyleIndex", args = 1)]
    pub fn get_style_index(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetRegionWidth", args = 1)]
    pub fn get_region_width(self, index: i32) -> i32;

    #[method(name = "GetRegionWidth", args = 1)]
    pub fn get_region_width_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetLineNum", args = 1)]
    pub fn get_line_num(self, index: i32) -> i32;

    #[method(name = "GetLineNum", args = 1)]
    pub fn get_line_num_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetFontIndex", args = 1)]
    pub fn get_font_index(self, index: i32) -> i32;

    #[method(name = "GetFontIndex", args = 1)]
    pub fn get_font_index_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetBaseColorIndex", args = 1)]
    pub fn get_base_color_index(self, index: i32) -> i32;

    #[method(name = "GetBaseColorIndex", args = 1)]
    pub fn get_base_color_index_2(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetStyleNum", args = 0)]
    pub fn get_style_num(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-binprjfile")]
impl BinPrjFile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BinPrjFile),
                ::core::stringify!(new),
            )
        });
        <Self as IBinPrjFileMethods>::ctor(this);
        this
    }
}
