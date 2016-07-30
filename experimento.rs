

fn foo<'a>(v1: &'a Vec<i32>, v2: &'a Vec<i32>) -> (&'a Vec<i32>, &'a Vec<i32>, i32) {
    (v1, v2, 42)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let x = foo(&v1, &v2);
    v1[1];
    test();
}



fn bar<'a>(i: &'a i32, j: &'a i32) -> i32 {
  i + j
}

fn bar2<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  i + j
}

fn test2(i: &i32) -> i32 {
  let y: i32 = 20;
  bar(i, &y) + bar2(i, &y)
}

fn test() {
  let x: i32 = 10;
  let y: i32 = 20;
  println!("x + y = {}", bar(&x, &y));
  println!("x + y = {}", test2(&x));
}
