pub trait Sum {
    fn get_sum(&self) -> Option<u32>;
}

impl Sum for [u32] {
    fn get_sum(&self) -> Option<u32> {
        let mut a:u64 = 0;
        for v in self.iter() {
            a = a + *v as u64;
        }
        let b:u32 = a as u32;
        if a > u32::MAX as u64 {
            None
        } else {
            Some(b)
        }
    }
}

fn main() {
    let arr1: &mut [u32] = &mut [100;8];
    let arr2: &mut [u32] = &mut [1,2,3,4,5];
    let arr3: &mut [u32] = &mut [u32::MAX];
    let arr4: &mut [u32] = &mut [u32::MAX,1];
    println!("{:?}",arr1.get_sum());
    println!("{:?}",arr2.get_sum());
    println!("{:?}",arr3.get_sum());
    println!("{:?}",arr4.get_sum());

}
