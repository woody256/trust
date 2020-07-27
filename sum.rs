fn sum(a: i32, b: i32) -> i32 {
    return a+b;
}

fn update(mut arr:[i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

fn main() {
    println!("Hello, world!");
    let result = sum(1, 2);
    println!("{}", result);
}
