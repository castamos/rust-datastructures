
/// A simple stack implementation for generic types.
///
/// # Examples
///
/// ```
/// use rust_datastructures::Stack;
///
/// // Type may be omitted and will be infered:
/// //                       vvvvvv
/// let mut num_stack = Stack::<i8>::new();
/// num_stack.push(5);
///
/// assert_eq!(num_stack.pop(), Some(5));
/// assert_eq!(num_stack.pop(), None);
/// ```
///
pub struct Stack<T> {
    contents: Option<StackElements<T>>,
    //        ^^^^^^ `contents` will be `None` in case the stack is empty.
}

/*
 * `StackElements` 
 *
 * A recursive linear container type that has at least one element.
 * This is implemented in a way similar to the Lisp "Cons List", or
 * in Prolog pseudocode:
 *
 *      cons(Head, Tail) :- 
 *          first_list_element(Head),
 *          rest_of_the_list(Tail).
 *
 *      list(X1, X2, X3) :- cons(X1, cons(X2, cons(X3, nil))).
 *
 *  In this implementation, the "cons" function is encoded via the `StackElements`
 *  recursive type and the "nil" value is encoded as `None`.
 */
struct StackElements<T> {
    head: T,
    tail: Box< Option<StackElements<T>> >,
    //         ^^^^^^ `tail` will be `Box<None>` in case there is only one element.
    //    ^^^ `Box` is used to allow the recursion.
}


impl<T> Stack<T> {
    
    // Constructor. Returns an empty stack.
    pub fn new() -> Stack<T> {
        Stack { contents: None }
    }

    // Adds an element to the stack.
    pub fn push(&mut self, element: T) {
        self.contents = Some( StackElements {
            head : element,                          // New element added as the head.
            tail : Box::new( self.contents.take() ), // The tail is the previous list.
        });
    }

    // Removes and returns the last element pushed into the stack, if there are `Some()`;
    // otherwise, returns `None` if there are no more elements.
    pub fn pop(&mut self) -> Option<T> {
        if let Some(contents) = self.contents.take() {
            self.contents = *contents.tail;
            Some(contents.head)
        }
        else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_lifecycle() {
        let mut list = Stack::new();
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
        
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);

        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }
}
