#[cfg(test)]
use crate::queue::Queue;

#[test]
fn can_enqueue() {
    let mut _queue = Queue::new();
    _queue = _queue.enqueue(1);
    _queue = _queue.enqueue(2);
}

#[test]
fn can_dequeue() {
    let array = [1, 2, 3, 4, 5];
    let mut queue = Queue::from_slice(&array);

    for i in array {
        let (value, q) = queue.dequeue().unwrap();
        queue = q;
        assert_eq!(value, i);
    }
}

#[test]
fn can_peek() {
    let array = [1, 2];
    let mut queue = Queue::from_slice(&array);

    assert_eq!(queue.peek(), Some(1));
    let (_, q) = queue.dequeue().unwrap();
    queue = q;

    assert_eq!(queue.peek(), Some(2));
    let (_, q) = queue.dequeue().unwrap();
    queue = q;

    assert_eq!(queue.peek(), None);
}
