
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/neterror/NetError_App.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NetError_App {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NetError_App {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NetError.App";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NetError_App {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NetError_App {
    pub fn general() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/neterror/NetError.md")))]
#[::unity2::class(namespace = "App", name = "NetError")]
#[parent(crate::system::object::Object)]
pub struct NetError {
    #[static_field]
    #[rename(name = "s_Kind")]
    pub s_kind: crate::app::neterror::NetError_Kind,
    #[static_field]
    #[rename(name = "s_App")]
    pub s_app: crate::app::neterror::NetError_App,
}

#[cfg(feature = "app-neterror")]
#[::unity2::methods]
impl NetError {
    #[method(name = "Show", args = 1)]
    pub fn show(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Show", args = 2)]
    pub fn show_2(
        super_: crate::app::procinst::ProcInst,
        app: crate::app::neterror::NetError_App,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "SetError", args = 1)]
    pub fn set_error(error: crate::app::neterror::NetError_App) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/neterror/NetError_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NetError_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NetError_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NetError.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NetError_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NetError_Kind {
    pub fn system() -> Self {
        Self { value: 0 }
    }

    pub fn app() -> Self {
        Self { value: 1 }
    }
}
