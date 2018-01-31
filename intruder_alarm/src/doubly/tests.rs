//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//

use super::*;
use super::Linked;
use quickcheck::TestResult;
use std::default::Default;

#[derive(Default, Debug)]
pub struct NumberedNode {
    pub number: usize,
    links: Links<NumberedNode>,
}

impl NumberedNode {
    pub fn new(number: usize) -> Self {
        NumberedNode {
            number: number,
            ..Default::default()
        }
    }
}

impl Linked for NumberedNode {
    #[inline]
    fn links(&self) -> &Links<Self> {
        &self.links
    }

    #[inline]
    fn links_mut(&mut self) -> &mut Links<Self> {
        &mut self.links
    }
}

impl AsRef<usize> for NumberedNode {
    fn as_ref(&self) -> &usize {
        &self.number
    }
}

impl AsMut<usize> for NumberedNode {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.number
    }
}

impl PartialEq for NumberedNode {
    fn eq(&self, rhs: &Self) -> bool {
        self.number == rhs.number
    }
}

impl From<usize> for NumberedNode {
    fn from(u: usize) -> NumberedNode {
        NumberedNode::new(u)
    }
}

impl Into<usize> for NumberedNode {
    fn into(self) -> usize {
        self.number
    }
}

mod boxed {
    use super::*;
    use std::boxed::Box;

    pub type NumberedList = List<usize, NumberedNode, Box<NumberedNode>>;


    mod push_node {
        use super::*;
        use std::boxed::Box;

        #[test]
        fn not_empty_after_first_push() {
            let mut list = NumberedList::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front_node(Box::new(NumberedNode::new(1)));

            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contents_after_first_push() {
            let mut list = NumberedList::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            list.push_front_node(Box::new(NumberedNode::new(555)));

            assert_eq!(list.tail().unwrap().number, 555);
            assert_eq!(list.head().unwrap().number, 555);
        }

        #[test]
        fn head_tail_equal_after_first_push() {
            let mut list = NumberedList::new();
            assert_eq!(list.head(), list.tail());

            list.push_front_node(Box::new(NumberedNode::new(444)));

            assert_eq!(list.head(), list.tail());
        }

        #[test]
        fn head_tail_not_equal_after_second_push() {
            let mut list = NumberedList::new();

            list.push_front_node(Box::new(NumberedNode::new(444)));
            list.push_front_node(Box::new(NumberedNode::new(555)));

            assert!(list.head().unwrap() != list.tail().unwrap());
        }
    }

    #[test]
    fn head_tail_not_same_second_push() {
        let mut list = NumberedList::new();
        let a = 444;
        let b = 555;

        list.push_front(a);
        list.push_front(b);

        assert!(list.head().unwrap() != list.tail().unwrap());
    }

    quickcheck! {
        fn push_front_node_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = NumberedList::new();
            list.push_front_node(Box::new(NumberedNode::new(x)));
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_front_node(Box::new(NumberedNode::new(x_2)));
                result = TestResult::from_bool(
                    list.tail().unwrap().number == x &&
                    list.head().unwrap().number == x_2
                );
            }
            result
        }

        fn not_empty_after_push(n: usize) -> bool {
            let mut list = NumberedList::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front(n);

            !list.is_empty() && list.len() == 1
        }

        fn contents_after_first_push(n: usize) -> bool {
            let mut list = NumberedList::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            list.push_front(n);
            list.tail().unwrap().number == n &&
            list.head().unwrap().number == n
        }

        fn linked_peek_prev_next(a: usize, b: usize) -> bool {
            let mut list = NumberedList::new();

            list.push_back(a);
            list.push_back(b);

            list.head().unwrap().peek_prev() == None
            && list.head().unwrap().peek_next() == Some(&b)
            && list.tail().unwrap().peek_prev() == Some(&a)
            && list.tail().unwrap().peek_next() == None
        }

        fn extend_sum_len(ys: Vec<usize>, xs: Vec<usize>) -> bool {
            let mut list = NumberedList::new();
            let total = ys.len() + xs.len();
            for y in ys {
                list.push_back(y);
            }
            list.extend(xs);

            list.len() == total
        }

        fn from_iter_len(xs: Vec<usize>) -> bool {
            let lx = xs.len();
            let list = NumberedList::from_iter(xs);

            list.len() == lx
        }

        fn collect_from_iter_equivalent(xs: Vec<usize>) -> bool {
            let mut list1 = NumberedList::from_iter(xs.clone());
            let mut list2 = xs.clone().into_iter().collect::<NumberedList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_front() == list2.pop_front()
                );
            }
            result
        }

        fn collect_and_loop_push_equivalent(xs: Vec<usize>) -> bool {
            let mut list1 = NumberedList::new();

            for x in xs.clone() {
                list1.push_back(x);
            }

            let mut list2 = xs.clone().into_iter().collect::<NumberedList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_front() == list2.pop_front()
                );
            }
            result
        }
    }

    #[test]
    fn contents_after_push_nodes() {
        let mut list = NumberedList::new();

        list.push_front_node(Box::new(NumberedNode::new(0)));
        list.push_front_node(Box::new(NumberedNode::new(1)));

        assert_eq!(list.tail().unwrap().number, 0);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(Box::new(NumberedNode::new(2)));
        assert_eq!(list.tail().unwrap().number, 2);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);
        assert_eq!(list.head().unwrap().number, 1);

        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_front_node() {
        let mut list = NumberedList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(Box::new(NumberedNode::new(1)));
        list.push_front_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(Box::new(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_front_node().unwrap().number, 0);
        assert_eq!(list.pop_front_node().unwrap().number, 1);
        assert_eq!(list.pop_front_node().unwrap().number, 2);
        assert_eq!(list.pop_front_node().unwrap().number, 3);
        assert_eq!(list.pop_front_node().unwrap().number, 4);

        assert!(list.is_empty());
        assert_eq!(list.pop_front_node(), None);
    }

    #[test]
    fn test_pop_back_node() {
        let mut list = NumberedList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(Box::new(NumberedNode::new(1)));
        list.push_front_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(Box::new(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_back_node().unwrap().number, 4);
        assert_eq!(list.pop_back_node().unwrap().number, 3);
        assert_eq!(list.pop_back_node().unwrap().number, 2);
        assert_eq!(list.pop_back_node().unwrap().number, 1);
        assert_eq!(list.pop_back_node().unwrap().number, 0);

        assert!(list.is_empty());
        assert_eq!(list.pop_back_node(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list = NumberedList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(Box::new(NumberedNode::new(1)));
        list.push_front_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(Box::new(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_front().unwrap(), 0);
        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.pop_front().unwrap(), 4);

        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = NumberedList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(Box::new(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(Box::new(NumberedNode::new(1)));
        list.push_front_node(Box::new(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(Box::new(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(Box::new(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_back().unwrap(), 4);
        assert_eq!(list.pop_back().unwrap(), 3);
        assert_eq!(list.pop_back().unwrap(), 2);
        assert_eq!(list.pop_back().unwrap(), 1);
        assert_eq!(list.pop_back().unwrap(), 0);

        assert!(list.is_empty());
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_extend() {
        let mut list = NumberedList::new();

        list.push_back(0);
        list.push_back(1);

        assert_eq!(list.tail().unwrap().number, 1);
        assert_eq!(list.head().unwrap().number, 0);

        let ext = vec![3, 4];
        list.extend(ext);

        assert_eq!(list.tail().unwrap().number, 4);
        assert_eq!(list.head().unwrap().number, 0);
    }

    #[test]
    fn test_fromiter() {
        let list_a = (0..10).into_iter();
        let mut nlist = NumberedList::from_iter(list_a);

        for i in 0..10 {
            assert_eq!(nlist.pop_front().unwrap(), i);
        }
    }
}



mod unsafe_ref {
    use super::*;
    use UnsafeRef;

    pub type UnsafeList = List<usize, NumberedNode, UnsafeRef<NumberedNode>>;


    mod push_node {
        use super::*;
        use std::boxed::Box;
        use UnsafeRef;

        #[test]
        fn not_empty_after_first_push() {
            let mut list = UnsafeList::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));

            assert_eq!(list.is_empty(), false);
            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contents_after_first_push() {
            let mut list = UnsafeList::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(555)));

            assert_eq!(list.tail().unwrap().number, 555);
            assert_eq!(list.head().unwrap().number, 555);
        }

        #[test]
        fn head_tail_equal_after_first_push() {
            let mut list = UnsafeList::new();
            assert_eq!(list.head(), list.tail());

            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(444)));

            assert_eq!(list.head(), list.tail());
        }

        #[test]
        fn head_tail_not_equal_after_second_push() {
            let mut list = UnsafeList::new();

            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(444)));
            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(555)));

            assert!(list.head().unwrap() != list.tail().unwrap());
        }
    }

    // #[test]
    // fn head_tail_not_same_second_push() {
    //     let mut list = UnsafeList::new();
    //     let a = 444;
    //     let b = 555;

    //     list.push_front(a);
    //     list.push_front(b);

    //     assert!(list.head().unwrap() != list.tail().unwrap());
    // }

    quickcheck! {
        fn push_front_node_order(x: usize, xs: Vec<usize>) -> TestResult {
            let mut list = UnsafeList::new();
            list.push_front_node(UnsafeRef::boxed(NumberedNode::new(x)));
            let mut result = TestResult::passed();
            for x_2 in xs {
                list.push_front_node(UnsafeRef::boxed(NumberedNode::new(x_2)));
                result = TestResult::from_bool(
                    list.tail().unwrap().number == x &&
                    list.head().unwrap().number == x_2
                );
            }
            result
        }

        fn not_empty_after_push(n: usize) -> bool {
            let mut list = UnsafeList::new();

            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);

            assert!(list.is_empty());
            assert_eq!(list.len(), 0);

            list.push_front_node(UnsafeRef::boxed(NumberedNode::from(n)));

            !list.is_empty() && list.len() == 1
        }

        fn contents_after_first_push(n: usize) -> bool {
            let mut list = UnsafeList::new();
            assert_eq!(list.head(), None);
            assert_eq!(list.tail(), None);
            list.push_front_node(UnsafeRef::boxed(NumberedNode::from(n)));
            list.tail().unwrap().number == n &&
            list.head().unwrap().number == n
        }

        fn linked_peek_prev_next(a: usize, b: usize) -> bool {
            let mut list = UnsafeList::new();

            list.push_front_node(UnsafeRef::boxed(NumberedNode::from(a)));
            list.push_back_node(UnsafeRef::boxed(NumberedNode::from(b)));

            list.head().unwrap().peek_prev() == None
            && list.head().unwrap().peek_next() == Some(&b)
            && list.tail().unwrap().peek_prev() == Some(&a)
            && list.tail().unwrap().peek_next() == None
        }

        fn extend_sum_len(ys: Vec<usize>, xs: Vec<usize>) -> bool {
            let mut list = UnsafeList::new();
            let total = ys.len() + xs.len();
            let ys = ys.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs = xs.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            for y in ys {
                list.push_back_node(y);
            }

            list.extend(xs);

            list.len() == total
        }

        fn from_iter_len(xs: Vec<usize>) -> bool {
            let lx = xs.len();
            let xs = xs.into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let list = UnsafeList::from_iter(xs);

            list.len() == lx
        }

        fn collect_from_iter_equivalent(xs: Vec<usize>) -> bool {

            let xs1 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs2 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            let mut list1 = UnsafeList::from_iter(xs1);
            let mut list2 = xs2.into_iter().collect::<UnsafeList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_front_node() == list2.pop_front_node()
                );
            }
            result
        }

        fn collect_and_loop_push_equivalent(xs: Vec<usize>) -> bool {
            let xs1 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();
            let xs2 = xs.clone().into_iter()
                .map(|i| UnsafeRef::boxed(NumberedNode::from(i)))
                .collect::<Vec<_>>();

            let mut list1 = UnsafeList::new();

            for x in xs1 {
                list1.push_back(x);
            }

            let mut list2 = xs2.into_iter().collect::<UnsafeList>();

            let mut result = list1.len() == list2.len();
            for _ in 0..list1.len() {
                result = result && (
                    list1.pop_front_node().unwrap().number ==
                    list2.pop_front_node().unwrap().number
                );
            }
            result
        }
    }

    #[test]
    fn contents_after_push_nodes() {
        let mut list = UnsafeList::new();

        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(0)));
        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));

        assert_eq!(list.tail().unwrap().number, 0);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(2)));
        assert_eq!(list.tail().unwrap().number, 2);
        assert_eq!(list.head().unwrap().number, 1);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);
        assert_eq!(list.head().unwrap().number, 1);

        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_front_node() {
        let mut list = UnsafeList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));
        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_front_node().unwrap().number, 0);
        assert_eq!(list.pop_front_node().unwrap().number, 1);
        assert_eq!(list.pop_front_node().unwrap().number, 2);
        assert_eq!(list.pop_front_node().unwrap().number, 3);
        assert_eq!(list.pop_front_node().unwrap().number, 4);

        assert!(list.is_empty());
        assert_eq!(list.pop_front_node(), None);
    }

    #[test]
    fn test_pop_back_node() {
        let mut list = UnsafeList::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
        assert!(list.is_empty());

        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(2)));

        assert!(!list.is_empty());
        assert_eq!(list.head(), list.tail());

        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));
        list.push_front_node(UnsafeRef::boxed(NumberedNode::new(0)));

        assert_eq!(list.head().unwrap().number, 0);
        assert_eq!(list.tail().unwrap().number, 2);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(3)));
        assert_eq!(list.tail().unwrap().number, 3);

        list.push_back_node(UnsafeRef::boxed(NumberedNode::new(4)));
        assert_eq!(list.tail().unwrap().number, 4);

        assert!(!list.is_empty());

        assert_eq!(list.pop_back_node().unwrap().number, 4);
        assert_eq!(list.pop_back_node().unwrap().number, 3);
        assert_eq!(list.pop_back_node().unwrap().number, 2);
        assert_eq!(list.pop_back_node().unwrap().number, 1);
        assert_eq!(list.pop_back_node().unwrap().number, 0);

        assert!(list.is_empty());
        assert_eq!(list.pop_back_node(), None);
    }

    // #[test]
    // fn test_pop_front() {
    //     let mut list = UnsafeList::new();

    //     assert_eq!(list.head(), None);
    //     assert_eq!(list.tail(), None);
    //     assert!(list.is_empty());

    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(2)));

    //     assert!(!list.is_empty());
    //     assert_eq!(list.head(), list.tail());

    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));
    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(0)));

    //     assert_eq!(list.head().unwrap().number, 0);
    //     assert_eq!(list.tail().unwrap().number, 2);

    //     list.push_back_node(UnsafeRef::boxed(NumberedNode::new(3)));
    //     assert_eq!(list.tail().unwrap().number, 3);

    //     list.push_back_node(UnsafeRef::boxed(NumberedNode::new(4)));
    //     assert_eq!(list.tail().unwrap().number, 4);

    //     assert!(!list.is_empty());

    //     assert_eq!(list.pop_front().unwrap(), 0);
    //     assert_eq!(list.pop_front().unwrap(), 1);
    //     assert_eq!(list.pop_front().unwrap(), 2);
    //     assert_eq!(list.pop_front().unwrap(), 3);
    //     assert_eq!(list.pop_front().unwrap(), 4);

    //     assert!(list.is_empty());
    //     assert_eq!(list.pop_front(), None);
    // }

    // #[test]
    // fn test_pop_back() {
    //     let mut list = UnsafeList::new();

    //     assert_eq!(list.head(), None);
    //     assert_eq!(list.tail(), None);
    //     assert!(list.is_empty());

    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(2)));

    //     assert!(!list.is_empty());
    //     assert_eq!(list.head(), list.tail());

    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(1)));
    //     list.push_front_node(UnsafeRef::boxed(NumberedNode::new(0)));

    //     assert_eq!(list.head().unwrap().number, 0);
    //     assert_eq!(list.tail().unwrap().number, 2);

    //     list.push_back_node(UnsafeRef::boxed(NumberedNode::new(3)));
    //     assert_eq!(list.tail().unwrap().number, 3);

    //     list.push_back_node(UnsafeRef::boxed(NumberedNode::new(4)));
    //     assert_eq!(list.tail().unwrap().number, 4);

    //     assert!(!list.is_empty());

    //     assert_eq!(list.pop_back().unwrap(), 4);
    //     assert_eq!(list.pop_back().unwrap(), 3);
    //     assert_eq!(list.pop_back().unwrap(), 2);
    //     assert_eq!(list.pop_back().unwrap(), 1);
    //     assert_eq!(list.pop_back().unwrap(), 0);

    //     assert!(list.is_empty());
    //     assert_eq!(list.pop_back(), None);
    // }

    #[test]
    fn test_extend() {
        let mut list = UnsafeList::new();

        list.push_back_node(UnsafeRef::boxed(NumberedNode::from(0)));
        list.push_back_node(UnsafeRef::boxed(NumberedNode::from(1)));

        assert_eq!(list.tail().unwrap().number, 1);
        assert_eq!(list.head().unwrap().number, 0);

        let ext = vec![
            UnsafeRef::boxed(NumberedNode::from(3)),
            UnsafeRef::boxed(NumberedNode::from(4))
        ];
        list.extend(ext);

        assert_eq!(list.tail().unwrap().number, 4);
        assert_eq!(list.head().unwrap().number, 0);
    }

    #[test]
    fn test_fromiter() {
        let list_a = (0..10).into_iter()
            .map(|i| UnsafeRef::boxed(NumberedNode::from(i)));
        let mut nlist = UnsafeList::from_iter(list_a);

        for i in 0..10 {
            assert_eq!(nlist.pop_front_node().unwrap().number, i);
        }
    }
}