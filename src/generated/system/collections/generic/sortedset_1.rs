
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/sortedset_1/SortedSet_1_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SortedSet_1_Enumerator<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for SortedSet_1_Enumerator<T0> {
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "SortedSet`1.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for SortedSet_1_Enumerator<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-sortedset_1")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity> SortedSet_1_Enumerator<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, set: crate::system::collections::generic::sortedset_1::SortedSet_1<T0>)
        -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        set: crate::system::collections::generic::sortedset_1::SortedSet_1<T0>,
        reverse: bool,
    ) -> ();

    #[method(
        name = "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
        args = 1
    )]
    pub fn system_runtime_serialization_i_deserialization_callback_on_deserialization(
        self,
        sender: crate::system::object::Object,
    ) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> T0;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "get_NotStartedOrEnded", args = 0)]
    pub fn get_not_started_or_ended(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/sortedset_1/SortedSet_1_Node.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "SortedSet`1.Node")]
pub struct SortedSet_1_Node<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-sortedset_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SortedSet_1_Node<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        item: T0,
        color: crate::system::collections::generic::nodecolor::NodeColor,
    ) -> ();

    #[method(name = "IsNonNullRed", args = 1)]
    pub fn is_non_null_red(
        node: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> bool;

    #[method(name = "IsNullOrBlack", args = 1)]
    pub fn is_null_or_black(
        node: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> bool;

    #[method(name = "get_Item", args = 0)]
    pub fn get_item(self) -> T0;

    #[method(name = "set_Item", args = 1)]
    pub fn set_item(self, value: T0) -> ();

    #[method(name = "get_Left", args = 0)]
    pub fn get_left(self)
        -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "set_Left", args = 1)]
    pub fn set_left(
        self,
        value: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();

    #[method(name = "get_Right", args = 0)]
    pub fn get_right(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "set_Right", args = 1)]
    pub fn set_right(
        self,
        value: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::system::collections::generic::nodecolor::NodeColor;

    #[method(name = "set_Color", args = 1)]
    pub fn set_color(self, value: crate::system::collections::generic::nodecolor::NodeColor) -> ();

    #[method(name = "get_IsBlack", args = 0)]
    pub fn get_is_black(self) -> bool;

    #[method(name = "get_IsRed", args = 0)]
    pub fn get_is_red(self) -> bool;

    #[method(name = "get_Is2Node", args = 0)]
    pub fn get_is2_node(self) -> bool;

    #[method(name = "get_Is4Node", args = 0)]
    pub fn get_is4_node(self) -> bool;

    #[method(name = "ColorBlack", args = 0)]
    pub fn color_black(self) -> ();

    #[method(name = "ColorRed", args = 0)]
    pub fn color_red(self) -> ();

    #[method(name = "GetRotation", args = 2)]
    pub fn get_rotation(
        self,
        current: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        sibling: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> crate::system::collections::generic::treerotation::TreeRotation;

    #[method(name = "GetSibling", args = 1)]
    pub fn get_sibling(
        self,
        node: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "Split4Node", args = 0)]
    pub fn split4_node(self) -> ();

    #[method(name = "Rotate", args = 1)]
    pub fn rotate(
        self,
        rotation: crate::system::collections::generic::treerotation::TreeRotation,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "RotateLeft", args = 0)]
    pub fn rotate_left(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "RotateLeftRight", args = 0)]
    pub fn rotate_left_right(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "RotateRight", args = 0)]
    pub fn rotate_right(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "RotateRightLeft", args = 0)]
    pub fn rotate_right_left(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "Merge2Nodes", args = 0)]
    pub fn merge2_nodes(self) -> ();

    #[method(name = "ReplaceChild", args = 2)]
    pub fn replace_child(
        self,
        child: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        new_child: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();
}

#[cfg(feature = "system-collections-generic-sortedset_1")]
impl<T0: ::unity2::ClassIdentity> SortedSet_1_Node<T0> {
    pub fn new(item: T0, color: crate::system::collections::generic::nodecolor::NodeColor) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedSet_1_Node),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedSet_1_NodeMethods<T0>>::ctor(this, item, color);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/sortedset_1/SortedSet_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "SortedSet`1")]
pub struct SortedSet_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "root")]
    pub root: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    #[rename(name = "comparer")]
    pub comparer:
        crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<T0>,
    #[rename(name = "count")]
    pub count: i32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-generic-sortedset_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SortedSet_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "InOrderTreeWalk", args = 1)]
    pub fn in_order_tree_walk(
        self,
        action: crate::system::collections::generic::treewalkpredicate_1::TreeWalkPredicate_1<T0>,
    ) -> bool;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "VersionCheck", args = 0)]
    pub fn version_check(self) -> ();

    #[method(name = "IsWithinRange", args = 1)]
    pub fn is_within_range(self, item: T0) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> bool;

    #[method(name = "AddIfNotPresent", args = 1)]
    pub fn add_if_not_present(self, item: T0) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;

    #[method(name = "DoRemove", args = 1)]
    pub fn do_remove(self, item: T0) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, index: i32) -> ();

    #[method(name = "CopyTo", args = 3)]
    pub fn copy_to_2(self, array: ::unity2::Array<T0>, index: i32, count: i32) -> ();

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Enumerator<T0>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "InsertionBalance", args = 4)]
    pub fn insertion_balance(
        self,
        current: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        parent: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        grand_parent: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        great_grand_parent: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();

    #[method(name = "ReplaceChildOrRoot", args = 3)]
    pub fn replace_child_or_root(
        self,
        parent: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        child: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        new_child: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();

    #[method(name = "ReplaceNode", args = 4)]
    pub fn replace_node(
        self,
        r#match: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        parent_of_match: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        successor: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
        parent_of_successor: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> ();

    #[method(name = "FindNode", args = 1)]
    pub fn find_node(
        self,
        item: T0,
    ) -> crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>;

    #[method(name = "UpdateVersion", args = 0)]
    pub fn update_version(self) -> ();

    #[method(
        name = "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
        args = 1
    )]
    pub fn system_runtime_serialization_i_deserialization_callback_on_deserialization(
        self,
        sender: crate::system::object::Object,
    ) -> ();

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Log2", args = 1)]
    pub fn log2(value: i32) -> i32;
}

#[cfg(feature = "system-collections-generic-sortedset_1")]
impl<T0: ::unity2::ClassIdentity> SortedSet_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedSet_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedSet_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedSet_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISortedSet_1Methods<T0>>::ctor_2(this, comparer);
        this
    }
}
