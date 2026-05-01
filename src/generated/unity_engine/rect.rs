
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rect/Rect.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Rect {
    pub m_x_min: f32,
    pub m_y_min: f32,
    pub m_width: f32,
    pub m_height: f32,
}

impl ::unity2::ClassIdentity for Rect {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Rect";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Rect {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rect")]
#[::unity2::methods(value)]
impl Rect {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x: f32, y: f32, width: f32, height: f32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        position: crate::unity_engine::vector2::Vector2,
        size: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, source: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::unity_engine::rect::Rect;

    #[method(name = "MinMaxRect", args = 4)]
    pub fn min_max_rect(
        xmin: f32,
        ymin: f32,
        xmax: f32,
        ymax: f32,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "Set", args = 4)]
    pub fn set(self, x: f32, y: f32, width: f32, height: f32) -> ();

    #[method(name = "get_x", args = 0)]
    pub fn get_x(self) -> f32;

    #[method(name = "set_x", args = 1)]
    pub fn set_x(self, value: f32) -> ();

    #[method(name = "get_y", args = 0)]
    pub fn get_y(self) -> f32;

    #[method(name = "set_y", args = 1)]
    pub fn set_y(self, value: f32) -> ();

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_min", args = 0)]
    pub fn get_min(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_max", args = 0)]
    pub fn get_max(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "set_width", args = 1)]
    pub fn set_width(self, value: f32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: f32) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_xMin", args = 0)]
    pub fn get_x_min(self) -> f32;

    #[method(name = "set_xMin", args = 1)]
    pub fn set_x_min(self, value: f32) -> ();

    #[method(name = "get_yMin", args = 0)]
    pub fn get_y_min(self) -> f32;

    #[method(name = "set_yMin", args = 1)]
    pub fn set_y_min(self, value: f32) -> ();

    #[method(name = "get_xMax", args = 0)]
    pub fn get_x_max(self) -> f32;

    #[method(name = "set_xMax", args = 1)]
    pub fn set_x_max(self, value: f32) -> ();

    #[method(name = "get_yMax", args = 0)]
    pub fn get_y_max(self) -> f32;

    #[method(name = "set_yMax", args = 1)]
    pub fn set_y_max(self, value: f32) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, point: crate::unity_engine::vector2::Vector2) -> bool;

    #[method(name = "Contains", args = 1)]
    pub fn contains_2(self, point: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "OrderMinMax", args = 1)]
    pub fn order_min_max(rect: crate::unity_engine::rect::Rect) -> crate::unity_engine::rect::Rect;

    #[method(name = "Overlaps", args = 1)]
    pub fn overlaps(self, other: crate::unity_engine::rect::Rect) -> bool;

    #[method(name = "Overlaps", args = 2)]
    pub fn overlaps_2(self, other: crate::unity_engine::rect::Rect, allow_inverse: bool) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::rect::Rect,
        rhs: crate::unity_engine::rect::Rect,
    ) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::rect::Rect,
        rhs: crate::unity_engine::rect::Rect,
    ) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::rect::Rect) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
