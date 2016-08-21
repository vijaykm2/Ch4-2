fn main(){
	print_number(5);
	print_number(sum(5, 30));
	diverges();
}

fn print_number(x: i32){
	println!("argument X = {}", x)
}
fn sum (x: i32, y: i32) -> i32{
	return x+y;
}

fn diverges() ->!{
	panic!("This function never returns!!");
}