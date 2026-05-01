
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nn/hid/npadbutton/NpadButton.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NpadButton {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NpadButton {
    const NAMESPACE: &'static str = "nn.hid";

    const NAME: &'static str = "NpadButton";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NpadButton {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NpadButton {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn a() -> Self {
        Self { value: 1 }
    }

    pub fn b() -> Self {
        Self { value: 2 }
    }

    pub fn x() -> Self {
        Self { value: 4 }
    }

    pub fn y() -> Self {
        Self { value: 8 }
    }

    pub fn stick_l() -> Self {
        Self { value: 16 }
    }

    pub fn stick_r() -> Self {
        Self { value: 32 }
    }

    pub fn l() -> Self {
        Self { value: 64 }
    }

    pub fn r() -> Self {
        Self { value: 128 }
    }

    pub fn zl() -> Self {
        Self { value: 256 }
    }

    pub fn zr() -> Self {
        Self { value: 512 }
    }

    pub fn plus() -> Self {
        Self { value: 1024 }
    }

    pub fn minus() -> Self {
        Self { value: 2048 }
    }

    pub fn left() -> Self {
        Self { value: 4096 }
    }

    pub fn up() -> Self {
        Self { value: 8192 }
    }

    pub fn right() -> Self {
        Self { value: 16384 }
    }

    pub fn down() -> Self {
        Self { value: 32768 }
    }

    pub fn stick_l_left() -> Self {
        Self { value: 65536 }
    }

    pub fn stick_l_up() -> Self {
        Self { value: 131072 }
    }

    pub fn stick_l_right() -> Self {
        Self { value: 262144 }
    }

    pub fn stick_l_down() -> Self {
        Self { value: 524288 }
    }

    pub fn stick_r_left() -> Self {
        Self { value: 1048576 }
    }

    pub fn stick_r_up() -> Self {
        Self { value: 2097152 }
    }

    pub fn stick_r_right() -> Self {
        Self { value: 4194304 }
    }

    pub fn stick_r_down() -> Self {
        Self { value: 8388608 }
    }

    pub fn left_sl() -> Self {
        Self { value: 16777216 }
    }

    pub fn left_sr() -> Self {
        Self { value: 33554432 }
    }

    pub fn right_sl() -> Self {
        Self { value: 67108864 }
    }

    pub fn right_sr() -> Self {
        Self { value: 134217728 }
    }
}
