pub trait Dormir {
    fn dormir(&self, duration: u64) {
        println!("Dormindo por {} segundos", duration);
    }
}