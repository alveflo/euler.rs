// If we list all the natural numbers below 10
// that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn test(num: i32, sum: i32) -> i32 {
  if num > 999 {
    return sum;
  } else {
    if num % 3 == 0 || num % 5 == 0 {
      return test(num + 1, sum + num);
    } else {
      return test(num + 1, sum);
    }
  }
}

pub fn run() {
  println!("Problem1: {}", test(1, 0));
}
