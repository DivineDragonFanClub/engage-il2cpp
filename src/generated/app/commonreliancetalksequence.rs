
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonreliancetalksequence/CommonRelianceTalkSequence.md")))]
#[::unity2::class(namespace = "App", name = "CommonRelianceTalkSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct CommonRelianceTalkSequence {
    #[rename(name = "m_MessFileName")]
    pub m_mess_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_IsLoadedMessFile")]
    pub m_is_loaded_mess_file: bool,
}

#[cfg(feature = "app-commonreliancetalksequence")]
#[::unity2::methods]
impl CommonRelianceTalkSequence {
    #[method(name = "CreateDescs", args = 0)]
    pub fn create_descs(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "CreateMessFileName", args = 1)]
    pub fn create_mess_file_name(self, is_reverse: bool) -> ::unity2::Il2CppString;

    #[method(name = "CreateMid", args = 0)]
    pub fn create_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();

    #[method(name = "get_MessFileName", args = 0)]
    pub fn get_mess_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_IsLoadedMessFile", args = 0)]
    pub fn get_is_loaded_mess_file(self) -> bool;
}

#[cfg(feature = "app-commonreliancetalksequence")]
impl CommonRelianceTalkSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRelianceTalkSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRelianceTalkSequenceMethods>::ctor(this);
        this
    }
}
