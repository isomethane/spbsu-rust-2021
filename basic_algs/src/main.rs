fn max<T: Ord>(elements: &[T; 10]) -> &T {
    let mut result = &elements[0];
    for e in elements {
        if result <= e {
            result = e;
        }
    }
    result
}

fn nth_prime(n: usize) -> usize {
    const MAX_NUMBERS: usize = 105000;
    let mut is_prime = [true; MAX_NUMBERS];

    let mut index = 0;

    for i in 2..MAX_NUMBERS {
        if !is_prime[i] {
            continue;
        }
        index += 1;
        if index == n {
            return i;
        }
        for j in (i * 2..MAX_NUMBERS).step_by(i) {
            is_prime[j] = false;
        }
    }
    panic!("Failed to find {}-th prime number", n)
}

fn binary_search<T: Ord>(elements: &[T; 10], x: &T) -> Result<usize, usize> {
    let mut l: usize = 0;
    let mut r: usize = elements.len();
    while l < r {
        let m = (l + r) / 2;
        if elements[m] < *x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    if elements[l] == *x {
        Ok(l)
    } else {
        Err(l)
    }
}

fn main() {
    let unordered: [i64; 10] = [55, 0, 15, 18, 1, 74, -5, 1312, 21, 9];
    let ordered: [i64; 10] = [-5, 0, 1, 9, 15, 18, 21, 55, 74, 1312];

    println!("max = {}, expected 1312", max(&unordered));
    println!("index of 55 = {}, expected 7", binary_search(&ordered, &55).unwrap());
    println!("index of >56 = {}, expected 8", binary_search(&ordered, &56).unwrap_err());

    for i in 1..11 {
        println!("prime number {}: {}", i, nth_prime(i))
    }
    println!("prime number 10000: {}", nth_prime(10000))
}
