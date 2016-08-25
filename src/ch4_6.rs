fn main(){
    let mut x:i64 =9;
    let mut done  = false;

    while !done {
        x+=x-2;
        println!("x = {}", x);
        if (x%5 ==0){
            done =true;
        }
    }

    println!("For loop!!");
    let mut y:u64 =1;
    for i in 1..10 {
        y*=i;
    }
    println!("y = {}", y);
}