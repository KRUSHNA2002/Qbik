// 1. Check if a string is palindrome

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// 2. Find index of first occurrence of a number in sorted array

fn find_index(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// 3. Find shortest word in a string

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

// 4. Check if a number is prime

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// 5. Find median of sorted array

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 6. Find longest common prefix of strings

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    let mut char_iterators: Vec<_> = strings.iter().map(|s| s.chars()).collect();
    let first_word_chars: Vec<_> = char_iterators.remove(0).collect();
    'outer: for (i, c) in first_word_chars.iter().enumerate() {
        for char_iter in &char_iterators {
            match char_iter.next() {
                Some(next_c) if next_c == *c => continue,
                _ => break 'outer,
            }
        }
        prefix.push(*c);
    }
    prefix
}

// 7. Find kth smallest element in array

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    Some(sorted_arr[k - 1])
}

// 8. Find maximum depth of binary tree

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// 9. Reverse a string

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime

fn is_prime_rust(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// 11. Merge two sorted arrays

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

// 12. Find the maximum subarray sum

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Testing the functions
    println!("Is 'radar' a palindrome? {}", is_palindrome("radar"));
    println!("Index of first occurrence of 3: {:?}", find_index(&[1, 2, 3, 4, 5], 3));
    println!("Shortest word: {:?}", shortest_word("This is a test string"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median of [1, 2, 3, 4, 5]: {}", median(&[1, 2, 3, 4, 5]));
    println!(
        "Longest common prefix: {:?}",
        longest_common_prefix(&["flower", "flow", "flight"])
    );
    println!("3rd smallest element: {:?}", kth_smallest(&[1, 3, 5, 2, 4], 3));
    println!("Maximum depth of binary tree: {:?}", max_depth(None));
    println!("Reverse of 'hello': {}", reverse_string("hello"));
    println!("Is 29 prime? {}", is_prime_rust(29));
    println!(
        "Merged arrays: {:?}",
        merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6])
    );
    println!("Maximum subarray sum: {:?}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}
