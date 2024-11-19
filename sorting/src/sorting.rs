// Simple bubble sort algo
pub fn bubble_sort(nums: &mut Vec<u8>) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                let temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            }
        }
    }
}

// Merge sort algo
fn merge(arr: &mut Vec<u8>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left + 1;
    let n2 = right - mid;

    // Initialize temp vectors with the required sizes
    let mut l = vec![0; n1];
    let mut r = vec![0; n2];

    for i in 0..n1 {
        l[i] = arr[left + i]
    }
    for i in 0..n2 {
        r[i] = arr[mid + 1 + i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    // Merge temp vectors back
    while i < n1 && j < n2 {
        if l[i] <= r[j] {
            arr[k] = l[i];
            i += 1;
        } else {
            arr[k] = r[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        arr[k] = l[i];
        i += 1;
        k += 1;
    }
    while j < n2 {
        arr[k] = r[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort(nums: &mut Vec<u8>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid: usize = left + (right - left) / 2;
    merge_sort(nums, left, mid);
    merge_sort(nums, mid + 1, right);
    merge(nums, left, mid, right)
}
