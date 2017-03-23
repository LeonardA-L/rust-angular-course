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

#[test]
fn test_sum_of_primes() {
	assert_eq!(5, root::sum_prime::sum_prime(4));
	assert_eq!(17, root::sum_prime::sum_prime(10));
	assert_eq!(328, root::sum_prime::sum_prime(50));
	assert_eq!(1060, root::sum_prime::sum_prime(100));
	assert_eq!(1264, root::sum_prime::sum_prime(103));
	assert_eq!(5736396, root::sum_prime::sum_prime(1000));
}