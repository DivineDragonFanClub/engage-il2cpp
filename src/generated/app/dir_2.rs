
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dir_2/Dir_2.md")))]
#[::unity2::class(namespace = "App", name = "Dir")]
#[parent(crate::system::object::Object)]
pub struct Dir_2 {}

#[cfg(feature = "app-dir_2")]
#[::unity2::methods]
impl Dir_2 {
    #[method(name = "GetDir", args = 4)]
    pub fn get_dir(sx: i32, sz: i32, tx: i32, tz: i32) -> crate::app::dir_2::Dir_Type;

    #[method(name = "GetReverse", args = 1)]
    pub fn get_reverse(dir: crate::app::dir_2::Dir_Type) -> crate::app::dir_2::Dir_Type;

    #[method(name = "GetX", args = 1)]
    pub fn get_x(dir: crate::app::dir_2::Dir_Type) -> i32;

    #[method(name = "GetZ", args = 1)]
    pub fn get_z(dir: crate::app::dir_2::Dir_Type) -> i32;

    #[method(name = "IsTerminate", args = 1)]
    pub fn is_terminate(dir: crate::app::dir_2::Dir_Type) -> bool;

    #[method(name = "GetAngle", args = 1)]
    pub fn get_angle(dir: crate::app::dir_2::Dir_Type) -> i32;

    #[method(name = "GetRotateL90", args = 1)]
    pub fn get_rotate_l90(dir: crate::app::dir_2::Dir_Type) -> crate::app::dir_2::Dir_Type;

    #[method(name = "GetRotateR90", args = 1)]
    pub fn get_rotate_r90(dir: crate::app::dir_2::Dir_Type) -> crate::app::dir_2::Dir_Type;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dir_2")]
impl Dir_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dir_2),
                ::core::stringify!(new),
            )
        });
        <Self as IDir_2Methods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dir_2/Dir_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Dir_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Dir_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Dir.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Dir_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Dir_Type {
    pub fn left() -> Self {
        Self { value: 134480385 }
    }

    pub fn right() -> Self {
        Self { value: 537396226 }
    }

    pub fn down() -> Self {
        Self { value: -2145384444 }
    }

    pub fn up() -> Self {
        Self { value: 8396808 }
    }

    pub fn center() -> Self {
        Self { value: 151027744 }
    }

    pub fn terminate() -> Self {
        Self { value: 168362112 }
    }

    pub fn none() -> Self {
        Self { value: 84543744 }
    }

    pub fn up_left() -> Self {
        Self { value: 100993545 }
    }

    pub fn up_right() -> Self {
        Self { value: 17171722 }
    }

    pub fn down_left() -> Self {
        Self { value: 67077 }
    }

    pub fn down_right() -> Self {
        Self { value: 262 }
    }
}
