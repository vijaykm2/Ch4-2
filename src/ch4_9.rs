fn main() {
    fn foo( v1: &Vec<i32>, v2: &Vec<i32> ) -> i32{
        let mut result = 0;
        for i in v1 {
            result =i+ result;
        }
        for i in v2 {
            result=i + result;
        }
        result
    }

    let v1 = vec![123,234,1232];
    let v2 = vec![203,392,3002];
    println!("result = {}", foo(&v1, &v2));
    let mut x =12;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("x = {}",x );
    let mut a = 2;
    {
        let mut b = a;
        b+=1;
    }
    println!("a = {}", a)
}