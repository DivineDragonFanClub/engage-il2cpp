
use crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase;
use crate::nintendo::message_studio::lib::binlibmsfilebase::IBinLibmsFileBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/binmsgfile/BinMsgFile.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "BinMsgFile")]
#[parent(crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase)]
pub struct BinMsgFile {}

#[cfg(feature = "nintendo-message_studio-lib-binmsgfile")]
#[::unity2::methods]
impl BinMsgFile {
    #[method(name = "InitObject", args = 1)]
    pub fn init_object(self, resource_ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "CloseObject", args = 1)]
    pub fn close_object(self, object_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "SearchMessageBlock", args = 1)]
    pub fn search_message_block(self, block: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetBlockInfo", args = 1)]
    pub fn get_block_info(
        self,
        block: ::unity2::Il2CppString,
    ) -> crate::nintendo::message_studio::lib::blockinfo::BlockInfo;

    #[method(name = "GetAttributes", args = 1)]
    pub fn get_attributes(self, label: ::unity2::Il2CppString) -> ::unity2::Array<u8>;

    #[method(name = "GetAttributes", args = 1)]
    pub fn get_attributes_2(self, index: i32) -> ::unity2::Array<u8>;

    #[method(name = "GetInt8Attribute", args = 2)]
    pub fn get_int8_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> i8;

    #[method(name = "GetUInt8Attribute", args = 2)]
    pub fn get_u_int8_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> u8;

    #[method(name = "GetInt16Attribute", args = 2)]
    pub fn get_int16_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> i16;

    #[method(name = "GetUInt16Attribute", args = 2)]
    pub fn get_u_int16_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> u16;

    #[method(name = "GetInt32Attribute", args = 2)]
    pub fn get_int32_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> i32;

    #[method(name = "GetUInt32Attribute", args = 2)]
    pub fn get_u_int32_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> u32;

    #[method(name = "GetFloatAttribute", args = 2)]
    pub fn get_float_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> f32;

    #[method(name = "GetDoubleAttribute", args = 2)]
    pub fn get_double_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> f64;

    #[method(name = "GetStringAttribute", args = 2)]
    pub fn get_string_attribute(
        self,
        attr: ::unity2::Array<u8>,
        index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetListAttribute", args = 2)]
    pub fn get_list_attribute(self, attr: ::unity2::Array<u8>, index: i32) -> i32;

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style(self, label: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style_2(self, index: i32) -> i32;

    #[method(name = "GetLabel", args = 1)]
    pub fn get_label(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetTextNum", args = 0)]
    pub fn get_text_num(self) -> i32;

    #[method(name = "GetTextIndex", args = 1)]
    pub fn get_text_index(self, label: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetTextSize", args = 1)]
    pub fn get_text_size(self, index: i32) -> i32;

    #[method(name = "GetText", args = 1)]
    pub fn get_text(self, str_label: ::unity2::Il2CppString) -> ::unity2::IntPtr;

    #[method(name = "GetText", args = 1)]
    pub fn get_text_2(self, index: i32) -> ::unity2::IntPtr;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-binmsgfile")]
impl BinMsgFile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BinMsgFile),
                ::core::stringify!(new),
            )
        });
        <Self as IBinMsgFileMethods>::ctor(this);
        this
    }
}
