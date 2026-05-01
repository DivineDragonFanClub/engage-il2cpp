
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/compatiblecomparer_2/CompatibleComparer_2.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "CompatibleComparer"
)]
#[parent(crate::system::object::Object)]
pub struct CompatibleComparer_2 {
    #[rename(name = "_comparer")]
    pub comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    #[static_field]
    #[rename(name = "defaultComparer")]
    pub default_comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    #[rename(name = "_hcp")]
    pub hcp: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
    #[static_field]
    #[rename(name = "defaultHashProvider")]
    pub default_hash_provider: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
}

#[cfg(feature = "system-collections-specialized-compatiblecomparer_2")]
#[::unity2::methods]
impl CompatibleComparer_2 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
        hash_code_provider: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
    ) -> ();

    #[method(name = "Equals", args = 2)]
    pub fn equals(self, a: crate::system::object::Object, b: crate::system::object::Object)
        -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: crate::system::object::Object) -> i32;

    #[method(name = "get_Comparer", args = 0)]
    pub fn get_comparer(
        self,
    ) -> crate::system::collections::icomparer_interface::IComparer_Interface;

    #[method(name = "get_HashCodeProvider", args = 0)]
    pub fn get_hash_code_provider(
        self,
    ) -> crate::system::collections::ihashcodeprovider::IHashCodeProvider;

    #[method(name = "get_DefaultComparer", args = 0)]
    pub fn get_default_comparer(
    ) -> crate::system::collections::icomparer_interface::IComparer_Interface;

    #[method(name = "get_DefaultHashCodeProvider", args = 0)]
    pub fn get_default_hash_code_provider(
    ) -> crate::system::collections::ihashcodeprovider::IHashCodeProvider;
}

#[cfg(feature = "system-collections-specialized-compatiblecomparer_2")]
impl CompatibleComparer_2 {
    pub fn new(
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
        hash_code_provider: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CompatibleComparer_2),
                ::core::stringify!(new),
            )
        });
        <Self as ICompatibleComparer_2Methods>::ctor(this, comparer, hash_code_provider);
        this
    }
}
