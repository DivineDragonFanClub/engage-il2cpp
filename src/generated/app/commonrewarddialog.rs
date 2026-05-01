
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewarddialog/CommonRewardDialog.md")))]
#[::unity2::class(namespace = "App", name = "CommonRewardDialog")]
#[parent(crate::system::object::Object)]
pub struct CommonRewardDialog {}

#[cfg(feature = "app-commonrewarddialog")]
#[::unity2::methods]
impl CommonRewardDialog {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        title_text: ::unity2::Il2CppString,
        item_param_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::itemdata::ItemData,
            i32,
        >,
        money: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-commonrewarddialog")]
impl CommonRewardDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRewardDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRewardDialogMethods>::ctor(this);
        this
    }
}
