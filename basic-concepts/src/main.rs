mod functions;
mod types;
mod control_flow;
mod ownership;

fn main() {
  functions::_func();
  functions::_func_with_params(10, 10.0);
  
  control_flow::_if();
  control_flow::_if_as_expression();
  control_flow::_loop();
  control_flow::_loop_as_expression();
  control_flow::_while_loop();
  control_flow::_iterate_array_unsafe();
  control_flow::_iterate_array_safe();
  control_flow::_iterate_range();

  ownership::_move_ownership();
}
