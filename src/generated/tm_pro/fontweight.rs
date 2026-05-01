
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/fontweight/FontWeight.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FontWeight {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FontWeight {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "FontWeight";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FontWeight {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FontWeight {
    pub fn thin() -> Self {
        Self { value: 100 }
    }

    pub fn extra_light() -> Self {
        Self { value: 200 }
    }

    pub fn light() -> Self {
        Self { value: 300 }
    }

    pub fn regular() -> Self {
        Self { value: 400 }
    }

    pub fn medium() -> Self {
        Self { value: 500 }
    }

    pub fn semi_bold() -> Self {
        Self { value: 600 }
    }

    pub fn bold() -> Self {
        Self { value: 700 }
    }

    pub fn heavy() -> Self {
        Self { value: 800 }
    }

    pub fn black() -> Self {
        Self { value: 900 }
    }
}
