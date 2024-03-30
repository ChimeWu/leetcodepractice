//初级篇

//1.穷竭搜索

//1.1递归
pub fn fib(n: i32) -> i32{
    let mut vec1 = vec![0; (n+1) as usize];

    fn fib1(n: i32, vec1: &mut Vec<i32>) -> i32{
        if n == 0{
            return 0;
        }
        if n == 1{
            return 1;
        }
        if vec1[n as usize] != 0{
            return vec1[n as usize];
        }
        vec1[n as usize] = fib1(n-1, vec1) + fib1(n-2, vec1);
        return vec1[n as usize];
    }
    fib1(n, &mut vec1)
}

pub fn fib2(n: i32) -> i32{
    if n == 0{
        return 0;
    }
    if n == 1{
        return 1;
    }
    return fib2(n-1) + fib2(n-2);
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_fib(){
        let start = Instant::now();
        println!("{}",fib(40));
        let duration = start.elapsed();
        println!("Time elapsed in fib() is: {:?}", duration);
        let start = Instant::now();
        println!("{}",fib2(40));
        let duration2 = start.elapsed();
        println!("Time elapsed in fib2() is: {:?}", duration2);
        println!("fib1/fib2 = {}", duration2.as_nanos() as f64 / duration.as_nanos() as f64);
    }
}