use std::collections::HashSet;

#[derive(Debug)]
struct Mat2x2(f32, f32, f32, f32);

impl Mat2x2 {
    fn comb(&self) -> [f32; 4]  {
        [self.0, self.1, self.2, self.3]
    }    

    fn determinant(&self) -> f32 {
        self.0 * self.3 - self.1 * self.2
    }
}

fn main() {
    println!("Unpacking Tuple");
    unpacking();
    unpacking_arbitrary();

    let tup = ("Tupple", 2.5, 37_i32);
    let reversed = reverse_triplet(tup);
    println!("{:?}", reversed);

    let square_mat = Mat2x2(1.5, -2f32, -0.5, 4.2);
    println!("{:?}", square_mat);

    for item in square_mat.comb() {
        println!("{}", item);
    }
    println!("{}", square_mat.determinant());
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


fn reverse_triplet(t: (&str, f32, i32)) -> (i32, f32, &str) {
    println!("{:?}", t);
    (t.2, t.1, t.0)
}
