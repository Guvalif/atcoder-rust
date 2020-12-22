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

  mod b {
    use atcoder_rust::abc128::b;

    #[test]
    fn case1() {
      let scores: b::Scores = vec![
        ("khabarovsk", 20),
        ("moscow", 10),
        ("kazan", 50),
        ("kazan", 35),
        ("moscow", 60),
        ("khabarovsk", 40),
      ];

      assert_eq!(
        b::solve(scores.len() as u32, scores),
        vec![
          3,
          4,
          6,
          1,
          5,
          2,
        ],
      )
    }

    #[test]
    fn case2() {
      let scores: b::Scores = vec![
        ("yakutsk", 10),
        ("yakutsk", 20),
        ("yakutsk", 30),
        ("yakutsk", 40),
        ("yakutsk", 50),
        ("yakutsk", 60),
        ("yakutsk", 70),
        ("yakutsk", 80),
        ("yakutsk", 90),
        ("yakutsk", 100),
      ];

      assert_eq!(
        b::solve(scores.len() as u32, scores),
        vec![
          10,
          9,
          8,
          7,
          6,
          5,
          4,
          3,
          2,
          1,
        ],
      )
    }
  }
}
