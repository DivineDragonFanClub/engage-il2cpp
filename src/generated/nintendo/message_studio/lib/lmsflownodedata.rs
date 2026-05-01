
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/lmsflownodedata/LMSFlowNodeData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct LMSFlowNodeData {
    pub node_type: crate::nintendo::message_studio::lib::lmsflownodetype::LMSFlowNodeType,
    pub param_type: crate::nintendo::message_studio::lib::lmsflowparamtype::LMSFlowParamType,
    pub reserved: u16,
    pub param_value: u32,
    pub rawdata0: u16,
    pub rawdata1: u16,
    pub rawdata2: u16,
    pub rawdata3: u16,
    pub entry: crate::nintendo::message_studio::lib::lmsflowentry::LMSFlowEntry,
    pub message: crate::nintendo::message_studio::lib::lmsflowmessage::LMSFlowMessage,
    pub branch: crate::nintendo::message_studio::lib::lmsflowbranch::LMSFlowBranch,
    pub event: crate::nintendo::message_studio::lib::lmsflowevent::LMSFlowEvent,
    pub jump: crate::nintendo::message_studio::lib::lmsflowjump::LMSFlowJump,
}

impl ::unity2::ClassIdentity for LMSFlowNodeData {
    const NAMESPACE: &'static str = "Nintendo.MessageStudio.Lib";

    const NAME: &'static str = "LMSFlowNodeData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LMSFlowNodeData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
