use std::collections::HashMap;

fn mean(list: &Vec<i32>, length: i32) -> i32 {
  let sum: i32 = list.iter().sum();
  sum / length
}

fn median_index_for(list: &Vec<i32>, index: usize) -> i32 {
  match list.get(index) {
    Some(&a) => a,
    None => 0,
  }
}

fn median(list: &Vec<i32>, length: usize) -> i32 {
  let rem = length % 2;
  let index = (length + rem) / 2 - 1;
  let median = median_index_for(list, index);

  if rem == 0 {
    let next_median = median_index_for(list, index + 1);
    (median + next_median) / 2
  }
  else
  {
    median
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

  let mean = mean(&list, length as i32);
  println!("Mean: {:?}", mean);

  let median = median(&list, length);
  println!("Median: {:?}", median);

  let mode = mode(&list);
  println!("Mode: {:?}", mode);
}
