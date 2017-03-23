pub fn is_prime(number: i32) -> bool {
	println!("The number is {}", number);
	
	return true;

	// For demo purposes:
    /*
    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    return number > 1;
    */
}