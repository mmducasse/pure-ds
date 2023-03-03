#[cfg(test)]
use crate::list::List;

fn _list(slice: &[i32]) -> crate::list::List<i32> {
    crate::list::List::from_slice(slice)
}

#[test]
fn can_equate() {
    let mut a1 = List::empty();
    let mut a2 = List::empty();
    let mut b = List::empty();

    for i in 0..10 {
        a1 = a1.prepend(i);
        a2 = a2.prepend(i);
        b = b.prepend(0);
    }

    assert_eq!(a1, a1);
    assert_eq!(a1, a2);
    assert_ne!(a1, b);
    assert_ne!(a1, List::empty());
}

#[test]
fn can_get_head_and_tail() {
    let five = _list(&[1, 2, 3, 4, 5]);
    let five_tail = _list(&[2, 3, 4, 5]);

    let head = five.head().unwrap();
    assert_eq!(head, 1);
    let tail = five.tail().unwrap();
    assert_eq!(tail, five_tail);
}

#[test]
fn can_insert() {
    let missing_3 = _list(&[1, 2, 4, 5]);
    let all = missing_3.insert(3, 2).unwrap();
    let none = missing_3.insert(0, 99);

    assert_eq!(all, _list(&[1, 2, 3, 4, 5]));
    assert_eq!(none, None);
}

#[test]
fn can_reverse() {
    let five = _list(&[1, 2, 3, 4, 5]);
    let reversed = _list(&[5, 4, 3, 2, 1]);

    assert_eq!(five.reverse(), reversed);
}

// todo:
//  + prepend
//  + insert
//  concat
//  + head
//  + tail
//  split
//  + reverse
