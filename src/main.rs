fn is_valid_upc(upc: &str) -> bool {
    if upc.len() != 12 || !upc.chars().all(|c| c.is_digit(10)) {
        return false;
    } else {
        let digits: Vec<u32> = upc.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let sum_odd: u32 = digits.iter().enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, &digit)| digit)
            .sum();

        let sum_even: u32 = digits.iter().enumerate()
            .filter(|&(i, _)| i % 2 != 0)
            .map(|(_, &digit)| digit)
            .sum();

        let total_sum = (sum_odd * 3) + sum_even;
        total_sum % 10 == 0
    }
}

fn main() {
    let upc = "036000291452";

    if is_valid_upc(upc) {
        println!("The UPC {} is valid.", upc);
    } else {
        println!("The UPC {} is not valid.", upc);
    }
}
