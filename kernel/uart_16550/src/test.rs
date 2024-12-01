test_system::declare_tests! {
    test_example => {
        assert_eq!(1 + 1, 2);
    },
    another_test => {
        assert_eq!(2 + 2, 4);
    },
}
