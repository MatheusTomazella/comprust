pub mod ram_component;
mod hd_component;
mod cpu_component;

fn main() {
  cpu_component::execute_source_code(
    hd_component::get_sorce_code("./programs/code1.txt"), 
    ram_component::init_ram()
  );
}
