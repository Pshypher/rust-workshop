// Destructuring Assignments
// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment.

pub fn destructure_assignment() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]); // assert that `left == right`
    println!("Successful compile of {} {}", x, y);
}
