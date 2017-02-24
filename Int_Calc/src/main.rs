use std::collections::HashMap;

// Calculate the average
fn average(x: &[u32]) -> f64 {
    let mut sum = 0.0;
    let mut ord = 0.0;
    for element in x.iter() {
        sum += f64::from(*element); // must dereference 'element' and convert to f64
        ord += 1.0;
    }
    sum/ord
}

fn median(x: &mut[u32]) -> f64 {
   x.sort();
   if x.len() % 2 == 1 {
      f64::from(x[(x.len() - 1) / 2])
   } else {
      f64::from((x[x.len() / 2] + x[(x.len() / 2) - 1]) / 2)
   }
}

fn mode(x: &[u32]) -> (u32, u32)  {
   let mut ind = 0;
   let mut max = 0;
   let mut map = HashMap::new();
   for element in x.iter() {
      let count = map.entry(*element).or_insert(0);
      *count += 1;
   }
   for (key, value) in map.iter() {
      if *value > max {
         max = *value;
         ind = *key;
      }
   }
   (ind, max)
}

fn main() {

    let mut v = vec![6, 17, 3, 4, 8, 9, 11, 2, 2, 2, 15, 840];
    let (num, freq) = mode(&v);

    println!("The average is {:?}.", average(&v));
    println!("The median is {:?}.", median(&mut v));
    println!("The mode is {:?}, which occurs {:?} times.", num, freq);
}
