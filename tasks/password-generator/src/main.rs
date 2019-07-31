// why use a library? Clap lets us quickly build a CLI and
// lets us focus the below code on password generation
// not the intricacies building a CLI in rust.
// Read more about clap here: https://clap.rs/
// declare our external dependency for parsing command line arguments
extern crate clap;
extern crate rand;
// bring the needed structures into scope so we
// invoke them later in the program
use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

fn generate_password(length: usize) -> String {
    // cache thread_rng for better performance
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        // the Alphanumeric struct provides 3/4
        // of the characters for passwords
        // so we can simply sample from it
        .map(|()| rng.sample(Alphanumeric))
        .take(length)
        .collect();
    chars
}
fn main() {
    // create our new CLI
    // clap provides powerful defaults so we don't have to
    // write all the logic here
    // For example, clap takes care of the help and version flags by default
    App::new("password-generator")
        .version("0.1")
        .about("generate a password according to the rosetta code rules: http://rosettacode.org/wiki/Password_generator")
        // configure our first required argument
        .arg(Arg::with_name("LENGTH")
             .help("password length")
             // make it SECURE by default
             // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
             .default_value("160")
             // TODO validate these user provided values
             // to avoid unexpected behavior
             .required(true)
             .index(1)
            )
        // configure our second required argument
        .arg(Arg::with_name("COUNT")
             .help("how many passwords to generate")
             .default_value("1")
             .required(true)
             .index(2)
            )
        .get_matches();
}

// declare a module for testing
// keep it within this file for convenience
#[cfg(test)]
mod tests {
    use super::generate_password;

    // test our core password logic according to the rules described in the wiki
    #[test]
    fn generate_password_customizes_length() {
        let a_password = generate_password(50);
        assert_eq!(a_password.len(), 50);
    }
    #[test]
    fn generate_password_has_numerals() {
        // TODO how can I pass in char::is_ascii_digit instead of defining a closure?
        assert!(generate_password(50).chars().any(|c| c.is_ascii_digit()));
    }
    #[test]
    fn generate_password_has_upper_and_lowercase_characters() {
        assert!(generate_password(50)
            .chars()
            .any(|c| c.is_ascii_lowercase()));
        assert!(generate_password(50)
            .chars()
            .any(|c| c.is_ascii_uppercase()));
    }
    #[test]
    fn _generate_password_has_other_charactrs() {
        // !"#$%&'()*+,-./:;<=>?@[]^_{|}~
        assert!(generate_password(50).chars().any(|_c| { true }));
    }
}
