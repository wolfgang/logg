#[test]
fn can_pass_function_as_argument() {
	assert_eq!(24, fun_fun(12, &times_2));
}

fn times_2(value: i32) -> i32 {
	value*2
}

fn fun_fun(value: i32, f: &Fn(i32) -> i32) -> i32 {
    f(value)
}