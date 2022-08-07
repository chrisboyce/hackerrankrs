#[derive(Debug)]
struct TestCase {
    items: Vec<i32>,
    solutions: Option<Solution>,
}
#[derive(Debug)]
struct Solution {
    max_subarray_sum: i32,
    max_subsequence_sum: i32,
}
fn main() {
    let test_cases = parse_input();
    // dbg!(&test_cases);
    for test_case in test_cases {
        let max_subsequence_sum = max_subsequence(&test_case);
        // If the max is negative, then we won't be able to find a more
        // positive subarray then the subsequence
        let max_subarray_sum = if max_subsequence_sum > 0 {
            max_subarray(&test_case)
        } else {
            max_subsequence_sum
        };

        // println!("RESULT- {max_subarray_sum} {max_subsequence_sum}");
        println!("{max_subarray_sum} {max_subsequence_sum}");
    }
}

fn max_subarray(test_case: &TestCase) -> i32 {
    let mut max_sum = 0;
    let mut max_subsequence: &[i32] = &[];
    for i in 0..test_case.items.len() {
        for j in i..test_case.items.len() {
            let slice = &test_case.items[i..j + 1];
            let slice_sum = slice.iter().sum();
            // println!(
            //     "Slice index [{},{}] is [{:?}] has sum {}",
            //     i, j, &slice, slice_sum
            // );
            if slice_sum > max_sum {
                max_sum = slice_sum;
                max_subsequence = slice;
            }
        }
    }
    // dbg!(max_sum);
    // dbg!(max_subsequence);
    max_sum
}
fn max_subsequence(test_case: &TestCase) -> i32 {
    // degenerate case where all are negative
    let has_at_least_one_positive = test_case.items.iter().filter(|i| **i > 0).sum::<i32>() > 0;

    if has_at_least_one_positive {
        test_case.items.iter().filter(|i| **i > 0).sum::<i32>()
    } else {
        let mut max = std::i32::MIN;
        for item in &test_case.items {
            if item > &max {
                max = *item;
            }
        }
        max
    }
}
fn parse_input() -> Vec<TestCase> {
    let mut buffer = String::new();
    while let Ok(bytes_read) = std::io::stdin().read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
    }

    let lines: Vec<_> = buffer.split("\n").map(|s| s.trim()).collect();
    let test_count = lines[0].parse::<usize>().unwrap();

    let mut tests = Vec::new();
    for n in 0..test_count {
        // skip the first line (since it indicates the number of tests)
        let _array_size_index = 1 + n * 2;
        let array_index = 1 + n * 2 + 1;
        tests.push(TestCase {
            solutions: None,
            items: lines[array_index]
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        });
    }
    tests
}
