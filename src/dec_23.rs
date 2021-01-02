use crate::common;

#[derive(Debug, Clone)]
struct Element {
    next: usize,
    prev: usize,
}

#[inline(always)]
fn wrap(i: i64, max_u: usize) -> usize {
    let max = max_u as i64;
    (((i % max) + max) % max) as usize
}

fn init(arr: &[usize]) -> Vec<Element> {
    let len = arr.len();
    let mut elements: Vec<Element> = vec![Element { prev: 0, next: 0 }; len];
    for (i, v) in arr.iter().enumerate() {
        let index = wrap(*v as i64 - 1, len);
        let next = arr[wrap(i as i64 + 1, len)] - 1;
        let prev = arr[wrap(i as i64 - 1, len)] - 1;
        elements[index] = Element { next, prev };
    }
    elements
}

pub fn part_one() {
    println!("--- Part One ---");

    let numbers = common::read_strings("./data/dec_23.txt")[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let cups = run(&numbers, 100);

    let mut r = 0;
    let mut result_vec: Vec<usize> = vec![];
    for _ in 0..(cups.len() - 1) {
        r = cups[r].next;
        result_vec.push(r);
    }

    let result = result_vec.iter().map(|r| (r + 1).to_string()).collect::<Vec<String>>().join("");
    println!("Result: {}", result);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let mut arr = vec![1, 5, 6, 7, 9, 4, 8, 2, 3];
    for n in (arr.len() + 1)..=1_000_000 {
        arr.push(n);
    }

    let cups = run(&arr, 10_000_000);

    let r1 = cups[0].next;
    let r2 = cups[r1].next;

    println!("Result: {}", common::format_to_product(&[r1 as i32 + 1, r2 as i32 + 1]));
}

fn run(arr: &[usize], iterations: usize) -> Vec<Element> {
    let mut cups = init(&arr);

    let mut c0 = arr[0] - 1;

    for _ in 0..iterations {
        let c1 = cups[c0].next;
        let c2 = cups[c1].next;
        let c3 = cups[c2].next;
        let c4 = cups[c3].next;

        cups[c0].next = c4;
        cups[c4].prev = c0;

        let mut dest = wrap(c0 as i64 - 1, cups.len());
        while dest == c1 || dest == c2 || dest == c3 {
            dest = wrap(dest as i64 - 1, cups.len());
        }
        let dest_next = cups[dest].next;

        cups[dest].next = c1;
        cups[c1].prev = dest;
        cups[dest_next].prev = c3;
        cups[c3].next = dest_next;
        c0 = c4;
    }
    cups
}
