use std::collections::HashSet;

fn main() {
    println!("Unpacking Tuple");
    unpacking();
    unpacking_arbitrary();
}

/*python
# Unpacking a sequence into separate variables
t = (7, 'Seas', [1, 3, 5], {'a', 'd', 'z'})
n, w, t, s = t
print(n, w, t, s)
*/

fn unpacking() {
    let hs: HashSet<char> = vec!['a', 'd', 'z'].into_iter().collect();
    let t = (7, "Seas", [1,3,5], hs);
    let (n, w, t, s) = t;
    println!("{}, {}, {:?}, {:?}", n, w, t, s);
}


fn unpacking_arbitrary() {
    // Currently only works with arrays and slices
    let record = [1, 2, 3, 4, 5, 6, 7];
    match record {
        [x, y @ ..] => println!("{:?} {:?}", x, y)
    };
}