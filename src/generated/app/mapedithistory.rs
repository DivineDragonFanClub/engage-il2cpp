
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory_ReplaceObjectCommand.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory.ReplaceObjectCommand")]
#[parent(crate::app::mapedithistory::MapEditHistory_MapEditCommand)]
pub struct MapEditHistory_ReplaceObjectCommand {
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_DeleteRotate")]
    pub m_delete_rotate: i32,
    #[rename(name = "m_DeleteObjName")]
    pub m_delete_obj_name: ::unity2::Il2CppString,
    #[rename(name = "m_CreateObjName")]
    pub m_create_obj_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory_ReplaceObjectCommand {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        x: i32,
        z: i32,
        delete_rotate: i32,
        delete_obj_name: ::unity2::Il2CppString,
        create_obj_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Do", args = 0)]
    pub fn r#do(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory_ReplaceObjectCommand {
    pub fn new(
        x: i32,
        z: i32,
        delete_rotate: i32,
        delete_obj_name: ::unity2::Il2CppString,
        create_obj_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory_ReplaceObjectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistory_ReplaceObjectCommandMethods>::ctor(
            this,
            x,
            z,
            delete_rotate,
            delete_obj_name,
            create_obj_name,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory_CreateObjectCommand.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory.CreateObjectCommand")]
#[parent(crate::app::mapedithistory::MapEditHistory_MapEditCommand)]
pub struct MapEditHistory_CreateObjectCommand {
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_ObjectName")]
    pub m_object_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory_CreateObjectCommand {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, x: i32, z: i32, object_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Do", args = 0)]
    pub fn r#do(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory_CreateObjectCommand {
    pub fn new(x: i32, z: i32, object_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory_CreateObjectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistory_CreateObjectCommandMethods>::ctor(this, x, z, object_name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory_DeleteObjectCommand.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory.DeleteObjectCommand")]
#[parent(crate::app::mapedithistory::MapEditHistory_MapEditCommand)]
pub struct MapEditHistory_DeleteObjectCommand {
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_DeleteObjectName")]
    pub m_delete_object_name: ::unity2::Il2CppString,
    #[rename(name = "m_Rotate")]
    pub m_rotate: i32,
}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory_DeleteObjectCommand {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        x: i32,
        z: i32,
        delete_object_name: ::unity2::Il2CppString,
        rotate: i32,
    ) -> ();

    #[method(name = "Do", args = 0)]
    pub fn r#do(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory_DeleteObjectCommand {
    pub fn new(x: i32, z: i32, delete_object_name: ::unity2::Il2CppString, rotate: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory_DeleteObjectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistory_DeleteObjectCommandMethods>::ctor(
            this,
            x,
            z,
            delete_object_name,
            rotate,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory_RotateObjectCommand.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory.RotateObjectCommand")]
#[parent(crate::app::mapedithistory::MapEditHistory_MapEditCommand)]
pub struct MapEditHistory_RotateObjectCommand {
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_PrevRotate")]
    pub m_prev_rotate: i32,
    #[rename(name = "m_NextRotate")]
    pub m_next_rotate: i32,
}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory_RotateObjectCommand {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x: i32, z: i32, prev_rotate: i32, next_rotate: i32) -> ();

    #[method(name = "Do", args = 0)]
    pub fn r#do(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory_RotateObjectCommand {
    pub fn new(x: i32, z: i32, prev_rotate: i32, next_rotate: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory_RotateObjectCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistory_RotateObjectCommandMethods>::ctor(
            this,
            x,
            z,
            prev_rotate,
            next_rotate,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapedithistory :: MapEditHistory >)]
pub struct MapEditHistory {
    #[static_field]
    #[rename(name = "MaxCommandCount")]
    pub max_command_count: i32,
    #[rename(name = "m_CommandList")]
    pub m_command_list: crate::system::collections::generic::list_1::List_1<
        crate::app::mapedithistory::MapEditHistory_MapEditCommand,
    >,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> bool;

    #[method(name = "Redo", args = 0)]
    pub fn redo(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "AddCreateObject", args = 3)]
    pub fn add_create_object(self, x: i32, z: i32, object_name: ::unity2::Il2CppString) -> ();

    #[method(name = "AddDeleteObject", args = 4)]
    pub fn add_delete_object(
        self,
        x: i32,
        z: i32,
        delete_object_name: ::unity2::Il2CppString,
        rotate: i32,
    ) -> ();

    #[method(name = "AddReplaceObject", args = 5)]
    pub fn add_replace_object(
        self,
        x: i32,
        z: i32,
        delete_rotate: i32,
        delete_obj_name: ::unity2::Il2CppString,
        create_obj_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddRotateObject", args = 4)]
    pub fn add_rotate_object(self, x: i32, z: i32, prev_rotate: i32, next_rotate: i32) -> ();

    #[method(name = "AddCommand", args = 1)]
    pub fn add_command(
        self,
        command: crate::app::mapedithistory::MapEditHistory_MapEditCommand,
    ) -> ();

    #[method(name = "ListAddImpl", args = 1)]
    pub fn list_add_impl(
        self,
        command: crate::app::mapedithistory::MapEditHistory_MapEditCommand,
    ) -> ();

    #[method(name = "ListRemoveAtImpl", args = 1)]
    pub fn list_remove_at_impl(self, index: i32) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistoryMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapedithistory/MapEditHistory_MapEditCommand.md")))]
#[::unity2::class(namespace = "App", name = "MapEditHistory.MapEditCommand")]
#[parent(crate::system::object::Object)]
pub struct MapEditHistory_MapEditCommand {}

#[cfg(feature = "app-mapedithistory")]
#[::unity2::methods]
impl MapEditHistory_MapEditCommand {
    #[method(name = "Do", args = 0)]
    pub fn r#do(self) -> ();

    #[method(name = "Undo", args = 0)]
    pub fn undo(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapedithistory")]
impl MapEditHistory_MapEditCommand {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditHistory_MapEditCommand),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditHistory_MapEditCommandMethods>::ctor(this);
        this
    }
}
