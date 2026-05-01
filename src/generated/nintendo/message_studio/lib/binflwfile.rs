
use crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase;
use crate::nintendo::message_studio::lib::binlibmsfilebase::IBinLibmsFileBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/binflwfile/BinFlwFile.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "BinFlwFile")]
#[parent(crate::nintendo::message_studio::lib::binlibmsfilebase::BinLibmsFileBase)]
pub struct BinFlwFile {}

#[cfg(feature = "nintendo-message_studio-lib-binflwfile")]
#[::unity2::methods]
impl BinFlwFile {
    #[method(name = "InitObject", args = 1)]
    pub fn init_object(self, resource_ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "CloseObject", args = 1)]
    pub fn close_object(self, object_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "GetNodeNum", args = 0)]
    pub fn get_node_num(self) -> i32;

    #[method(name = "GetEntryNodeIndex", args = 1)]
    pub fn get_entry_node_index(self, label: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetNodeData", args = 1)]
    pub fn get_node_data(
        self,
        index: i32,
    ) -> crate::nintendo::message_studio::lib::lmsflownodedata::LMSFlowNodeData;

    #[method(name = "GetCaseIndexesFromBranchNode", args = 1)]
    pub fn get_case_indexes_from_branch_node(self, index: i32) -> ::unity2::Array<u16>;

    #[method(name = "GetFlowParamText", args = 1)]
    pub fn get_flow_param_text(self, offset: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetIndexTest", args = 1)]
    pub fn get_index_test(self, index: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-binflwfile")]
impl BinFlwFile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BinFlwFile),
                ::core::stringify!(new),
            )
        });
        <Self as IBinFlwFileMethods>::ctor(this);
        this
    }
}
