
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/stencilstate/StencilState.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct StencilState {
    pub m_enabled: u8,
    pub m_read_mask: u8,
    pub m_write_mask: u8,
    pub m_padding: u8,
    pub m_compare_function_front: u8,
    pub m_pass_operation_front: u8,
    pub m_fail_operation_front: u8,
    pub m_z_fail_operation_front: u8,
    pub m_compare_function_back: u8,
    pub m_pass_operation_back: u8,
    pub m_fail_operation_back: u8,
    pub m_z_fail_operation_back: u8,
}

impl ::unity2::ClassIdentity for StencilState {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "StencilState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for StencilState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-stencilstate")]
#[::unity2::methods(value)]
impl StencilState {
    #[method(name = "get_defaultValue", args = 0)]
    pub fn get_default_value() -> crate::unity_engine::rendering::stencilstate::StencilState;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        enabled: bool,
        read_mask: u8,
        write_mask: u8,
        compare_function: crate::unity_engine::rendering::comparefunction::CompareFunction,
        pass_operation: crate::unity_engine::rendering::stencilop::StencilOp,
        fail_operation: crate::unity_engine::rendering::stencilop::StencilOp,
        z_fail_operation: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = ".ctor", args = 11)]
    pub fn ctor_2(
        self,
        enabled: bool,
        read_mask: u8,
        write_mask: u8,
        compare_function_front: crate::unity_engine::rendering::comparefunction::CompareFunction,
        pass_operation_front: crate::unity_engine::rendering::stencilop::StencilOp,
        fail_operation_front: crate::unity_engine::rendering::stencilop::StencilOp,
        z_fail_operation_front: crate::unity_engine::rendering::stencilop::StencilOp,
        compare_function_back: crate::unity_engine::rendering::comparefunction::CompareFunction,
        pass_operation_back: crate::unity_engine::rendering::stencilop::StencilOp,
        fail_operation_back: crate::unity_engine::rendering::stencilop::StencilOp,
        z_fail_operation_back: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "get_readMask", args = 0)]
    pub fn get_read_mask(self) -> u8;

    #[method(name = "get_writeMask", args = 0)]
    pub fn get_write_mask(self) -> u8;

    #[method(name = "SetCompareFunction", args = 1)]
    pub fn set_compare_function(
        self,
        value: crate::unity_engine::rendering::comparefunction::CompareFunction,
    ) -> ();

    #[method(name = "SetPassOperation", args = 1)]
    pub fn set_pass_operation(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "SetFailOperation", args = 1)]
    pub fn set_fail_operation(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "SetZFailOperation", args = 1)]
    pub fn set_z_fail_operation(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_compareFunctionFront", args = 0)]
    pub fn get_compare_function_front(
        self,
    ) -> crate::unity_engine::rendering::comparefunction::CompareFunction;

    #[method(name = "set_compareFunctionFront", args = 1)]
    pub fn set_compare_function_front(
        self,
        value: crate::unity_engine::rendering::comparefunction::CompareFunction,
    ) -> ();

    #[method(name = "get_passOperationFront", args = 0)]
    pub fn get_pass_operation_front(self) -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_passOperationFront", args = 1)]
    pub fn set_pass_operation_front(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_failOperationFront", args = 0)]
    pub fn get_fail_operation_front(self) -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_failOperationFront", args = 1)]
    pub fn set_fail_operation_front(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_zFailOperationFront", args = 0)]
    pub fn get_z_fail_operation_front(self)
        -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_zFailOperationFront", args = 1)]
    pub fn set_z_fail_operation_front(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_compareFunctionBack", args = 0)]
    pub fn get_compare_function_back(
        self,
    ) -> crate::unity_engine::rendering::comparefunction::CompareFunction;

    #[method(name = "set_compareFunctionBack", args = 1)]
    pub fn set_compare_function_back(
        self,
        value: crate::unity_engine::rendering::comparefunction::CompareFunction,
    ) -> ();

    #[method(name = "get_passOperationBack", args = 0)]
    pub fn get_pass_operation_back(self) -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_passOperationBack", args = 1)]
    pub fn set_pass_operation_back(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_failOperationBack", args = 0)]
    pub fn get_fail_operation_back(self) -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_failOperationBack", args = 1)]
    pub fn set_fail_operation_back(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "get_zFailOperationBack", args = 0)]
    pub fn get_z_fail_operation_back(self) -> crate::unity_engine::rendering::stencilop::StencilOp;

    #[method(name = "set_zFailOperationBack", args = 1)]
    pub fn set_z_fail_operation_back(
        self,
        value: crate::unity_engine::rendering::stencilop::StencilOp,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::rendering::stencilstate::StencilState) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
