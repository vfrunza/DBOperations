struct LinearHash {
    hash_digit: u64,
    hash_mask: u64,
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
            buckets: Vec::new()
        };
        lh.buckets.push(b0);
        lh.buckets.push(b1);
        return lh;  
    }

    pub fn display(&self){
        println!("=== Data Structure State ===============================================");
        for i in 0..self.buckets.len() {
            if i < 2 {
                println!("Bucket    {}: key = {:#05b}, values = {:?}", i, self.buckets[i].key, self.buckets[i].values);
            } else {
                println!("OVERFLOW  {}: key = {:#05b}, values = {:?}", i, self.buckets[i].key, self.buckets[i].values);
            }
        }
    }

    fn hash(&mut self, value: u64) -> u64 {
        let hash_value = value & self.hash_mask;

        println!("=== HASHING ============================================================");
        println!("Inputed value - {:#014b}", value);
        println!("Hashing digit - {:#014b}", self.hash_digit);
        println!("Hashing mask  - {:#014b}", self.hash_mask);
        println!("Hash value    - {:#014b}", hash_value);
        return hash_value;
    }

    pub fn add(&mut self, value: u64){
        // hashing value
        println!("=== ADDING ============================================================");
        let hash_value = self.hash(value);

        // determine bucket

        // add to bucket

        // split on threshold
    }

    fn update_hash_mask(&mut self) {
        let mut mask: u64 = self.hash_mask << 1;
        if mask > 1{
            mask += 1;
        }
        self.hash_mask = mask;
    }
}

fn main(){
    println!("Welcome to Linear Hashing Simulator!");
    let mut lh = LinearHash::new();

    lh.display();

    lh.add(1);

    lh.display();
}