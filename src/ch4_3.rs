fn main(){
	let x =true;
	let y:bool = true;
	let x= 'x';
	let x = 42;
	let a = [1,2,3,4,5];
	let mut m = [1,2,3];
	println! ("m has {} elements", m.len());
	let names = ["Graydon", "Brian", "Niko"];
	println!("{}",names[2]);
	println!("{}", &a[1..4][0]);
	let mut x = (1,2);
	let y = (22,23);
	x=y;
	let (x, a, z) = (1, 2, 3);

	println!("y 2is {}", y.1);

	fn inc(x: i32) -> i32{
		x+1
						 }

	let x = inc;

}
