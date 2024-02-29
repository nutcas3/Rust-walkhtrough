fn main() {
      // Stack-allocated variable
      let stack_variable = 42;

      // Heap-allocated variable
      let heap_variable = Box::new(42);
  
      println!("Stack variable: {}", stack_variable);
      println!("Heap variable: {}", heap_variable);
}
