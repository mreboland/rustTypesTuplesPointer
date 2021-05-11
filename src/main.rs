fn main() {
    println!("Hello, world!");
}


// Tuples

// A tuple is a pair, or triple, or quad, ... of values of assorted types. You can write a tuple as a sequence of elements, separated by commas and surrounded by parentheses. I.E ("Brazil", 1985) is a tuple whose first element is a statically allocated string, second an integer. It's type is (&str, i32), the second type can also be what Rust infers. Given tuple t, we can access its elements as t.0, t.1 and so on.

// Tuples differ to arrays as each element of a tuple can have a different type. Array's elements must be all the same type. Tuples also only allow constants as indices, like t.4. We can't write t.i or t[i] to get the i'th element.

// Rust code often uses tuple types to return multiple values from a function. Ex, a method on string slices which divides a string into two halves and returns them both:
fn split_at(&self, mid: usize) -> (&str, &str);
// The return type (&str, &str) is a tuple of two string slices. We can use pattern-matching syntax to assign each element of the return value to a different variable.
let text = "I see the eigenvalue in thine eye";
let (head, tail) = text.split_at(21);
assert_eq!(head, "i see the eigenvalue ");
assert_eq!(tail, "in thine eye");

// The above is more legible than:
let text = "I see the eigenvalue in thine eye";
let temp = text.split_at(21);
let head = temp.0;
let tail = temp.1;
assert_eq!(head, "I see the eigenvalue ");
assert_eq!(tail, "in thine eye");

// The other commonly used tuple type is the zero-tuple (). This is traditionally called the unit type because it has only one value, also written (). Rust uses the unit type where there's no meaningful value to carry, but context requires some sort of type nonetheless.

// For example, a function that returns no value has a return type of (). The standard library's std::mem::swap function has no meaningful return value; it just exchanges the values of its two arguments. The declaration for std::mem::swap reads:
fn swap<T>(x: &mut T, y: &mut T);
// The <T> means that swap is generic. We can use it on references to values of any type T. But the signature omits the swap's return type altogether, which is shorthand for returning the unit type:
fn swap<T>(x: &mut T, y: &mut T) -> ();

// Rust consistently permits an extra trailing comma everywhere commas are used (function arguments, arrays, struct, and enums, etc).
// There are even tuples that contain a single value. The literal ("lonely hearts",) is a tuple containing a single string, it's type is (&str,). Here, the comma after the value is necessary to distinguish the singleton tuple from a simple parenthetic expression.

// Pointer Types

// Rust has several types that represent memory addresses.
// Rust is as language designed to help keep allocations to a minimum. Values nest by default. The value ((0, 0), (1440, 900)) is stored as four adjacent integers. If we store it in a local variable, we've got a local variable four integers wide. Nothing is allocated in the heap.
// It's great for memory efficiency, however when Rust needs values to point to other values, it must use pointer types explicitly. Pointer types used in safe Rust are constrained to eliminate undefined behaviour, so pointers are much easier to use correctly in Rust compared to C++.

