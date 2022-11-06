#[derive(Debug)]
enum AlgorithmKind {
    Bubble,
    Selection,
    Insertion,
    Merge,
}

struct SortingRequest {
    unsorted_array: Vec<i32>,
    sorted_array: Vec<i32>,
    algorithm: AlgorithmKind,
    steps: Vec<Vec<i32>>,
}

fn main() {
    // let fake_request = SortingRequest {
    //     unsorted_array: vec![-2, 45, 0, 11, -9],
    //     sorted_array: vec![],
    //     algorithm: AlgorithmKind::Bubble,
    //     steps: vec![],
    // };
    //
    // let other_fake_request = SortingRequest {
    //     unsorted_array: vec![20, 12, 10, 15, 2],
    //     sorted_array: vec![],
    //     algorithm: AlgorithmKind::Selection,
    //     steps: vec![],
    // };
    //
    // let insertion_request = SortingRequest {
    //     unsorted_array: vec![9, 5, 1, 4, 3],
    //     sorted_array: vec![],
    //     algorithm: AlgorithmKind::Insertion,
    //     steps: vec![],
    // };

    let merge_request = SortingRequest {
        unsorted_array: vec![6, 5, 12, 10, 9, 1],
        sorted_array: vec![],
        algorithm: AlgorithmKind::Merge,
        steps: vec![],
    };

    // sort_array(fake_request);
    // sort_array(other_fake_request);
    // sort_array(insertion_request);
    sort_array(merge_request);
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

fn merge_sort(unsorted_array: &[i32], p: usize, q: usize, r: usize) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut tmp_array = unsorted_array.to_owned();
    let mut steps = vec![];

    let halfway_point: usize = (tmp_array.len() as f32 / 2.0).floor() as usize;

    let l = tmp_array.slice();

    (tmp_array, steps)
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
            // (object.sorted_array, object.steps) = merge_sort(&object.unsorted_array);
        }
    }

    pretty_print_struct(&object)
}
