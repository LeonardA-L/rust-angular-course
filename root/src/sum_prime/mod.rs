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

pub fn sum_prime(limit: i32) -> i32 {
	println!("The limit is {}", limit);

	return 0;

	// For demo purposes:
	/*
	let mut sum = 0;
	
    for x in 2..(limit+1) {
	    if is_prime(x) {
	    	sum += x;
	    }
	}

	return sum;
	*/
}