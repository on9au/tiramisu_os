#![no_std]

/// Declare tests in a given module.
#[macro_export]
macro_rules! declare_tests {
    ($($name:ident => $body:block),* $(,)?) => {
        use test_system::Testable;
        $(
            pub fn $name() {
                $body
            }
        )*

        pub const TESTS: &[&dyn Testable] = &[
            $(
                &$name,
            )*
        ];
    };
}

pub trait Testable {
    fn run(&self);
    fn name(&self) -> &str;
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }

    fn name(&self) -> &str {
        core::any::type_name::<T>()
    }
}
