/*

给定一个数组 prices ，其中 prices[i] 是一支给定股票第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

作者：力扣 (LeetCode)
链接：https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x2zsx1/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

提示：

    1 <= prices.length <= 3 * 104
    0 <= prices[i] <= 104
    
输入: prices = [7,1,5,3,6,4]
输出: 7
解释: 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
     随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。


*/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest = prices[0];
        let mut highest = prices[0];
        let mut profit_past = 0;
        let mut profit_curr = 0;
    
        for i in 1..prices.len() {
            if prices[i] >= highest { // 涨
                highest = prices[i];
                profit_curr = highest - lowest;
            } else { // 跌了
                lowest = prices[i];
                highest = prices[i];
                profit_past += profit_curr;
                profit_curr = 0;
            }
        }
    
        profit_curr + profit_past
    }
}
