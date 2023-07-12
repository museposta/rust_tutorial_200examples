

// the most straightforward one is Box.   keeps in heap

// SMART POINTERS

// Box<T>:     The Box<T> is a smart pointer which points to the data allocated on the heap of type T where 'T' is the type of the data. It is used to store the data on the heap rather than on the stack.
// Deref<T>:   The Deref<T> is a smart pointer which is used to customize the behavior of the dereference operator(*).
// Drop<T>:    The Drop<T> is a smart pointer used to free the space from the heap memory when the variable goes out of the scope.
// Rc<T>:      The Rc<T> stands for reference counted pointer. It is a smart pointer which keeps a record of the number of references to a value stored on the heap.
// RefCell<T>: The RefCell<T> is a smart pointer which allows you to borrow the mutable data even if the data is immutable. This process is known as interior mutability.


fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
   }