
use crate::app::capabilitydefinition::CapabilityDefinition;
use crate::app::capabilitydefinition::ICapabilityDefinition;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/capabilitybase_1/CapabilityBase_1.md")))]
#[::unity2::class(namespace = "App", name = "CapabilityBase`1")]
pub struct CapabilityBase_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Data")]
    pub m_data: ::unity2::Array<T0>,
}

#[cfg(feature = "app-capabilitybase_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> CapabilityBase_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: T0) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> T0;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, i: i32, value: T0) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, i: i32, v: T0) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, i: i32) -> T0;

    #[method(name = "Add", args = 2)]
    pub fn add(self, i: i32, v: T0) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(self, t: crate::app::capabilitydefinition::CapabilityDefinition_Type, v: T0)
        -> ();

    #[method(name = "Get", args = 1)]
    pub fn get_2(self, t: crate::app::capabilitydefinition::CapabilityDefinition_Type) -> T0;

    #[method(name = "Add", args = 2)]
    pub fn add_2(self, t: crate::app::capabilitydefinition::CapabilityDefinition_Type, v: T0)
        -> ();

    #[method(name = "AddHp", args = 1)]
    pub fn add_hp(self, v: T0) -> ();

    #[method(name = "AddStr", args = 1)]
    pub fn add_str(self, v: T0) -> ();

    #[method(name = "AddTech", args = 1)]
    pub fn add_tech(self, v: T0) -> ();

    #[method(name = "AddQuick", args = 1)]
    pub fn add_quick(self, v: T0) -> ();

    #[method(name = "AddLuck", args = 1)]
    pub fn add_luck(self, v: T0) -> ();

    #[method(name = "AddDef", args = 1)]
    pub fn add_def(self, v: T0) -> ();

    #[method(name = "AddMagic", args = 1)]
    pub fn add_magic(self, v: T0) -> ();

    #[method(name = "AddMdef", args = 1)]
    pub fn add_mdef(self, v: T0) -> ();

    #[method(name = "AddPhys", args = 1)]
    pub fn add_phys(self, v: T0) -> ();

    #[method(name = "AddSight", args = 1)]
    pub fn add_sight(self, v: T0) -> ();

    #[method(name = "AddMove", args = 1)]
    pub fn add_move(self, v: T0) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_3(self, v: crate::app::capabilitybase_1::CapabilityBase_1<T0>) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_3(self, v: crate::app::capabilitybase_1::CapabilityBase_1<T0>) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::capabilitybase_1::CapabilityBase_1<T0>) -> ();

    #[method(name = "IsZero", args = 0)]
    pub fn is_zero(self) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "WriteToStream", args = 2)]
    pub fn write_to_stream(self, stream: crate::app::stream_2::Stream_2, v: T0) -> ();

    #[method(name = "ReadFromStream", args = 1)]
    pub fn read_from_stream(self, stream: crate::app::stream_2::Stream_2) -> T0;

    #[method(name = "get_Hp", args = 0)]
    pub fn get_hp(self) -> T0;

    #[method(name = "set_Hp", args = 1)]
    pub fn set_hp(self, value: T0) -> ();

    #[method(name = "get_Str", args = 0)]
    pub fn get_str(self) -> T0;

    #[method(name = "set_Str", args = 1)]
    pub fn set_str(self, value: T0) -> ();

    #[method(name = "get_Tech", args = 0)]
    pub fn get_tech(self) -> T0;

    #[method(name = "set_Tech", args = 1)]
    pub fn set_tech(self, value: T0) -> ();

    #[method(name = "get_Quick", args = 0)]
    pub fn get_quick(self) -> T0;

    #[method(name = "set_Quick", args = 1)]
    pub fn set_quick(self, value: T0) -> ();

    #[method(name = "get_Luck", args = 0)]
    pub fn get_luck(self) -> T0;

    #[method(name = "set_Luck", args = 1)]
    pub fn set_luck(self, value: T0) -> ();

    #[method(name = "get_Def", args = 0)]
    pub fn get_def(self) -> T0;

    #[method(name = "set_Def", args = 1)]
    pub fn set_def(self, value: T0) -> ();

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> T0;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: T0) -> ();

    #[method(name = "get_Mdef", args = 0)]
    pub fn get_mdef(self) -> T0;

    #[method(name = "set_Mdef", args = 1)]
    pub fn set_mdef(self, value: T0) -> ();

    #[method(name = "get_Phys", args = 0)]
    pub fn get_phys(self) -> T0;

    #[method(name = "set_Phys", args = 1)]
    pub fn set_phys(self, value: T0) -> ();

    #[method(name = "get_Sight", args = 0)]
    pub fn get_sight(self) -> T0;

    #[method(name = "set_Sight", args = 1)]
    pub fn set_sight(self, value: T0) -> ();

    #[method(name = "get_Move", args = 0)]
    pub fn get_move(self) -> T0;

    #[method(name = "set_Move", args = 1)]
    pub fn set_move(self, value: T0) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "app-capabilitybase_1")]
impl<T0: ::unity2::ClassIdentity> CapabilityBase_1<T0> {
    pub fn new(value: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CapabilityBase_1),
                ::core::stringify!(new),
            )
        });
        <Self as ICapabilityBase_1Methods<T0>>::ctor(this, value);
        this
    }
}
