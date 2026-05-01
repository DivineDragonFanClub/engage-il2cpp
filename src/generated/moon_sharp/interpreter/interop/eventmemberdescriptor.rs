
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper01.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper01"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper01 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper01 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, o1: crate::system::object::Object) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper01 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper01),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper01Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper12.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper12"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper12 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper12 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 12)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
        o12: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper12 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper12),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper12Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper00.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper00"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper00 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper00 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper00 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper00),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper00Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper11.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper11"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper11 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper11 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 11)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper11 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper11),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper11Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper06.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper06"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper06 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper06 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 6)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper06 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper06),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper06Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper16.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper16"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper16 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper16 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 16)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
        o12: crate::system::object::Object,
        o13: crate::system::object::Object,
        o14: crate::system::object::Object,
        o15: crate::system::object::Object,
        o16: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper16 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper16),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper16Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper02.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper02"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper02 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper02 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, o1: crate::system::object::Object, o2: crate::system::object::Object)
        -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper02 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper02),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper02Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper04.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper04"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper04 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper04 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper04 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper04),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper04Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper15.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper15"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper15 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper15 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 15)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
        o12: crate::system::object::Object,
        o13: crate::system::object::Object,
        o14: crate::system::object::Object,
        o15: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper15 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper15),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper15Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper14.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper14"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper14 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper14 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 14)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
        o12: crate::system::object::Object,
        o13: crate::system::object::Object,
        o14: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper14 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper14),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper14Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper05.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper05"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper05 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper05 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 5)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper05 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper05),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper05Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper03.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper03"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper03 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper03 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper03 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper03),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper03Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper10.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper10"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper10 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper10 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 10)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper10 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper10),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper10Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper09.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper09"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper09 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper09 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 9)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper09 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper09),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper09Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper13.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper13"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper13 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper13 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 13)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
        o9: crate::system::object::Object,
        o10: crate::system::object::Object,
        o11: crate::system::object::Object,
        o12: crate::system::object::Object,
        o13: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper13 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper13),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper13Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper07.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper07"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper07 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper07 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 7)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper07 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper07),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper07Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor"
)]
#[parent(crate::system::object::Object)]
pub struct EventMemberDescriptor {
    #[static_field]
    #[rename(name = "MAX_ARGS_IN_DELEGATE")]
    pub max_args_in_delegate: i32,
    #[rename(name = "m_Lock")]
    pub m_lock: ::unity2::IlInstance,
    #[rename(name = "m_Callbacks")]
    pub m_callbacks:
        crate::moon_sharp::interpreter::data_structs::multidictionary_2::MultiDictionary_2<
            crate::system::object::Object,
            crate::moon_sharp::interpreter::closure::Closure,
        >,
    #[rename(name = "m_Delegates")]
    pub m_delegates: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::system::object::Object,
        crate::system::delegate::Delegate,
    >,
    #[rename(name = "m_Add")]
    pub m_add: crate::system::reflection::methodinfo::MethodInfo,
    #[rename(name = "m_Remove")]
    pub m_remove: crate::system::reflection::methodinfo::MethodInfo,
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor {
    #[method(name = "TryCreateIfVisible", args = 2)]
    pub fn try_create_if_visible(
        ei: crate::system::reflection::eventinfo::EventInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> crate::moon_sharp::interpreter::interop::eventmemberdescriptor::EventMemberDescriptor;

    #[method(name = "CheckEventIsCompatible", args = 2)]
    pub fn check_event_is_compatible(
        ei: crate::system::reflection::eventinfo::EventInfo,
        throw_exception: bool,
    ) -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        ei: crate::system::reflection::eventinfo::EventInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "get_EventInfo", args = 0)]
    pub fn get_event_info(self) -> crate::system::reflection::eventinfo::EventInfo;

    #[method(name = "set_EventInfo", args = 1)]
    pub fn set_event_info(self, value: crate::system::reflection::eventinfo::EventInfo) -> ();

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "set_IsStatic", args = 1)]
    pub fn set_is_static(self, value: bool) -> ();

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "AddCallback", args = 3)]
    pub fn add_callback(
        self,
        o: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "RemoveCallback", args = 3)]
    pub fn remove_callback(
        self,
        o: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "RegisterCallback", args = 1)]
    pub fn register_callback(self, o: crate::system::object::Object) -> ();

    #[method(name = "UnregisterCallback", args = 1)]
    pub fn unregister_callback(self, o: crate::system::object::Object) -> ();

    #[method(name = "CreateDelegate", args = 1)]
    pub fn create_delegate(
        self,
        sender: crate::system::object::Object,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "DispatchEvent", args = 17)]
    pub fn dispatch_event(
        self,
        sender: crate::system::object::Object,
        o01: ::unity2::IlInstance,
        o02: ::unity2::IlInstance,
        o03: ::unity2::IlInstance,
        o04: ::unity2::IlInstance,
        o05: ::unity2::IlInstance,
        o06: ::unity2::IlInstance,
        o07: ::unity2::IlInstance,
        o08: ::unity2::IlInstance,
        o09: ::unity2::IlInstance,
        o10: ::unity2::IlInstance,
        o11: ::unity2::IlInstance,
        o12: ::unity2::IlInstance,
        o13: ::unity2::IlInstance,
        o14: ::unity2::IlInstance,
        o15: ::unity2::IlInstance,
        o16: ::unity2::IlInstance,
    ) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_MemberAccess", args = 0)]
    pub fn get_member_access (self ,) -> crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: memberdescriptoraccess :: MemberDescriptorAccess ;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        v: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor {
    pub fn new(
        ei: crate::system::reflection::eventinfo::EventInfo,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptorMethods>::ctor(this, ei, access_mode);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/eventmemberdescriptor/EventMemberDescriptor_EventWrapper08.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EventMemberDescriptor.EventWrapper08"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventMemberDescriptor_EventWrapper08 {}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
#[::unity2::methods]
impl EventMemberDescriptor_EventWrapper08 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 8)]
    pub fn invoke(
        self,
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
        o3: crate::system::object::Object,
        o4: crate::system::object::Object,
        o5: crate::system::object::Object,
        o6: crate::system::object::Object,
        o7: crate::system::object::Object,
        o8: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-eventmemberdescriptor")]
impl EventMemberDescriptor_EventWrapper08 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventMemberDescriptor_EventWrapper08),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMemberDescriptor_EventWrapper08Methods>::ctor(this, object, method);
        this
    }
}
