fn main() {
    let i1: i32 = 5;
    let i2: i32 = i1;

    println!("i1: {i1}, i2: {i2}");

    let s1: String = String::from("Rust");
    let s2: String = s1;

    println!("s1: {s1}, s2: {s2}");
}

/*
 *
 * Ownership is Rust’s most unique feature.
 * It enables Rust to make memory safety guarantees without needing a garbage collector.
 *
 * * The Stack and the Heap
 * Both the stack and the heap are parts of memory available to your code to use at runtime.
 *
 * Stack    => All data stored on the stack must have a known, fixed size.
 *          => Adding data is called pushing onto the stack, and removing data is called popping off the stack.
 *          => last in, first out
 *
 * Heap     => Unknown size at compile time or a size that might change must be stored on the heap
 *          => when you put data on the heap, you request a certain amount of space.
 *             The memory allocator finds an empty spot in the heap that is big enough, marks it
 *             as being in use, and returns a pointer, which is the address of that location.
 *             This process is called allocating on the heap and is sometimes abbreviated as just allocating
 *          => The heap is less organized
 *          => Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack.
 *
 * Pointer  => address of that location
 *
 * Pushing to the stack is faster than allocating on the heap because the allocator never has to search
 * for a place to store new data; that location is always at the top of the stack. Comparatively, allocating
 * space on the heap requires more work because the allocator must first find a big enough space to hold
 * the data and then perform bookkeeping to prepare for the next allocation.
 *
 * Accessing data in the heap is slower than accessing data on the stack because you have to follow
 * a pointer to get there.
 *
 *
 * 
 * * Ownership Rules:
 * Each value in Rust has an owner.
 * There can only be one owner at a time.
 * When the owner goes out of scope, the value will be dropped.
 * 
 * 
 * 
 * 
*/
