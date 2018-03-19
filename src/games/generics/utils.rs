use rand;
use rand::Rng;

pub fn choose_random(w: i64, h: i64, s: i64) -> (i64, i64){
    (rand::thread_rng().gen_range(0, w / s),
    rand::thread_rng().gen_range(0, h / s))
}
