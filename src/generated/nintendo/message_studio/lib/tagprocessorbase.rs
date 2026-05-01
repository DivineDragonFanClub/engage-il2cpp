
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/tagprocessorbase/TagProcessorBase.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "TagProcessorBase")]
#[parent(crate::system::object::Object)]
pub struct TagProcessorBase {
    #[static_field]
    #[rename(name = "ShiftIn")]
    pub shift_in: u16,
    #[static_field]
    #[rename(name = "ShiftOut")]
    pub shift_out: u16,
}

#[cfg(feature = "nintendo-message_studio-lib-tagprocessorbase")]
#[::unity2::methods]
impl TagProcessorBase {
    #[method(name = "Process", args = 1)]
    pub fn process(self, p: ::unity2::IntPtr) -> ();

    #[method(name = "ProcessTag", args = 3)]
    pub fn process_tag(self, group: u16, tag: u16, param: ::unity2::Array<u8>) -> ();

    #[method(name = "ProcessSystemTag", args = 2)]
    pub fn process_system_tag(self, tag: u16, param: ::unity2::Array<u8>) -> ();

    #[method(name = "ProcessChar", args = 1)]
    pub fn process_char(self, c: u16) -> ();

    #[method(name = "ProcessRubyTag", args = 1)]
    pub fn process_ruby_tag(
        self,
        ruby_tag_info: crate::nintendo::message_studio::lib::rubytaginfo::RubyTagInfo,
    ) -> ();

    #[method(name = "ProcessFontTag", args = 1)]
    pub fn process_font_tag(
        self,
        font_tag_info: crate::nintendo::message_studio::lib::fonttaginfo::FontTagInfo,
    ) -> ();

    #[method(name = "ProcessSizeTag", args = 1)]
    pub fn process_size_tag(
        self,
        size_tag_info: crate::nintendo::message_studio::lib::sizetaginfo::SizeTagInfo,
    ) -> ();

    #[method(name = "ProcessColorTag", args = 1)]
    pub fn process_color_tag(
        self,
        color_tag_info: crate::nintendo::message_studio::lib::colortaginfo::ColorTagInfo,
    ) -> ();

    #[method(name = "ProcessPageBreakTag", args = 1)]
    pub fn process_page_break_tag(
        self,
        page_break_tag_info : crate :: nintendo :: message_studio :: lib :: pagebreaktaginfo :: PageBreakTagInfo,
    ) -> ();

    #[method(name = "ProcessCustomTag", args = 1)]
    pub fn process_custom_tag(
        self,
        tag_info: crate::nintendo::message_studio::lib::customtaginfo::CustomTagInfo,
    ) -> ();

    #[method(name = "ProcessEnd", args = 0)]
    pub fn process_end(self) -> ();

    #[method(name = "ReadChar", args = 2)]
    pub fn read_char(self, p: ::unity2::IntPtr, offset: i32) -> u16;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-tagprocessorbase")]
impl TagProcessorBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TagProcessorBase),
                ::core::stringify!(new),
            )
        });
        <Self as ITagProcessorBaseMethods>::ctor(this);
        this
    }
}
