/*

给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

说明：

你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？

作者：力扣 (LeetCode)
链接：https://leetcode-cn.com/leetbook/read/top-interview-questions-easy/x21ib6/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

示例 2:

输入: [4,1,2,1,2]
输出: 4

*/

use std::ops::BitXor;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(0, BitXor::bitxor)
    }
}
      
// 相较于闭包 内存占用更小……
