use super::ram_component::Ram;

fn parse_line (line: &str) -> Vec<&str> {
  let string = String::from(line);
  Vec::from_iter(string.split(' '))
}

fn execute_line (line: &str, mut ram: &Ram ) {
  let parsed_line = parse_line(line);
    
}

pub fn execute_source_code (source: String, ram: Ram ) {
  for line in source.lines() {
    execute_line(line, &ram);
  } 
}
