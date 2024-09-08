use std::{borrow::Borrow, cell::RefCell, rc::Rc};

// pub struct IntoIter<T>(List<T>);

// impl<T> Iterator for IntoIter<T>{
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.head.take().and_then(|node| {
//             match Rc::try_unwrap(node){
//                 Ok(mut inner_node) => {
//                     self.0.head = inner_node.next.take();
//                     Some(inner_node.elem)
//                 }
//                 Err(err_node) =>{
//                     self.0.head = err_node.next.take();
//                     None
//                 }
//             }
//         })
//     }
// }

pub struct Iter<'a,T>{
    next: Option<&'a Node<T>>,
}

impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// pub struct IterMut<'a,T>{
//     next: Option<&'a mut Node<T>>,
// }
// impl<'a,T> Iterator for IterMut<'a,T>{
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next().take().and_then(|node|{
//             self.next = node.next.as_mut().and_then(|rc_node| Rc::get_mut(rc_node));
//             &mut node.elem 
//         })
//     }
// }

type Link<T> = Option<Rc<Node<T>>>;
// enum Link{
//     Empty,
//     More(Box<Node>),
// }
struct Node<T>{
    elem: T,
    next: Link<T>,
}

pub struct List<T>{
    head: Link<T>,
}

impl<T> List<T>{
    pub fn new() -> Self{
        List{head:None}
    }
    pub fn prepend(&self,elem:T) -> List<T>{
        List { 
            head: Some(Rc::new(Node { elem: elem, next: self.head.clone() }))
         }
    }
    pub fn tail(&self) ->List<T>{
        List { 
            head: self.head.as_ref().and_then(|node| {
                node.as_ref().next.clone()
            }) 
        }
    }
    pub fn head(&self) ->Option<&T>{
        self.head.as_ref().map(|node| {
            &node.as_ref().elem
        })
    }
    // #[deprecated]
    // pub fn into_iter(self) -> IntoIter<T>{
    //     IntoIter(self)
    // }
    pub fn iter<'a>(&'a self) -> Iter<'a,T> {
        Iter { next: self.head.as_deref().map(|node| node )}
    }
    // #[deprecated]
    // pub fn iter_mut(&mut self) ->IterMut<'_,T>{
    //     let mut next = None;
    //     if let Some(ref mut rc_node) = self.head{
    //         next = Rc::get_mut(rc_node);
    //     }
    //     IterMut{next}
    // }
}

impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link{
            match Rc::try_unwrap(node){
                Ok(mut rc_node) =>{
                    cur_link = rc_node.next.take();
                }
                Err(_) =>{
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
    
    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    // #[test]
    // fn into_iter() {
    //     let mut list = List::new();
    //     assert_eq!(list.head(), None);
    //     list = list.prepend(4).prepend(5).prepend(6);
    //     // let list2 = list.prepend(7).prepend(8).prepend(9);

    //     let mut iter = list.into_iter();
    //     assert_eq!(iter.next(), Some(6));
    //     assert_eq!(iter.next(), Some(5));
    //     assert_eq!(iter.next(), Some(4));
    //     assert_eq!(iter.next(), None);
    //     //try_unwrap只有在强引用计数为1的时候才是安全的，如果考虑两个链表共享相同节点的情况
    //     // let mut iter = list2.into_iter();
    //     // assert_eq!(iter.next(), Some(9));
    //     // assert_eq!(iter.next(), Some(8));
    //     // assert_eq!(iter.next(), Some(7));
    //     // assert_eq!(iter.next(), Some(6));
    //     // assert_eq!(iter.next(), Some(5));
    //     // assert_eq!(iter.next(), Some(4));
    //     // assert_eq!(iter.next(), None);
    // }
}