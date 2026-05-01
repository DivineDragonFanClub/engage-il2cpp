
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/pagebreaktaginfo/PageBreakTagInfo.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "PageBreakTagInfo")]
#[parent(crate::system::object::Object)]
pub struct PageBreakTagInfo {}

#[cfg(feature = "nintendo-message_studio-lib-pagebreaktaginfo")]
#[::unity2::methods]
impl PageBreakTagInfo {
    #[method(name = "get_Tag", args = 0)]
    pub fn get_tag(self) -> u16;

    #[method(name = "get_TagGroup", args = 0)]
    pub fn get_tag_group(self) -> u16;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-pagebreaktaginfo")]
impl PageBreakTagInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PageBreakTagInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IPageBreakTagInfoMethods>::ctor(this);
        this
    }
}
