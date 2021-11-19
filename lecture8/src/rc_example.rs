use std::rc::Rc;

fn main() {
    let mut rc = Rc::new(5); // create a strong pointer
                             // *rc = 6; //Doesn't work
    println!(
        "Strong count: {}, weak count: {}",
        Rc::strong_count(&rc),
        Rc::weak_count(&rc)
    );

    let strong_ptr1 = Rc::clone(&rc); // second strong pointer
    println!(
        "Strong count: {}, weak count: {}",
        Rc::strong_count(&rc),
        Rc::weak_count(&rc)
    );

    let weak_ptr = Rc::downgrade(&rc); // create a weak pointer
    println!(
        "Strong count: {}, weak count: {}",
        Rc::strong_count(&rc),
        Rc::weak_count(&rc)
    );

    // upgrade the weak pointer to be the third strong poointer
    let strong_ptr2 = weak_ptr.upgrade();
    println!(
        "Strong count: {}, weak count: {}",
        Rc::strong_count(&rc),
        Rc::weak_count(&rc)
    );

    // drop a strong pointer
    drop(rc);
    drop(strong_ptr1);
    drop(strong_ptr2);
    // at this time, the data stored in rc was dropped, but not the count
    drop(weak_ptr);
    // at this time, the rc was dropped
}
