
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashhelpers/HashHelpers.md")))]
#[::unity2::class(namespace = "System.Collections", name = "HashHelpers")]
#[parent(crate::system::object::Object)]
pub struct HashHelpers {
    #[static_field]
    #[rename(name = "primes")]
    pub primes: ::unity2::Array<i32>,
}

#[cfg(feature = "system-collections-hashhelpers")]
#[::unity2::methods]
impl HashHelpers {
    #[method(name = "IsPrime", args = 1)]
    pub fn is_prime(candidate: i32) -> bool;

    #[method(name = "GetPrime", args = 1)]
    pub fn get_prime(min: i32) -> i32;

    #[method(name = "ExpandPrime", args = 1)]
    pub fn expand_prime(old_size: i32) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
