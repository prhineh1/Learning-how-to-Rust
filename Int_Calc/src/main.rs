use std::collections::HashMap;

// Calculates the average
fn average(x: &[u32]) -> f64 {
    let mut sum = 0.0;
    let mut ord = 0.0;
    for element in x.iter() {
        sum += f64::from(*element);
        ord += 1.0;
    }
    sum/ord
}
//calculates the median
fn median(x: &mut[u32]) -> f64 {
   x.sort();
   //if x's length is odd, the midpoint is the quotient
   //of the penultimate index divded by 2.
   if x.len() % 2 == 1 {
      f64::from(x[(x.len() - 1) / 2])
   //if even, we take the average of the two mid points
   } else {
      average(&vec![x[x.len()/2], x[(x.len()/2)-1]])
   }
}
//calculates the mode
fn mode(x: &[u32]) -> (u32, u32)  {
   let mut ind = 0;
   let mut max = 0;
   //Create a map whose keys are elements of x
   //and whose values are their frequency
   let mut map = HashMap::new();
   for element in x.iter() {
      let count = map.entry(*element).or_insert(0);
      *count += 1;
   }
   //Find the most frequent key in our map; return it
   //and its corresponding value as a duple
   for (key, value) in map.iter() {
      if *value > max {
         max = *value;
         ind = *key;
      }
   }
   (ind, max)
}

fn main() {

    let mut v = vec![0];
    let (num, freq) = mode(&v);

    println!("The average is {:?}.", average(&v));
    println!("The median is {:?}.", median(&mut v));

    if freq > 1 {
      println!("The mode is {:?}, which occurs {:?} times.", num, freq);
   } else {
      println!("There is no mode");
   }

}
