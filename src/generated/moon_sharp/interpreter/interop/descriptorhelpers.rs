
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/descriptorhelpers/DescriptorHelpers.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "DescriptorHelpers"
)]
#[parent(crate::system::object::Object)]
pub struct DescriptorHelpers {}

#[cfg(feature = "moon_sharp-interpreter-interop-descriptorhelpers")]
#[::unity2::methods]
impl DescriptorHelpers {
    #[method(name = "IsDelegateType", args = 1)]
    pub fn is_delegate_type(t: ::unity2::SystemType) -> bool;

    #[method(name = "GetClrVisibility", args = 1)]
    pub fn get_clr_visibility(r#type: ::unity2::SystemType) -> ::unity2::Il2CppString;

    #[method(name = "GetClrVisibility", args = 1)]
    pub fn get_clr_visibility_2(
        info: crate::system::reflection::fieldinfo::FieldInfo,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetClrVisibility", args = 1)]
    pub fn get_clr_visibility_3(
        info: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetClrVisibility", args = 1)]
    pub fn get_clr_visibility_4(
        info: crate::system::reflection::methodbase::MethodBase,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsPropertyInfoPublic", args = 1)]
    pub fn is_property_info_public(
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> bool;

    #[method(name = "GetMetaNamesFromAttributes", args = 1)]
    pub fn get_meta_names_from_attributes(
        mi: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "SafeGetTypes", args = 1)]
    pub fn safe_get_types(
        asm: crate::system::reflection::assembly::Assembly,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetConversionMethodName", args = 1)]
    pub fn get_conversion_method_name(r#type: ::unity2::SystemType) -> ::unity2::Il2CppString;

    #[method(name = "GetAllImplementedTypes", args = 1)]
    pub fn get_all_implemented_types(
        t: ::unity2::SystemType,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::SystemType>;

    #[method(name = "IsValidSimpleIdentifier", args = 1)]
    pub fn is_valid_simple_identifier(str: ::unity2::Il2CppString) -> bool;

    #[method(name = "ToValidSimpleIdentifier", args = 1)]
    pub fn to_valid_simple_identifier(str: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Camelify", args = 1)]
    pub fn camelify(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "UpperFirstLetter", args = 1)]
    pub fn upper_first_letter(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}
