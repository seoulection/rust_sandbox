pub mod one {
    pub fn mean(num_list: &[i32]) -> f64 {
        let mut count = 0;
        let v = num_list.to_vec();
        for i in &v {
            count += i;
        }

        (count as f64) / (v.len() as f64)
    }
}
