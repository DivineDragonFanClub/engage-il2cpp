
use crate::app::struct_object::baseitem::BaseItem;
use crate::app::struct_object::baseitem::IBaseItem;
use crate::app::struct_object::basepiece::BasePiece;
use crate::app::struct_object::basepiece::IBasePiece;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/struct_object/basearrayitem_1/BaseArrayItem_1.md")))]
#[::unity2::class(namespace = "App.StructObject", name = "BaseArrayItem`1")]
pub struct BaseArrayItem_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-struct_object-basearrayitem_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> BaseArrayItem_1<T0> {
    #[method(name = "get_Pieces", args = 0)]
    pub fn get_pieces(self) -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "set_Pieces", args = 1)]
    pub fn set_pieces(self, value: crate::system::collections::generic::list_1::List_1<T0>) -> ();

    #[method(name = "AddPiece", args = 1)]
    pub fn add_piece(self, piece: crate::system::object::Object) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-struct_object-basearrayitem_1")]
impl<T0: ::unity2::ClassIdentity> BaseArrayItem_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseArrayItem_1),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseArrayItem_1Methods<T0>>::ctor(this);
        this
    }
}
