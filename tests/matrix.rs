use chu::prelude::Matrix;

#[test]
fn test_matrix() {
    //

    let mut m = Matrix::<usize>::new((3, 5));
    m.fill(|(i, j)| i * j);
    // dbg!(m);
    let n = m.transpose();

    println!("{:?}", m.data);
    println!("{:?}", n.data);
    // println!("{:?}", m.row(0));
}
