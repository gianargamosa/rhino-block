mod block;

fn main() {
  let block = Block::create_block(
    String::from("this is a new block created!"),
    [],
    0,
  );

  println!("this is the new block! {}", block.hash)
}
