use fib::fib;
use hello_world_methods::MULTIPLY_ID;

fn main() {
    // Pick two numbers
    let receipt = fib();

    // Here is where one would send 'receipt' over the network...
    // println!(
    //     "receipt journal(public output): {:?}",
    //     receipt.journal.decode::<u64>().unwrap()
    // );

    // Verify receipt, panic if it's wrong
    receipt.verify(MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
