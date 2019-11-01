use std::time::Instant;

fn bubble_sort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr_vec = arr.to_vec();
    let mut _i = 0;

    for _ in 0..arr_vec.len() {
        let mut j = 0;
        let mut _same = false;
        for _i in 0..arr_vec.len() {
            if j != arr_vec.len() - 1 {
                _same = arr_vec[j] == arr_vec[j + 1];
                if _same == true {
                    arr_vec.swap_remove(j);
                    continue;
                }
                if arr_vec[j] > arr_vec[j + 1] {
                    let temp = arr_vec[j];
                    arr_vec[j] = arr_vec[j + 1];
                    arr_vec[j + 1] = temp;
                }
                j += 1;
                continue;
            } else {
                _same = arr_vec[j] == arr_vec[j - 1];
                j += 1;
            }
            continue;
        }
        _i += 1;
    }

    return arr_vec;
}

// Big O Notation : O(n)
fn finding_big_o(n: u32) -> u32 {
    let res = n * (n + 1) / 2;
    return res;
}

fn main() {
    println!("=== Bubble Sort ===");
    let start_bs = Instant::now();
    let elapsed_bs = start_bs.elapsed();
    let b_payload = vec![
        321, 23004, 2112, 12, 321, 100, 121, 121, 22, 1231, 220000012, 100201, 22, 220000012, 100,
        2112, 121, 22, 1231, 220000012, 100201, 22, 220000012, 100, 2112, 121, 22, 1231, 220000012,
        100201, 22, 220000012, 321, 100, 121, 1, 22, 220000012, 1, 22, 220000012,
    ];
    println!("{:?}", bubble_sort(&b_payload));
    println!("End: {:?}", elapsed_bs);

    println!("=== Bign O(n) ===");
    let start_bo = Instant::now();
    let elapsed_bo = start_bo.elapsed();
    println!("{:?}", &finding_big_o(100));
    println!("{:?}", elapsed_bo);
}
