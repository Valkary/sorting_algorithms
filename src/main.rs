use std::cmp::Ordering;

#[derive(Debug)]
enum AlgorithmKind {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
}

struct SortingRequest {
    unsorted_array: Vec<i32>,
    sorted_array: Vec<i32>,
    algorithm: AlgorithmKind,
    steps: Vec<Vec<i32>>,
}

fn main() {
    let bubble_request = SortingRequest {
        unsorted_array: vec![-2, 45, 0, 11, -9],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Bubble,
        steps: vec![],
    };

    let selection_request = SortingRequest {
        unsorted_array: vec![20, 12, 10, 15, 2],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Selection,
        steps: vec![],
    };

    let insertion_request = SortingRequest {
        unsorted_array: vec![9, 5, 1, 4, 3],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Insertion,
        steps: vec![],
    };

    let merge_request = SortingRequest {
        unsorted_array: vec![6, 5, 12, 10, 9, 1],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Merge,
        steps: vec![],
    };

    let quick_request = SortingRequest {
        unsorted_array: vec![8, 7, 6, 1, 0, 9, 2],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Quick,
        steps: vec![],
    };

    sort_array(bubble_request);
    sort_array(selection_request);
    sort_array(insertion_request);
    sort_array(merge_request);
    sort_array(quick_request);
}

fn bubble_sort(unsorted_array: &[i32]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut tmp_array = unsorted_array.to_owned();
    let mut steps = vec![];
    for i in 0..(tmp_array.len() - 1) {
        for j in 0..(tmp_array.len() - i - 1) {
            if tmp_array[j] > tmp_array[j + 1] {
                tmp_array.swap(j, j + 1);
                steps.push(tmp_array.clone());
            }
        }
    }
    (tmp_array, steps)
}

fn selection_sort(unsorted_array: &[i32]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut tmp_array = unsorted_array.to_owned();
    let mut steps = vec![];
    let mut min: usize;

    for i in 0..(tmp_array.len()) {
        min = i;

        for j in (i + 1)..(tmp_array.len()) {
            if tmp_array[j] < tmp_array[min] {
                min = j;
            }
        }

        if min != i {
            tmp_array.swap(i, min);
            steps.push(tmp_array.clone());
        }
    }

    (tmp_array, steps)
}

fn insertion_sort(unsorted_array: &[i32]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut tmp_array = unsorted_array.to_owned();
    let mut steps = vec![];

    for i in 1..tmp_array.len() {
        let mut j = i;

        while j > 0 && tmp_array[j] < tmp_array[j - 1] {
            tmp_array.swap(j, j - 1);
            steps.push(tmp_array.clone());
            j -= 1;
        }
    }

    (tmp_array, steps)
}

fn merge_sort(unsorted_array: &[i32]) -> Vec<i32> {
    // TODO: somehow represent the steps taken to merge sort an array
    let tmp_array = unsorted_array.to_owned();

    if tmp_array.len() == 1 {
        tmp_array
    } else {
        let size = tmp_array.len() / 2;
        let left = merge_sort(&tmp_array[0..size]);
        let right = merge_sort(&tmp_array[size..]);

        merge(&left, &right)
    }
}

fn quick_sort(arr: &Vec<i32>) -> Vec<i32> {
    let tmp_array = arr.to_owned();

    if arr.len() < 2 {
        tmp_array
    } else {
        let middle_index = tmp_array.len() / 2;
        let pivot = tmp_array[middle_index];

        let mut lesser = vec![];
        let mut equal = vec![];
        let mut greater = vec![];

        for element in tmp_array {
            match element.cmp(&pivot) {
                Ordering::Greater => greater.push(element),
                Ordering::Equal => equal.push(element),
                Ordering::Less => lesser.push(element),
            }
        }

        let left = quick_sort(&lesser);
        let right = quick_sort(&greater);

        [&left[..], &equal[..], &right[..]].concat()
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = vec![];

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    } else if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}

fn pretty_print_struct(object: &SortingRequest) {
    let SortingRequest {
        unsorted_array,
        sorted_array,
        algorithm,
        steps,
    } = object;

    println!("SortObject = {{\n    unsorted_array: {:?},\n    sorted_array: {:?},\n    algorithm: {:?},\n    steps: {:?}\n}}",
             unsorted_array,
             sorted_array,
             algorithm,
             steps);
}

fn sort_array(mut object: SortingRequest) {
    match object.algorithm {
        AlgorithmKind::Bubble => {
            (object.sorted_array, object.steps) = bubble_sort(&object.unsorted_array)
        }
        AlgorithmKind::Selection => {
            (object.sorted_array, object.steps) = selection_sort(&object.unsorted_array)
        }
        AlgorithmKind::Insertion => {
            (object.sorted_array, object.steps) = insertion_sort(&object.unsorted_array);
        }
        AlgorithmKind::Merge => {
            object.sorted_array = merge_sort(&object.unsorted_array);
        }
        AlgorithmKind::Quick => {
            object.sorted_array = quick_sort(&object.unsorted_array);
        }
    }

    pretty_print_struct(&object)
}
