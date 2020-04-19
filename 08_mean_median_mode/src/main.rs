use std::collections::HashMap;

fn mean(list: &Vec<i32>, length: usize) -> i32 {
  let sum: i32 = list.iter().sum();
  sum / (length as i32)
}

fn median(list: &Vec<i32>, length: usize) -> i32 {
  if length % 2 == 0 {
    let median_index = length / 2 - 1;
    let median =
      match list.get(median_index) {
        Some(&a) => a,
        None => 0,
      };

    let next_median =
      match list.get(median_index + 1) {
        Some(&a) => a,
        None => 0,
      };
    (median + next_median) / 2
  }
  else
  {
    let median_index = (length + 1) / 2 - 1;
    match list.get(median_index) {
      Some(&a) => a,
      None => 0,
    }
  }
}

fn mode(list: &Vec<i32>) -> i32 {
  let mut map = HashMap::new();

  for i in list.iter() {
    let count = map.entry(i).or_insert(0);
    *count += 1;
  }

  let result =
    map
    .iter()
    .max_by_key(|(_, v)| *v);

  match result {
    Some((k, _v)) => **k,
    None => 0,
  }
}

fn main() {
  let mut list = vec![1, 20, 10, 59, 10, 40, 100];
  println!("List {:?}", list);

  let length = list.len();
  list.sort();

  let mean = mean(&list, length);
  println!("Mean: {:?}", mean);

  let median = median(&list, length);
  println!("Median: {:?}", median);

  let mode = mode(&list);
  println!("Mode: {:?}", mode);
}
