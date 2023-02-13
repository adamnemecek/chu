use chu::prelude::Matrix;

#[test]
fn test_matrix() {
    //

    let mut m = Matrix::<usize>::new((3, 5));
    m.fill(|(i, j)| i * j);
    // dbg!(m);
    // println!("{:?}", m.row(1));
    println!("{:?}", m.data);
}
