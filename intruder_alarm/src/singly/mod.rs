//! An intrusive linked list implementation using `RawLink`s modified as singly.
//!
//! An _intrusive_ list is a list structure wherein the type of element stored
//! in the list holds references to other nodes. This means that we don't have
//! to store a separate node data type that holds the stored elements and
//! pointers to other nodes, reducing the amount of memory allocated. We can
//! use intrusive lists in code that runs without the kernel memory allocator,
//! like the allocator implementation itself, since each list element manages
//! its own memory.
use super::{Link, OwningRef};
use core::marker::PhantomData;
use core::mem;
use core::ops::DerefMut;
#[cfg(test)]
mod tests;

//-----------------------------------------------------------------------------
// Public API types
//-----------------------------------------------------------------------------
//  List
/// An intrusive singly-linked list.
///
/// This type is a wrapper around a series of [`Node`]s. It stores [`Link`]s
/// to the head and tail [`Node`]s and the length of the list.
///
/// # Type parameters
/// - `T`: the type of the items stored by each `N`
/// - `N`: the type of nodes in the list
/// - `R`: the type of [`OwningRef`] that owns each `N`.
///
/// [`Node`]: trait.Node.html
/// [`Link`]: ../struct.Link.html
/// [`OwningRef]: ../trait.OwningRef.html
#[derive(Default)]
pub struct List<T, N, R> {
    /// Link to the head node of the list.
    head: Link<N>,

    /// Length of the list.
    len: usize,

    /// Type marker for items stored in the list.
    _elem_ty: PhantomData<T>,

    /// Type marker for the `OwningRef` type.
    _ref_ty: PhantomData<R>,
}

//  Linked
/// Trait that must be implemented in order to be a member of an intrusive
/// linked list.
pub trait Linked: Sized // + Drop
{
    /// Borrow this element's [`Link`].
    ///
    /// [`Links`]: struct.Links.html
    fn links(&self) -> &Link<Self>;

    /// Mutably borrow this element's [`Links`].
    ///
    /// [`Links`]: struct.Links.html
    fn links_mut(&mut self) -> &mut Link<Self>;

    /// De-link this node, returning its' Links.
    fn take_links(&mut self) -> Link<Self> {
        mem::replace(self.links_mut(), Link::none())
    }

    /// Borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    fn next(&self) -> Option<&Self> {
        self.links().as_ref()
    }

    /// Mutably borrow the `next` element in the list, or `None` if this is the
    /// last.
    #[inline]
    fn next_mut(&mut self) -> Option<&mut Self> {
        self.links_mut().as_mut()
    }

    /// Borrow the `next` linked element, or `None` if this is the last.
    #[inline]
    fn peek_next<T>(&self) -> Option<&T>
    where
        Self: AsRef<T>,
    {
        self.next().map(Self::as_ref)
    }

    /// Mutably borrow the `next` linked element, or `None` if this is the
    /// last.
    #[inline]
    fn peek_next_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsMut<T>,
    {
        self.next_mut().map(Self::as_mut)
    }
}

//-----------------------------------------------------------------------------
// Implementations
//-----------------------------------------------------------------------------

// ===== impl List =====

impl<T, Node, R> List<T, Node, R> {
    /// Create a new `List` with 0 elements.
    pub const fn new() -> Self {
        List {
            head: Link::none(),
            len: 0,
            _elem_ty: PhantomData,
            _ref_ty: PhantomData,
        }
    }

    /// Returns the length of the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the list is empty, false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows the first node of the list as an `Option`.
    ///
    /// Note that this is distinct from `front`: this method
    /// borrows the head _node_, not the head _element_.
    ///
    /// # Returns
    ///   - `Some(&N)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn head(&self) -> Option<&Node> {
        self.head.as_ref()
    }

    /// Mutably borrows the first node of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut Node)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn head_mut(&mut self) -> Option<&mut Node> {
        self.head.as_mut()
    }
}

impl<T, Node, Ref> List<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
    Ref: DerefMut,
{
    /// Push a node to the head of the list.
    pub fn push_front_node(&mut self, mut node: Ref) -> &mut Self {
        unsafe {
            /*
                Link is also a struct, with Optional interface
                What is this `node`?
            */
            *node.links_mut() = self.head;

            let node = Link::from_owning_ref(node);

            //TODO: What about this ()?
            match self.head.0 {
                None => (),  //FIXME
                Some(mut head) => ()  //FIXME
            }

            self.head = node;
            self.len += 1;
        };
        self
    }
}

impl<T, Node, Ref> List<T, Node, Ref>
where
    Node: Linked,
    Ref: OwningRef<Node>,
{
    /// Pop a node from the front of the list.
    pub fn pop_front_node(&mut self) -> Option<Ref> {
        unsafe {
            self.head.as_ptr().map(|node| {
                self.head = (*node).take_links();

                match self.head.as_mut() {
                    None => (),
                    Some(head) => ()
                }

                self.len -= 1;
                Ref::from_ptr(node as *const Node)
            })
        }
    }
}

impl<T, Node, R> List<T, Node, R>
where
    Node: AsRef<T>,
{
    /// Borrows the first item of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.head().map(Node::as_ref)
    }
}

impl<T, Node, R> List<T, Node, R>
where
    Node: AsMut<T>,
{
    /// Mutably borrows the first element of the list as an `Option`
    ///
    /// # Returns
    ///   - `Some(&mut T)` if the list has elements
    ///   - `None` if the list is empty.
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head_mut().map(Node::as_mut)
    }
}

#[cfg(all(
    feature = "alloc",
    not(any(feature = "std", test))
))]
use alloc::boxed::Box;
#[cfg(any(feature = "std", test))]
use std::boxed::Box;


#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> List<T, Node, Box<Node>>
where
    Node: From<T>,
    Node: Linked,
{
    /// Push an item to the front of the list.
    #[inline]
    pub fn push_front(&mut self, item: T) -> &mut Self {
        self.push_front_node(Box::new(Node::from(item)))
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
impl<T, Node> List<T, Node, Box<Node>>
where
    Node: Linked,
    Node: Into<T>,
{
    /// Pop an item from the front of the list.
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|b| (*b).into())
    }
}