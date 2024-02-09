fn main() {
   let mut val = vec![1,3,5,7];
   let result = is_value_one(&val);
   println!("{:?}", result);
   val.push(15);
   println!("{:?}", result);



   let mut val1 = 2;
   add_two(val1);
   val1 = 5;
   println!("{:?}", val1);
}

fn is_value_one(val: &Vec<i8>) -> bool {
   return val[0] == 1;
}

fn add_two(val: i8) {
   let _ = val + 2;
}
