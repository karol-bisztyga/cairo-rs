use cairo_rs::cairo_run::cairo_run;
use cairo_rs::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::{
    BuiltinHintProcessor, HintFunc,
};
use cairo_rs::hint_processor::builtin_hint_processor::hint_utils::get_integer_from_var_name;
use cairo_rs::hint_processor::hint_processor_definition::HintReference;
use cairo_rs::types::exec_scope::ExecutionScopes;
use cairo_rs::serde::deserialize_program::ApTracking;
use cairo_rs::vm::{errors::vm_errors::VirtualMachineError, vm_core::VirtualMachine};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::path::Path;

// Create the function that implements the custom hint
fn print_a_hint(
    vm: &mut VirtualMachine,
    _exec_scopes: &mut ExecutionScopes,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
    _constants: &HashMap<String, BigInt>,
) -> Result<(), VirtualMachineError> {
    let a = get_integer_from_var_name("a", vm, ids_data, ap_tracking)?;
    println!("{}", a);
    Ok(())
}

fn main() {
    // Wrap the Rust hint implementation in a Box smart pointer inside a HintFunc
    let hint = HintFunc(Box::new(print_a_hint));

    //Instantiate the hint processor
    let mut hint_processor = BuiltinHintProcessor::new_empty();

    //Add the custom hint, together with the Python code
    hint_processor.add_hint(String::from("print(ids.a)"), hint);

    //Run the cairo program
    cairo_run(
        Path::new("custom_hint.json"),
        "main",
        false,
        false,
        &hint_processor,
    )
    .expect("Couldn't run program");
}
