extern crate root;

#[test]
fn test_with_prime_numbers() {
	assert!(root::is_prime::is_prime(2));
	assert!(root::is_prime::is_prime(5));
	assert!(root::is_prime::is_prime(11));
	assert!(root::is_prime::is_prime(23));
	assert!(root::is_prime::is_prime(47));
}

#[test]
#[should_panic]
fn test_with_non_prime_numbers() {
	assert!(root::is_prime::is_prime(1));
	assert!(root::is_prime::is_prime(15));
	assert!(root::is_prime::is_prime(52));
	assert!(root::is_prime::is_prime(121));
}