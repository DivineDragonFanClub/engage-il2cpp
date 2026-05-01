
use crate::moon_sharp::interpreter::interop::basic_descriptors::dispatchinguserdatadescriptor::DispatchingUserDataDescriptor;
use crate::moon_sharp::interpreter::interop::basic_descriptors::dispatchinguserdatadescriptor::IDispatchingUserDataDescriptor;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standarduserdatadescriptor/StandardUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "StandardUserDataDescriptor"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: dispatchinguserdatadescriptor :: DispatchingUserDataDescriptor)]
pub struct StandardUserDataDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-standarduserdatadescriptor")]
#[::unity2::methods]
impl StandardUserDataDescriptor {
    #[method(name = "get_AccessMode", args = 0)]
    pub fn get_access_mode(
        self,
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_AccessMode", args = 1)]
    pub fn set_access_mode(
        self,
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FillMemberList", args = 0)]
    pub fn fill_member_list(self) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(
        self,
        t: crate::moon_sharp::interpreter::table::Table,
        members : crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < crate :: system :: collections :: generic :: keyvaluepair_2 :: KeyValuePair_2 < :: unity2 :: Il2CppString , crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: imemberdescriptor_interface :: IMemberDescriptor_Interface > >,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-standarduserdatadescriptor")]
impl StandardUserDataDescriptor {
    pub fn new(
        r#type: ::unity2::SystemType,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
        friendly_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StandardUserDataDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IStandardUserDataDescriptorMethods>::ctor(
            this,
            r#type,
            access_mode,
            friendly_name,
        );
        this
    }
}
