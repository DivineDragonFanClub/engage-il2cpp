
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_CheckTextResultFunction.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.CheckTextResultFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Ngc_CheckTextResultFunction {}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_CheckTextResultFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, is_ok: bool) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_CheckTextResultFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_CheckTextResultFunction),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_CheckTextResultFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_MaskTextSequence.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.MaskTextSequence")]
#[parent(crate::app::ngc::Ngc_MaskTextSequenceBase)]
pub struct Ngc_MaskTextSequence {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::ngc::Ngc_MaskTextResultFunction,
}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_MaskTextSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_MaskTextResultFunction,
    ) -> ();

    #[method(name = "CallResultFunc", args = 0)]
    pub fn call_result_func(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_MaskTextResultFunction,
    ) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_MaskTextSequence {
    pub fn new(
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_MaskTextResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_MaskTextSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_MaskTextSequenceMethods>::ctor(this, text, result_func);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc.md")))]
#[::unity2::class(namespace = "App", name = "Ngc")]
#[parent(crate::system::object::Object)]
pub struct Ngc {}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc {
    #[method(name = "CreateBindMaskText", args = 3)]
    pub fn create_bind_mask_text(
        super_: crate::app::procinst::ProcInst,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_MaskTextResultFunction,
    ) -> ();

    #[method(name = "CreateBindCheckText", args = 3)]
    pub fn create_bind_check_text(
        super_: crate::app::procinst::ProcInst,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_CheckTextResultFunction,
    ) -> ();

    #[method(name = "CreateBindCheckMultiText", args = 3)]
    pub fn create_bind_check_multi_text(
        super_: crate::app::procinst::ProcInst,
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        result_func: crate::app::ngc::Ngc_CheckMultiTextResultFunction,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_MaskTextSequenceBase_Data.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Ngc_MaskTextSequenceBase_Data {
    pub original_text: ::unity2::Il2CppString,
    pub is_success: bool,
    pub result_text: ::unity2::Il2CppString,
    pub ng_word_count: i32,
}

impl ::unity2::ClassIdentity for Ngc_MaskTextSequenceBase_Data {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Ngc.MaskTextSequenceBase.Data";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Ngc_MaskTextSequenceBase_Data {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-ngc")]
#[::unity2::methods(value)]
impl Ngc_MaskTextSequenceBase_Data {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "IsOk", args = 0)]
    pub fn is_ok(self) -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_CheckTextSequence.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.CheckTextSequence")]
#[parent(crate::app::ngc::Ngc_MaskTextSequenceBase)]
pub struct Ngc_CheckTextSequence {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::ngc::Ngc_CheckTextResultFunction,
}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_CheckTextSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_CheckTextResultFunction,
    ) -> ();

    #[method(name = "CallResultFunc", args = 0)]
    pub fn call_result_func(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_CheckTextResultFunction,
    ) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_CheckTextSequence {
    pub fn new(
        text: ::unity2::Il2CppString,
        result_func: crate::app::ngc::Ngc_CheckTextResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_CheckTextSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_CheckTextSequenceMethods>::ctor(this, text, result_func);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_MaskTextSequenceBase.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.MaskTextSequenceBase")]
#[parent(crate::app::procinst::ProcInst)]
pub struct Ngc_MaskTextSequenceBase {
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<crate::app::ngc::Ngc_MaskTextSequenceBase_Data>,
}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_MaskTextSequenceBase {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "StartThread", args = 0)]
    pub fn start_thread(self) -> ();

    #[method(name = "WaitThread", args = 0)]
    pub fn wait_thread(self) -> ();

    #[method(name = "CallResultFunc", args = 0)]
    pub fn call_result_func(self) -> ();

    #[method(name = "Result", args = 0)]
    pub fn result(self) -> ();

    #[method(name = "ThreadFunc", args = 1)]
    pub fn thread_func(obj: crate::system::object::Object) -> ();

    #[method(name = "CreateBindImpl", args = 1)]
    pub fn create_bind_impl(self, super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_MaskTextSequenceBase {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_MaskTextSequenceBase),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_MaskTextSequenceBaseMethods>::ctor(this, text);
        this
    }

    pub fn new_2(
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_MaskTextSequenceBase),
                ::core::stringify!(new_2),
            )
        });
        <Self as INgc_MaskTextSequenceBaseMethods>::ctor_2(this, texts);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_CheckMultiTextSequence.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.CheckMultiTextSequence")]
#[parent(crate::app::ngc::Ngc_MaskTextSequenceBase)]
pub struct Ngc_CheckMultiTextSequence {
    #[rename(name = "m_ResultFunc")]
    pub m_result_func: crate::app::ngc::Ngc_CheckMultiTextResultFunction,
}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_CheckMultiTextSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        result_func: crate::app::ngc::Ngc_CheckMultiTextResultFunction,
    ) -> ();

    #[method(name = "CallResultFunc", args = 0)]
    pub fn call_result_func(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        result_func: crate::app::ngc::Ngc_CheckMultiTextResultFunction,
    ) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_CheckMultiTextSequence {
    pub fn new(
        texts: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        result_func: crate::app::ngc::Ngc_CheckMultiTextResultFunction,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_CheckMultiTextSequence),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_CheckMultiTextSequenceMethods>::ctor(this, texts, result_func);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_CheckMultiTextResultFunction.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.CheckMultiTextResultFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Ngc_CheckMultiTextResultFunction {}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_CheckMultiTextResultFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, is_ok: ::unity2::Array<bool>) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_CheckMultiTextResultFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_CheckMultiTextResultFunction),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_CheckMultiTextResultFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ngc/Ngc_MaskTextResultFunction.md")))]
#[::unity2::class(namespace = "App", name = "Ngc.MaskTextResultFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Ngc_MaskTextResultFunction {}

#[cfg(feature = "app-ngc")]
#[::unity2::methods]
impl Ngc_MaskTextResultFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        is_success: bool,
        result_text: ::unity2::Il2CppString,
        ng_word_count: i32,
    ) -> ();
}

#[cfg(feature = "app-ngc")]
impl Ngc_MaskTextResultFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Ngc_MaskTextResultFunction),
                ::core::stringify!(new),
            )
        });
        <Self as INgc_MaskTextResultFunctionMethods>::ctor(this, object, method);
        this
    }
}
