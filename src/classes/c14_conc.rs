/// This module presents these topics:
///     threads
///     message-passing and channels
///     concurrency-related traits (Send, Sync)
///     phantom types

/// Material for this module:
/// https://doc.rust-lang.org/book/ch16-00-concurrency.html
/// https://doc.rust-lang.org/rust-by-example/generics/phantom.html

// goal: overview of general notions and paradigm shift:
// threads + communication

// Terminology first: concurrency VS parallelism (VS distribution)
// Concurrency is when two or more tasks can start, run, and complete in overlapping time periods.
//   It doesn't necessarily mean they'll ever both be running at the same instant.
//   For example, multitasking on a single-core machine.
// Parallelism is when tasks literally run at the same time, e.g., on a multicore processor.
//
// OR :
// Concurrency is two lines of customers ordering from a single cashier (lines take turns ordering);
// Parallelism is two lines of customers ordering from two cashiers (each line gets its own cashier)
//
// Distribution: tasks run on physically separated nodes

// In most current operating systems, a program code is run in a process, and the OS manages
//   multiple processes at once. Within your program, you can also have independent parts that
//   run simultaneously. The features that run these independent parts are called threads.
//
// Multi-threaded task can lead to the follwoing problems:
// - Race conditions, where threads are accessing data or resources in an inconsistent order
// - Deadlocks, where two threads are waiting for each other to finish using a resource
//     the other thread has, preventing both threads from continuing
// - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
//
// Rust std provides 1:1 threading, meaning one operating system thread per one language thread.
//   There are crates that provide M:N threading in rust, such as futures,
//   but we will focus on the theading methods in Rust std.

use std::thread;
use std::time::Duration;

pub fn thread_spawning() {
    // QUIZ: what is the type of the argument of spawn ?
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Q: what numbers do we expect to see printed?

    //
    // The threads will probably take turns, but that isn’t guaranteed:
    //   it depends on how your operating system schedules the threads.
    // In this run, the main thread printed first, even though the print statement
    //   from the spawned thread appears first in the code. And even though we told the
    //   spawned thread to print until i is 9, it only got to 5 before the main thread shut down.
    //
    // We can fix the problem of the spawned thread not getting to run,
    //   or not getting to run completely, by saving the return value of thread::spawn in a variable.
    //   The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that,
    //   when we call the join method on it, will wait for its thread to finish.
    //
    handle.join().unwrap();
}

pub fn thread_test() {
    let v = vec![1, 2, 3];

    // QUIZ: why do we need the move keyword here?
    //  how is v passed? immutable borrow/mutable borrow/ownership
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // oh no!
    handle.join().unwrap();

    // Rust infers how to capture v, and because println! only needs a reference to v,
    //   the closure tries to borrow v. However, there’s a problem: Rust can’t tell
    // how long the spawned thread will run, so it doesn’t know if the reference to v
    // will always be valid . So, we can use move to solve this problem. However,
    // this also means we cannot use v anymore after calling spawn!
}


/// message passing
///   https://go.dev/doc/effective_go#concurrency
/// Do not communicate by sharing memory; instead, share memory by communicating.

// sharing memory across threads can cause many issues,
// other languages have other means for threads communication:
//  pipes, signals, etc

// channels are the concurrency abstraction, much like functions are the sequential one

// We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
// In short, the way Rust’s standard library implements channels means a channel can have
//      multiple sending ends that produce values
//      but only one receiving end that consumes those values.
// The mpsc::channel function returns a tuple, the first element of which is the sending end
//   and the second element is the receiving end.

use std::sync::mpsc;

pub fn channel_example() {
    // (transmitter, receiver)
    let (tx, rx) = mpsc::channel();

    // clone the transmitter so that each child thread gets one
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("one rubber duck in river 1"),
            String::from("two rubber ducks in river 1"),
            String::from("three rubber ducks in river 1"),
            String::from("four rubber ducks in river 1"),
        ];
        for val in vals {
            tx.send(val).unwrap();  // send() returns a Result
            thread::sleep(Duration::from_secs(1));
        }
        // send takes the ownership of val
        // println!("{}",val); ERROR!
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("one rubber duck in river 2"),
            String::from("two rubber ducks in river 2"),
            String::from("three rubber ducks in river 2"),
            String::from("four rubber ducks in river 2"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // rx blocks the main thread! it implements the Iterator trait
    for received in rx {
        // the main thread listens on rx and receives ducks from threads 1 and 2
        println!("Got: {}", received);
    }
}

// Channels in any programming language are similar to single ownership,
//   because once you transfer a value down a channel,
//   you should no longer use that value.
// Shared memory concurrency is like multiple ownership:
//   multiple threads can access the same memory location at the same time.

// The secret is Mutex smart pointer. Mutex is an abbreviation for mutual exclusion,
//   as in, a mutex allows only one thread to access some data at any given time.
//   Like RefCell, Mutex provides interior mutability.

use std::sync::Mutex;

pub fn mutex_example() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // unlock

    println!("m = {:?}", m);
}

// As with many types, we create a Mutex<T> using the associated function new.
//   To access the data inside the mutex, we use the lock method to acquire the lock.
//   This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
// The call to lock would fail if another thread holding the lock panicked.
//   In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap
//   and have this thread panic if we’re in that situation.
// After we’ve acquired the lock, we can treat the return value, named num in this case,
//   as a mutable reference to the data inside. The type system ensures that we acquire a
//   lock before using the value in m: Mutex<i32> is not an i32, so we must acquire the
//   lock to be able to use the i32 value. We can’t forget; the type system won’t let us
//   access the inner i32 otherwise.
// As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock
//   returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled
//   with the call to unwrap. The MutexGuard smart pointer implements Deref to point at our
//   inner data; the smart pointer also has a Drop implementation that releases the lock
//   automatically when a MutexGuard goes out of scope. As a result, we don’t risk forgetting
//   to release the lock and blocking the mutex from being used by other threads because the
//   lock release happens automatically.
//
// After dropping the lock, we can print the mutex value and see that we were able to change
//   the inner i32 to 6.
//
// As move will take the ownership of Mutex if we use it directly in spawn.
// So if we want to share Mutex<T> between multiple threads, we need to give the ownership
//   of the Mutex value to every thread. To do this, we use the atomic reference counting
//   smart pointer Arc<T>, where atomic means it’s an atomically reference counted type
//   that can be sent across threads.

use std::sync::{Arc,};
pub fn arc_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

// The Send marker trait indicates that ownership of values of the type implementing
//   Send can be transferred between threads. Almost every Rust type is Send,
//   but there are some exceptions, including Rc<T>
// The Sync marker trait indicates that it is safe for the type implementing Sync
//   to be referenced from multiple threads. Primitive types are Sync,
//   and types composed entirely of types that are Sync are also Sync. Rc<T> is also not Sync.
// Send and Sync are marker traits.
// Marker types don’t have any methods to implement.
//   They’re just useful for enforcing invariants related to concurrency



// next: advanced classes: will + recordings
//