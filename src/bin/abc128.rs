extern crate atcoder_rust;

fn main() {}

#[cfg(test)]
mod tests {
  mod a {
    use atcoder_rust::abc128::a;

    #[test]
    fn case1() {
      assert_eq!(a::solve(1, 3), 3);
    }

    #[test]
    fn case2() {
      assert_eq!(a::solve(0, 1), 0);
    }

    #[test]
    fn case3() {
      assert_eq!(a::solve(32, 21), 58);
    }
  }
}
