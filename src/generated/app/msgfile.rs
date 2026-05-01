
use crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase;
use crate::nintendo::message_studio::lib::binlibmsfilebase::IBinLibmsFileBase;
use crate::nintendo::message_studio::lib::binmsgfile::BinMsgFile;
use crate::nintendo::message_studio::lib::binmsgfile::IBinMsgFile;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/msgfile/MsgFile.md")))]
#[::unity2::class(namespace = "App", name = "MsgFile")]
#[parent(crate::nintendo::message_studio::lib::binmsgfile::BinMsgFile)]
pub struct MsgFile {
    #[rename(name = "m_reference")]
    pub m_reference: i32,
}

#[cfg(feature = "app-msgfile")]
#[::unity2::methods]
impl MsgFile {
    #[method(name = "GetReference", args = 0)]
    pub fn get_reference(self) -> i32;

    #[method(name = "SetReference", args = 1)]
    pub fn set_reference(self, refernce: i32) -> ();

    #[method(name = "IncReference", args = 0)]
    pub fn inc_reference(self) -> ();

    #[method(name = "DecReference", args = 0)]
    pub fn dec_reference(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-msgfile")]
impl MsgFile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MsgFile),
                ::core::stringify!(new),
            )
        });
        <Self as IMsgFileMethods>::ctor(this);
        this
    }
}
