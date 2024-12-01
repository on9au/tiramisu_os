#![no_std]

/// Declare tests in a given module.
#[macro_export]
macro_rules! declare_tests {
    ($($name:ident => $body:block),* $(,)?) => {
        $(
            pub fn $name() {
                $body
            }
        )*

        pub const TESTS: &[(&dyn Fn(), &str)] = &[
            $(
                (&$name, stringify!($name)),
            )*
        ];
    };
}
