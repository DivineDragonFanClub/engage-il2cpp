
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinspectors/MapInspectors_EventUnitScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapInspectors_EventUnitScope {}

impl ::unity2::ClassIdentity for MapInspectors_EventUnitScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapInspectors.EventUnitScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapInspectors_EventUnitScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapinspectors")]
#[::unity2::methods(value)]
impl MapInspectors_EventUnitScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinspectors/MapInspectors.md")))]
#[::unity2::class(namespace = "App", name = "MapInspectors")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapinspectors :: MapInspectors >)]
pub struct MapInspectors {
    #[rename(name = "m_Inspectors")]
    pub m_inspectors:
        crate::system::collections::generic::list_1::List_1<crate::app::mapinspector::MapInspector>,
    #[rename(name = "m_KindInspectors")]
    pub m_kind_inspectors: ::unity2::Array<
        crate::system::collections::generic::list_1::List_1<crate::app::mapinspector::MapInspector>,
    >,
    #[static_field]
    #[rename(name = "s_InspectorSearch")]
    pub s_inspector_search: crate::app::mapinspectors::MapInspectors_MapInspectorSearch,
}

#[cfg(feature = "app-mapinspectors")]
#[::unity2::methods]
impl MapInspectors {
    #[method(name = "get_Inspectors", args = 0)]
    pub fn get_inspectors(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::mapinspector::MapInspector>;

    #[method(name = "GetKindInspectors", args = 1)]
    pub fn get_kind_inspectors(
        kind: crate::app::mapinspector::MapInspector_Kind,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::mapinspector::MapInspector>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ClearImpl", args = 0)]
    pub fn clear_impl(self) -> ();

    #[method(name = "AddImpl", args = 1)]
    pub fn add_impl(self, inspector: crate::app::mapinspector::MapInspector) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(
        inspector: crate::app::mapinspector::MapInspector,
    ) -> crate::app::mapinspector::MapInspector;

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "GetEnableInspectors", args = 6)]
    pub fn get_enable_inspectors(
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    ) -> crate::system::collections::generic::stack_1::Stack_1<crate::app::mapinspector::MapInspector>;

    #[method(name = "GetEnableInspector", args = 6)]
    pub fn get_enable_inspector(
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    ) -> crate::app::mapinspector::MapInspector;

    #[method(name = "GetPokeInspector", args = 2)]
    pub fn get_poke_inspector(x: i32, z: i32) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "GetPokeInspector", args = 3)]
    pub fn get_poke_inspector_2(
        kind: crate::app::mapinspector::MapInspector_Kind,
        x: i32,
        z: i32,
    ) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "IsEnable", args = 3)]
    pub fn is_enable(kind: crate::app::mapinspector::MapInspector_Kind, x: i32, z: i32) -> bool;

    #[method(name = "IsEnable", args = 2)]
    pub fn is_enable_2(
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "IsEnable", args = 4)]
    pub fn is_enable_3(
        kind: crate::app::mapinspector::MapInspector_Kind,
        x: i32,
        z: i32,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "IsEnable", args = 3)]
    pub fn is_enable_4(
        kind: crate::app::mapinspector::MapInspector_Kind,
        from: crate::app::unit::Unit,
        to: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "FindBreakable", args = 2)]
    pub fn find_breakable(x: i32, z: i32) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "IsBreakable", args = 2)]
    pub fn is_breakable(x: i32, z: i32) -> bool;

    #[method(name = "FindCannon", args = 2)]
    pub fn find_cannon(x: i32, z: i32) -> crate::app::cannoninspector::CannonInspector;

    #[method(name = "TryCreateCannonInspector", args = 3)]
    pub fn try_create_cannon_inspector(
        x: i32,
        z: i32,
        max_shells: i32,
    ) -> crate::app::cannoninspector::CannonInspector;

    #[method(name = "CalcLayerSize", args = 4)]
    pub fn calc_layer_size(x: i32, z: i32, w: i32, h: i32) -> ();

    #[method(name = "Regist", args = 0)]
    pub fn regist() -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapinspectors")]
impl MapInspectors {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapInspectors),
                ::core::stringify!(new),
            )
        });
        <Self as IMapInspectorsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinspectors/MapInspectors_MapInspectorSearch.md")))]
#[::unity2::class(namespace = "App", name = "MapInspectors.MapInspectorSearch")]
#[parent(crate::system::object::Object)]
pub struct MapInspectors_MapInspectorSearch {
    #[rename(name = "m_List")]
    pub m_list: crate::app::rawclasslist_1::RawClassList_1<crate::app::mapinspector::MapInspector>,
}

#[cfg(feature = "app-mapinspectors")]
#[::unity2::methods]
impl MapInspectors_MapInspectorSearch {
    #[method(name = "TryUpdateEnable", args = 6)]
    pub fn try_update_enable(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        value1: i32,
        value2: i32,
        value3: i32,
        value4: i32,
    ) -> bool;

    #[method(name = "GetEnableStack", args = 6)]
    pub fn get_enable_stack(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        value1: i32,
        value2: i32,
        value3: i32,
        value4: i32,
    ) -> crate::system::collections::generic::stack_1::Stack_1<crate::app::mapinspector::MapInspector>;

    #[method(name = "GetEnableInspector", args = 6)]
    pub fn get_enable_inspector(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        unit: crate::app::unit::Unit,
        value1: i32,
        value2: i32,
        value3: i32,
        value4: i32,
    ) -> crate::app::mapinspector::MapInspector;

    #[method(name = "GetPokeInspector", args = 3)]
    pub fn get_poke_inspector(
        self,
        kind: crate::app::mapinspector::MapInspector_Kind,
        x: i32,
        z: i32,
    ) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapinspectors")]
impl MapInspectors_MapInspectorSearch {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapInspectors_MapInspectorSearch),
                ::core::stringify!(new),
            )
        });
        <Self as IMapInspectors_MapInspectorSearchMethods>::ctor(this);
        this
    }
}
