use generics::kcoord::Kcoord;

pub fn collide_with_window(a: &mut Kcoord, w: f64, h: f64, s: f64){
    if a.x * s < 0. {a.x = w / s;}
    if a.x * s > w {a.x = 0.;}
    if a.y * s < 0. {a.y = h / s;}
    if a.y * s > h {a.y = 0.;}
}

pub fn collide_with_part(a: &Kcoord, b: &Vec<Kcoord>) -> i32{
    for i in 1..b.len(){
        if a.x == b[i].x && a.y == b[i].y{
            return i as i32;
        }
    }
    -1
}

pub fn collision_exists(a: &Kcoord, b: &Kcoord) -> bool{
    if a.x == b.x && a.y == b.y{
        return true;
    }
    false
}
