// This file is intended for analysis by the hubstry-iso-code CLI,
// not as a standalone runnable example.
// We add a dummy main and stubs to make it a valid binary for the compiler.

#![allow(dead_code, unused_variables)]

/// ECA.PARENT.CONSENT: This function collects data and must have consent.
fn save_user_profile() {
    // This function call should be detected as data collection.
    save_data_to_database();
}

/// ECA.AGE.VERIFY: This function must check the user's age.
fn check_user_age() {
    // Missing actual age check logic.
    println!("Proceeding without age check.");
}

// --- Stubs to make the file compile ---

fn save_data_to_database() {}

fn main() {
    // This main function is empty because this file is for static analysis.
}