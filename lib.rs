#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hello_sys() {
		unsafe {
			hello();
		}
	}
}