use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // LinkedList
    linked_list_operations();

    // HashMap
    hash_map_operations();

    // TODO: your text explanation to the questions in the spec

    // Vector
    // Vector insertion at the end, this process does not require reallowcating and copying the entire array
    // Removing an element from anywhere other than the end requires shifting elements to fill the gap left by the removed element
    
    // VecDeque
    // Insertion at the end is fast takes O(1) time
    // Removal from the front and the end are fast which take O(1) time

    // LinkedList
    // push back and pop front are fast because they only need to update pointers which takes O(1) time
    // VecDeque may perform better than LinkedList due to better memory access patterns and cache utilitzation

    // HashMap
    // insertion and removal involves computing the hash of the key, determining the appropriate bucket based on this hash, and then 
    // deal with the key-value pair to that bucket
}

// measure the insertion and removal
// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

// measure the insertion and removal
// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

// measure the insertion and removal
// operations of a LinkedList
fn linked_list_operations() {
    let mut linked_list = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        linked_list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        linked_list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

// measure the insertion and removal
// operations of a HashMap
fn hash_map_operations() {
    let mut hash_map = HashMap::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.insert(i, 1);
    }
    let time_end = std::time::Instant::now();

    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}
