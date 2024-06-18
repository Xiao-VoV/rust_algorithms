async fn do_something() {
    println!("this is an async function!"); /* ... */
}

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;
fn test_async() {
    let future = do_something(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
mod timer_future;
