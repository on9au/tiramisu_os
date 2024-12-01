#![no_std]

#[macro_export]
macro_rules! declare_tests {
    ($($name:ident => $body:block),* $(,)?) => {
        $(
            pub fn $name() {
                $body
            }
        )*

        pub const TESTS: &[&dyn Fn()] = &[
            $(
                &$name,
            )*
        ];
    };
}
