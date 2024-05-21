// The Borrow Checker - Checks to see if every borrow (reference made using &) is valid

fn main() {
    let propellant; // --- lifetime `a starts
    let rpl = String::from("RP-1"); // The lifetime of this scope is `a.  Since propellant and rpl are in the same lifetime, this code compiles.
    {
        propellant = &rpl; // --- lifetime `b

        // the lifetime of this is `b.  It's lifetime does not start at the second pair of curly braces, but rather,
        // it starts when rpl is first initialized.
    } // --- lifetime `b ends
    println!("propellant is {}", propellant); // --- lifetime `a ends

    // Since propellant and rpl are still in lifetime `a, lifetime `b is still utilized in lifetime `a
}
