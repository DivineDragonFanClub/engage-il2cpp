
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/multiitemgainsequence/MultiItemGainSequence.md")))]
#[::unity2::class(namespace = "App", name = "MultiItemGainSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MultiItemGainSequence {
    #[rename(name = "m_Items")]
    pub m_items:
        crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-multiitemgainsequence")]
#[::unity2::methods]
impl MultiItemGainSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        items: crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    ) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        items: crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    ) -> ();
}

#[cfg(feature = "app-multiitemgainsequence")]
impl MultiItemGainSequence {
    pub fn new(
        items: crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MultiItemGainSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMultiItemGainSequenceMethods>::ctor(this, items);
        this
    }
}
