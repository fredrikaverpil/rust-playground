mod comments_and_documentation;
mod mod01_core_datatypes;
mod mod02_operators;
mod mod03_scope_and_shadowing;
mod mod04_global_variables;
mod mod05_stack_and_heap;
mod mod06_control_flow;
mod mod07_structs;
mod mod08_enums;
mod mod09_unions;
mod mod10_options;
mod mod11_arrays;
mod mod12_slices;
mod mod13_tuples;
mod mod14_pattern_matching;
mod mod15_generics;
mod mod16_vectors;
mod mod17_hashmaps;
mod mod18_hashsets;
mod mod19_iterators;
mod mod20_strings;
mod mod21_format_macro;
mod mod22_number_guessing_game;
mod mod23_functions;
mod mod24_methods;
mod mod25_closures;
mod mod26_higher_order_functions;
mod mod27_traits;
mod mod28_trait_parameters;
mod mod29_trait_into;
mod mod30_trait_drop;
mod mod32_trait_operator_overloading;
mod mod33_static_dynamic_dispatch;
mod mod34_trait_vectors_different_objects;
mod mod35_ownership_and_memory_safety;
mod mod36_borrowing;
mod mod37_lifetime;
mod mod39_reference_counting;
mod mod40_atomic_reference_counting;
mod mod41_mutex;
mod mod42_threads;
mod mod43_consuming_crates;
mod mod44_building_modules_and_crates;

fn main() {
    // Datatypes
    mod01_core_datatypes::main();
    mod02_operators::main();
    mod03_scope_and_shadowing::main();
    mod04_global_variables::main();
    mod05_stack_and_heap::main();

    // Control flow
    mod06_control_flow::main();

    // Data structures
    mod07_structs::main();
    mod08_enums::main();
    mod09_unions::main();
    mod10_options::main();
    mod11_arrays::main();
    mod12_slices::main();
    mod13_tuples::main();
    mod14_pattern_matching::main();
    mod15_generics::main();

    // Standard collections
    mod16_vectors::main();
    mod17_hashmaps::main();
    mod18_hashsets::main();
    mod19_iterators::main();
    mod20_strings::main(); // https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
    mod21_format_macro::main();
    mod22_number_guessing_game::main();

    // Functions
    mod23_functions::main();
    mod24_methods::main();
    mod25_closures::main();
    mod26_higher_order_functions::main();

    // Traits
    mod27_traits::main();
    mod28_trait_parameters::main();
    mod29_trait_into::main();
    mod30_trait_drop::main();
    mod32_trait_operator_overloading::main();
    mod33_static_dynamic_dispatch::main();
    mod34_trait_vectors_different_objects::main();

    // Lifetime and memory
    mod35_ownership_and_memory_safety::main();
    mod36_borrowing::main();
    mod37_lifetime::main();
    mod39_reference_counting::main();
    mod40_atomic_reference_counting::main();
    mod41_mutex::main();

    // Concurrency
    mod42_threads::main();

    // Modules and crates
    mod43_consuming_crates::main();
    mod44_building_modules_and_crates::main();

    // Tests
    // See phrases module, src/lib.rs and tests/integration_tests.rs

    // Comments and documentation
    comments_and_documentation::main()
}
