
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/sortingcriteria/SortingCriteria.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortingCriteria {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortingCriteria {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SortingCriteria";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortingCriteria {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortingCriteria {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn sorting_layer() -> Self {
        Self { value: 1 }
    }

    pub fn render_queue() -> Self {
        Self { value: 2 }
    }

    pub fn back_to_front() -> Self {
        Self { value: 4 }
    }

    pub fn quantized_front_to_back() -> Self {
        Self { value: 8 }
    }

    pub fn optimize_state_changes() -> Self {
        Self { value: 16 }
    }

    pub fn canvas_order() -> Self {
        Self { value: 32 }
    }

    pub fn renderer_priority() -> Self {
        Self { value: 64 }
    }

    pub fn common_opaque() -> Self {
        Self { value: 59 }
    }

    pub fn common_transparent() -> Self {
        Self { value: 23 }
    }
}
