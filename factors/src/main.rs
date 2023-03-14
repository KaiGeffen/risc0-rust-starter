use methods::{BOUNDED_ID, BOUNDED_PATH};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};


fn main() {
    // Make the prover.
    let method_code = std::fs::read(BOUNDED_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut prover = Prover::new(&method_code, BOUNDED_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // TODO: Implement communication with the guest here
    let a: u64 = 17;
    let b: u64 = 23;

    prover.add_input_u32_slice(&to_vec(&a).unwrap());
    prover.add_input_u32_slice(&to_vec(&b).unwrap());

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(BOUNDED_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);
}
