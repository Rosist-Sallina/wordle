pub mod select{
    use rand::Rng;

    pub fn get_useable_word() -> String{
        let v = vec!["Apple" , "Pineapple"];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..v.len());
        return v[index].to_string();
    }
    pub fn get_available_word() -> String{
        let v = vec!["Apple" , "Pineapple"];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..v.len());
        return v[index].to_string();
    }
}