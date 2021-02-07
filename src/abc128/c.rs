pub type Connections = Vec<Vec<u16>>;
pub type Parities = Vec<u16>;

fn connection_to_bits(connection: Vec<u16>) -> u16 {
  let bits = connection
    .into_iter()
    .fold(0, |acc, x| acc | (0b01 << (x - 1)));
  
  bits
}

fn parities_to_bits(parities: Parities) -> u16 {
  unimplemented!();
}

pub fn solve(n: u32, connections: Connections, parities: Parities) -> u32 {
  let switch_patterns = vec![ 0 .. 2u32.pow(n) ];

  let _bits_patterns: Vec<_> = connections
    .into_iter()
    .map(connection_to_bits)
    .collect();

  unimplemented!();
}
