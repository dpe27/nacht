#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs_70(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        let mut dp = vec![0u8; n as usize + 1];
        (dp[0], dp[1]) = (1, 1);
        (2..=n as usize).for_each(|i| dp[i] = dp[i - 1] + dp[i - 2]);
        dp[n as usize] as i32
    }

    pub fn generate_118(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp: Vec<Vec<i32>> = vec![];
        let mut prev_row: Vec<i32> = vec![];

        (0..num_rows as usize).for_each(|i| {
            let mut curr = vec![1i32; i + 1];
            (1..i).for_each(|j| curr[j] = prev_row[j - 1] + prev_row[j]);
            dp.push(curr.clone());
            prev_row = curr;
        });
        dp
    }

    pub fn max_profit_121(prices: Vec<i32>) -> i32 {
        let mut profit = vec![0; prices.len()];
        let mut max_price = *prices.last().unwrap();
        (0..prices.len() - 1).rev().for_each(|i| {
            if max_price > prices[i] {
                profit[i] = profit[i + 1].max(max_price - prices[i]);
            } else {
                profit[i] = profit[i + 1];
            }
            max_price = max_price.max(prices[i]);
        });
        profit[0]
    }

    pub fn fib_509(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        (2..=n as usize).for_each(|i| dp[i] = dp[i - 1] + dp[i - 2]);
        dp[n as usize]
    }

    pub fn min_cost_climbing_stairs_746(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len()];
        (dp[0], dp[1]) = (cost[0], cost[1]);
        (2..cost.len()).for_each(|i| dp[i] = cost[i] + dp[i - 1].min(dp[i - 2]));
        dp[cost.len() - 1].min(dp[cost.len() - 2])
    }

    pub fn tribonacci_1127(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }

        if n == 2 {
            return 1;
        }
        let mut dp = vec![0; n as usize + 1];
        (dp[1], dp[2]) = (1, 1);
        (3..=n as usize).for_each(|i| dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3]);
        dp[n as usize]
    }
}
