pub mod price_stock {
    use std::i32;

    pub fn test() {
        println!(
            "[7,1,5,3,6,4] -> {}",
            max_profit(Vec::from([7, 1, 5, 3, 6, 4]))
        );

        println!("[1,2,3,4,5] -> {}", max_profit(Vec::from([1, 2, 3, 4, 5])));
        println!("[7,6,4,3,1] -> {}", max_profit(Vec::from([7, 6, 4, 3, 1])));
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;

        for i in 1..prices.len() {
            let current = prices[i];
            let prev = prices[i - 1];

            if current > prev {
                total_profit += current - prev;
            }
        }

        total_profit
    }
}
