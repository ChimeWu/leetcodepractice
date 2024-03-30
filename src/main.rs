use std::collections::HashMap;

fn main() {
    let mut vec:Vec<Vec<i32>> = Vec::new();
    let mut x = 10;
    let mut t = 9;
    let mut y = &mut x;
    let z = &mut y;
    *z = &mut t;
    **z = 100;
    println!("{}",z);
    println!("{}",y);
    println!("{}",t);
    println!("{}",x);
    println!("{}",x);
    for i in 0..5 {
        let mut v:Vec<i32> = Vec::new();
        for j in 0..5 {
            v.push(j as i32);
        }
        vec.push(v);
    }
    let a = &mut vec;
    (*a)[0][0] = 100;
    println!("{:?}",a);
    *a = vec![vec![1,2,3],vec![4,5,6]];

    println!("{:?}",a);
    println!("{:?}",vec);

}

