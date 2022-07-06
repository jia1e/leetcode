/*
 * @lc app=leetcode id=912 lang=rust
 *
 * [912] Sort an Array
 */

// @lc code=start
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(nums)
    }

    /// # 冒泡排序
    /// 比较相邻的元素。如果第一个比第二个大，就交换他们两个。
    /// 对每一对相邻元素作同样的工作，从开始第一对到结尾的最后一对。这步做完后，最后的元素会是最大的数。
    /// 针对所有的元素重复以上的步骤，除了最后一个。
    /// 持续每次对越来越少的元素重复上面的步骤，直到没有任何一对数字需要比较。
    pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            for j in 0..(nums.len() - i) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
        nums
    }

    /// # 选择排序
    /// 首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置。
    /// 再从剩余未排序元素中继续寻找最小（大）元素，然后放到已排序序列的末尾。
    /// 重复第二步，直到所有元素均排序完毕。
    pub fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() > 0 {
            let (mut l, mut r) = (0, nums.len() - 1);
            while l < r {
                for i in (l + 1)..=r {
                    if nums[i] < nums[l] {
                        nums.swap(i, l);
                    }
                }

                l += 1;

                for j in l..r {
                    if nums[j] > nums[r] {
                        nums.swap(j, r);
                    }
                }

                r -= 1;
            }
        }
        nums
    }

    /// # 插入排序
    /// 将第一待排序序列第一个元素看做一个有序序列，把第二个元素到最后一个元素当成是未排序序列。
    /// 从头到尾依次扫描未排序序列，将扫描到的每个元素插入有序序列的适当位置。（如果待插入的元素与有序序列中的某个元素相等，则将待插入元素插入到相等元素的后面。）
    pub fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            let mut j = i;
            while j > 0 {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                }
                j -= 1;
            }
        }

        nums
    }

    /// # 快速排序
    /// 从数列中挑出一个元素，称为 "基准"（pivot）;
    /// 重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边）。在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作；
    /// 递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序；
    pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len > 0 {
            Self::_quick_sort(&mut nums, 0, len);
        }
        nums
    }

    fn _quick_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
        Self::_partition(nums, start, end)
    }

    fn _partition(nums: &mut Vec<i32>, start: usize, end: usize) {
        let (mut pivot, mut index, mut divide) = (start, start + 1, start);
        while index < end {
            if nums[index] < nums[pivot] {
                nums.swap(index, divide);
                if pivot == divide {
                    pivot = index;
                }
                divide += 1;
            }
            index += 1;
        }

        nums.swap(divide, pivot);

        if divide > start + 1 {
            Self::_quick_sort(nums, start, divide)
        }

        if divide + 2 < end {
            Self::_quick_sort(nums, divide + 1, end);
        }
    }
}
// @lc code=end

pub struct Solution;

#[test]
fn test() {
    let cases = vec![
        (vec![], vec![]),
        (vec![0], vec![0]),
        (vec![5, 2, 3, 1], vec![1, 2, 3, 5]),
        (vec![5, 1, 1, 2, 0, 0], vec![0, 0, 1, 1, 2, 5]),
    ];

    for (input, output) in cases {
        assert_eq!(Solution::sort_array(input.clone()), output);
        assert_eq!(Solution::bubble_sort(input.clone()), output);
        assert_eq!(Solution::selection_sort(input.clone()), output);
        assert_eq!(Solution::insertion_sort(input.clone()), output);
        assert_eq!(Solution::quick_sort(input.clone()), output);
    }
}
