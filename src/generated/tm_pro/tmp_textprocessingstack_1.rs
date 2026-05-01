
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_textprocessingstack_1/TMP_TextProcessingStack_1.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_TextProcessingStack_1<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for TMP_TextProcessingStack_1<T0> {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_TextProcessingStack`1";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for TMP_TextProcessingStack_1<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-tmp_textprocessingstack_1")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity> TMP_TextProcessingStack_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stack: ::unity2::Array<T0>) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(self, capacity: i32, rollover_size: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_current", args = 0)]
    pub fn get_current(self) -> T0;

    #[method(name = "get_rolloverSize", args = 0)]
    pub fn get_rollover_size(self) -> i32;

    #[method(name = "set_rolloverSize", args = 1)]
    pub fn set_rollover_size(self, value: i32) -> ();

    #[method(name = "SetDefault", args = 2)]
    pub fn set_default(
        stack: ::unity2::Array<
            crate::tm_pro::tmp_textprocessingstack_1::TMP_TextProcessingStack_1<T0>,
        >,
        item: T0,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetDefault", args = 1)]
    pub fn set_default_2(self, item: T0) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> ();

    #[method(name = "Remove", args = 0)]
    pub fn remove(self) -> T0;

    #[method(name = "Push", args = 1)]
    pub fn push(self, item: T0) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> T0;

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> T0;

    #[method(name = "CurrentItem", args = 0)]
    pub fn current_item(self) -> T0;

    #[method(name = "PreviousItem", args = 0)]
    pub fn previous_item(self) -> T0;
}
