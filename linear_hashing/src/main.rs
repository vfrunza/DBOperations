use rand::Rng;

struct LinearHash {
    hash_digit: u64,
    hash_mask: u64,
    threshold: f64,
    size: u64,
    bucket_size: u64,
    bucket_count: u64,
    buckets: Vec<Bucket>
}

#[derive(Debug)]
struct Bucket {
    key: u64,
    values: Vec<u64>
}

impl LinearHash {
    fn new() -> LinearHash {
        let mut b0 = Bucket{key: 0, values: Vec::new()};
        let mut b1 = Bucket{key: 1, values: Vec::new()};

        let mut lh = LinearHash {
            hash_digit: 1,
            hash_mask: 1,
            threshold: 0.85,
            size: 0, 
            bucket_size: 2,
            bucket_count: 2,
            buckets: Vec::new()
        };
        lh.buckets.push(b0);
        lh.buckets.push(b1);
        return lh;  
    }

    pub fn display(&self){
        println!("=== Data Structure State ===============================================");
        println!("hash_digit: {}", self.hash_digit);
        println!("size: {}", self.size);
        println!("bucket_count: {}", self.bucket_count);

        for i in 0..self.buckets.len() {
            println!("Bucket {}: key = {:#05b}", i, self.buckets[i].key);
            print!("\tRegular:  [");
            for j in 0..self.buckets[i].values.len() {
                if j == 0{
                    print!("");
                }
                else if j == 2 {
                    print!("]\n\tOverflow: [");
                } 
                else{
                    print!(", ");
                }
                print!("{}", self.buckets[i].values[j]);
            }
            println!("]")
        }
    }

    fn hash(&mut self, value: u64) -> u64 {
        let hash_value = value & self.hash_mask;

        println!("=== HASHING ===");
        println!("Hashing digit - {}", self.hash_digit);
        println!("Inputed value - {:#014b}", value);
        println!("Hashing mask  - {:#014b}", self.hash_mask);
        println!("Hash value    - {:#014b}", hash_value);
        return hash_value;
    }

    pub fn add(&mut self, value: u64){
        println!("=== ADDING =============================================================");
        // hashing value
        let hash_value = self.hash(value);
        let mut bucket_index:usize = 0;

        // determine bucket
        if hash_value < self.bucket_count{
            bucket_index = hash_value as usize;

        }
        else{
            let i: u64 = hash_value - u64::pow(2, (self.hash_digit - 1) as u32);
            bucket_index = i as usize;
        }

        // add to buckets
        self.buckets[bucket_index].values.push(value);

        // iterate state
        self.size += 1;

        println!("Added {} to bucket {}", value, bucket_index);

        // split on threshold
        let capacity: f64 = self.size as f64 / (self.bucket_size as f64 * self.bucket_count as f64);
        println!("Capacity is {}", capacity);
        if capacity > self.threshold {
            println!("Threshold exceded, creating new bucket...");
            self.create_bucket();
        }

    }

    fn create_bucket(&mut self){
        // increment ds variables
        self.bucket_count += 1;

        //add new bucket
        let mut b = Bucket{key: self.bucket_count - 1, values: Vec::new()};
        self.buckets.push(b);

        // change bucket hash width, redistribute the values
        if self.is_power_of_two(self.bucket_count - 1){
            self.hash_digit += 1;
            self.update_hash_mask();
        }

        self.redistribute();
    }

    fn redistribute(&mut self){
        println!("=== REDISTRIBUTING ===");
        self.display();
        let mut v: Vec<Vec<u64>> = Vec::new();

        // create buckets
        for i in 0..self.bucket_count {
            v.push(Vec::new());
        }

        for i in 0..self.buckets.len() {
            for j in 0..self.buckets[i].values.len() {
                // hashing value
                let hash_value = self.hash(self.buckets[i].values[j]);
                let mut bucket_index:usize = 0;

                // determine bucket
                if hash_value < self.bucket_count{
                    bucket_index = hash_value as usize;

                }
                else{
                    let n: u64 = hash_value - u64::pow(2, (self.hash_digit - 1) as u32);
                    bucket_index = n as usize;
                }

                // add to buckets
                v[bucket_index].push(self.buckets[i].values[j]);

                println!("Added {} to bucket {}", self.buckets[i].values[j], bucket_index);
            }
        }

        // clear buckets
        for i in 0..self.buckets.len() {
            self.buckets[i].values.clear();
            for j in 0..v[i].len(){
                self.buckets[i].values.push(v[i][j]);
             }
        }

        self.display();
    }

    fn update_hash_mask(&mut self) {
        let mut mask: u64 = self.hash_mask << 1;
        if mask > 1{
            mask += 1;
        }
        self.hash_mask = mask;
    }

    fn is_power_of_two(&self, x: u64) -> bool {
        return (x != 0) && ((x & (x - 1)) == 0);
    }
}

fn main(){
    println!("Welcome to Linear Hashing Simulator!");

    let mut rng = rand::thread_rng();
    let mut lh = LinearHash::new();

    lh.display();

    lh.add(0);
    lh.add(15);
    lh.add(8);
    lh.add(4);
    lh.add(7);
    lh.add(12);
    lh.add(10);
    lh.add(11);

    //for i in 0..200{
    //    lh.add(rng.gen_range(0..1000) as u64);
    //}

    lh.display();
}