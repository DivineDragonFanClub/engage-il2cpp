
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_listpool_1/TMP_ListPool_1.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_ListPool`1")]
pub struct TMP_ListPool_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_ListPool")]
    pub s_list_pool: crate::tm_pro::tmp_objectpool_1::TMP_ObjectPool_1<
        crate::system::collections::generic::list_1::List_1<T0>,
    >,
}

#[cfg(feature = "tm_pro-tmp_listpool_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> TMP_ListPool_1<T0> {
    #[method(name = "Get", args = 0)]
    pub fn get() -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "Release", args = 1)]
    pub fn release(to_release: crate::system::collections::generic::list_1::List_1<T0>) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
