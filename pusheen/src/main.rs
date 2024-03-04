fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // exclusive borrow should be unique
    // if there are two, then it can't compile
    // we should ensure that only one mutable reference exists at any given time

    {
        let a = &mut vec;
        a.push(11);
    }

    {
        let b = &mut vec;
        b.push(12);
    }
    
    
}
