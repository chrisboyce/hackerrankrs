We define subsequence as any subset of an array. We define a subarray as a contiguous subsequence in an array.

Given an array, find the maximum possible sum among:

- all nonempty subarrays.
- all nonempty subsequences.

Print the two values as space-separated integers on one line.

Note that empty subarrays/subsequences should not be considered.

Example
The maximum subarray sum is comprised of elements at inidices . Their sum is . The maximum subsequence sum is comprised of elements at indices and their sum is

.

Function Description

Complete the maxSubarray function in the editor below.

maxSubarray has the following parameter(s):

    int arr[n]: an array of integers

Returns

    int[2]: the maximum subarray and subsequence sums

Input Format

The first line of input contains a single integer

, the number of test cases.

The first line of each test case contains a single integer
.
The second line contains space-separated integers where

.

Constraints

The subarray and subsequences you consider should have at least one element.