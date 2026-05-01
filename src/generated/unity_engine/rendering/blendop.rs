
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/blendop/BlendOp.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BlendOp {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BlendOp {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "BlendOp";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BlendOp {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BlendOp {
    pub fn add() -> Self {
        Self { value: 0 }
    }

    pub fn subtract() -> Self {
        Self { value: 1 }
    }

    pub fn reverse_subtract() -> Self {
        Self { value: 2 }
    }

    pub fn min() -> Self {
        Self { value: 3 }
    }

    pub fn max() -> Self {
        Self { value: 4 }
    }

    pub fn logical_clear() -> Self {
        Self { value: 5 }
    }

    pub fn logical_set() -> Self {
        Self { value: 6 }
    }

    pub fn logical_copy() -> Self {
        Self { value: 7 }
    }

    pub fn logical_copy_inverted() -> Self {
        Self { value: 8 }
    }

    pub fn logical_noop() -> Self {
        Self { value: 9 }
    }

    pub fn logical_invert() -> Self {
        Self { value: 10 }
    }

    pub fn logical_and() -> Self {
        Self { value: 11 }
    }

    pub fn logical_nand() -> Self {
        Self { value: 12 }
    }

    pub fn logical_or() -> Self {
        Self { value: 13 }
    }

    pub fn logical_nor() -> Self {
        Self { value: 14 }
    }

    pub fn logical_xor() -> Self {
        Self { value: 15 }
    }

    pub fn logical_equivalence() -> Self {
        Self { value: 16 }
    }

    pub fn logical_and_reverse() -> Self {
        Self { value: 17 }
    }

    pub fn logical_and_inverted() -> Self {
        Self { value: 18 }
    }

    pub fn logical_or_reverse() -> Self {
        Self { value: 19 }
    }

    pub fn logical_or_inverted() -> Self {
        Self { value: 20 }
    }

    pub fn multiply() -> Self {
        Self { value: 21 }
    }

    pub fn screen() -> Self {
        Self { value: 22 }
    }

    pub fn overlay() -> Self {
        Self { value: 23 }
    }

    pub fn darken() -> Self {
        Self { value: 24 }
    }

    pub fn lighten() -> Self {
        Self { value: 25 }
    }

    pub fn color_dodge() -> Self {
        Self { value: 26 }
    }

    pub fn color_burn() -> Self {
        Self { value: 27 }
    }

    pub fn hard_light() -> Self {
        Self { value: 28 }
    }

    pub fn soft_light() -> Self {
        Self { value: 29 }
    }

    pub fn difference() -> Self {
        Self { value: 30 }
    }

    pub fn exclusion() -> Self {
        Self { value: 31 }
    }

    pub fn hsl_hue() -> Self {
        Self { value: 32 }
    }

    pub fn hsl_saturation() -> Self {
        Self { value: 33 }
    }

    pub fn hsl_color() -> Self {
        Self { value: 34 }
    }

    pub fn hsl_luminosity() -> Self {
        Self { value: 35 }
    }
}
