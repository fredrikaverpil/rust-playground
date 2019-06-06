mod helloworld;
mod core_datatypes;
mod operators;
mod scope_and_shadowing;
mod global_variables;
mod stack_and_heap;
mod control_flow;
mod structs;
mod enums;
mod unions;
mod options;
mod arrays;
mod vectors;
mod slices;
mod strings;
mod tuples;
mod pattern_matching;
mod generics;
mod functions;
mod methods;
mod closures;
mod higher_order_functions;
mod traits;
mod operator_overloading;
mod static_dynamic_dispatch;
mod ownership_and_memory_safety;
mod borrowing;
mod lifetime;
mod reference_counting;
mod atomic_reference_counting;
mod mutex;
mod modules;
mod comments_and_documentation;

fn main() {
    helloworld::main();

    // Datatypes
    core_datatypes::main();
    operators::main();
    scope_and_shadowing::main();
    global_variables::main();
    stack_and_heap::main();

    // Control flow
    control_flow::main();

    // Data structures
    structs::main();
    enums::main();
    unions::main();
    options::main();
    arrays::main();
    vectors::main();
    slices::main();
    strings::main();  // https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
    tuples::main();
    pattern_matching::main();
    generics::main();

    // Functions
    functions::main();
    methods::main();
    closures::main();
    higher_order_functions::main();

    // Traits
    traits::main();
    operator_overloading::main();
    static_dynamic_dispatch::main();

    // Lifetime and memory
    ownership_and_memory_safety::main();
    borrowing::main();
    lifetime::main();
    reference_counting::main();
    atomic_reference_counting::main();
    mutex::main();

    // Modules
    modules::main();

    // Tests
    // See phrases module, src/lib.rs and tests/integration_tests.rs

    // Comments and documentation
    comments_and_documentation::main()

}
