extern mod toml = "toml#0.1";

use std::os;

fn main() {
  if os::args().len() < 2 {
    println!("usage: ./simple input-file");
    os::set_exit_status(1);
    return;
  }
  let value = match toml::parse_from_file(os::args()[1]) {
    Ok(v) => v,
    Err(toml::ParseError) => {
      println!("parse error");
      os::set_exit_status(1);
      return;
    }
    Err(toml::IOError(e)) => {
      println!("I/O error: {}", e);
      os::set_exit_status(1);
      return;
    }
  };
  println!("{:s}", value.to_str());

  let a = value.lookup_key("a").and_then(|a| a.get_str());
  if a.is_some() { println!("Found a: {:?}", a) }

  let abc_def_a = value.lookup_key("abc").and_then(|a| a.lookup_key("def").and_then(|a| a.lookup_key("a")));
  if abc_def_a.is_some() { println!("Found abc.def.a: {:?}", abc_def_a) }

  let a = value.lookup_path(["abc", "def", "a"]);
  if a.is_some() { println!("Found a: {:?}", a) }

  let product_0 = value.lookup("products.0");
  if product_0.is_some() { println!("Found product[0]: {:?}", product_0) }
}
