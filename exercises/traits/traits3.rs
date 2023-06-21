// traits3.rs
//
// Your task is to implement the Licensed trait for
// both structures and have them return the same
// information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a hint.


use std::fmt;

pub trait Licensed {
    fn licensing_info(&self) -> String
    where Self: fmt::Display
    {
        format!("{}", self)
    }
}

struct SomeSoftware {
    version_number: i32,
}

impl fmt::Display for SomeSoftware {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "Some information")
    }
}

struct OtherSoftware {
    version_number: String,
}

impl fmt::Display for OtherSoftware {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "Some information")
    }
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
