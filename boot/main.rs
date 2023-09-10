extern crate inkwell;

use std::process::Command;

use inkwell::{
    values::{BasicMetadataValueEnum, IntValue},
    OptimizationLevel,
};

mod grammar;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input = args[1].clone();
    let parsetree = grammar::TermParser::new().parse(&input).unwrap();

    let ctx = inkwell::context::Context::create();
    let module = ctx.create_module("boot");

    // Declare the external `printf` function
    let i8_type = ctx.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::default());
    let printf_type = ctx.i32_type().fn_type(&[i8_ptr_type.into()], true);
    let printf = module.add_function("printf", printf_type, None);

    let i32_type = ctx.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = ctx.append_basic_block(function, "entry");

    let builder = ctx.create_builder();
    builder.position_at_end(basic_block);

    let value = i32_type.const_int(parsetree as u64, false);
    let format_string_ptr = builder.build_global_string_ptr("%d\n", "format_string");

    builder.build_call(
        printf,
        &[format_string_ptr.as_pointer_value().into(), value.into()],
        "printf_call",
    );

    let const_int = i32_type.const_int(0, false);
    builder.build_return(Some(&const_int));

    if function.verify(true) {
        println!("Function is valid");
    } else {
        println!("Function is invalid");
    }

    let ir_code = module.print_to_string().to_string();
    println!("{}", ir_code);

    let output_file = "output.ll";
    module
        .print_to_file(output_file)
        .expect("Failed to write LLVM IR to file");

    let output = Command::new("llc")
        .arg("output.ll")
        .output()
        .expect("Failed to execute GCC");

    let output = Command::new("gcc")
        .arg("-o")
        .arg(format!("output-{input}"))
        .arg("output.s")
        .output()
        .expect("Failed to execute GCC");

    println!("ok");
}
