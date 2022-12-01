use std::fs;

fn main() {
    let contents = fs::read_to_string("../../TestCases/text_example.in").expect("Should have been able to read the file");
    let mut temp_sum: i32 = 0;
    let mut sums = vec![]; 

    for s in contents.lines() {
       match s {
            "" => { 
                sums.push(temp_sum);
	        temp_sum = 0;
	    },
	    _ => { 
                temp_sum += s.parse::<i32>().unwrap();
	    },
       }
    }

   let max_value = sums.iter().max().unwrap();
   println!("Solution 1: {}", max_value);

   sums.sort();
   sums.reverse();
   println!("Solution 2: {}", sums[0] + sums[1] + sums[2]);
   println!("{:?}", sums); 


}
