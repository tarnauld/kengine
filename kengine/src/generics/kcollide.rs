use generics::kcoord::Kcoord;

pub enum WindowBorders{
    RIGHT,
    LEFT,
    DOWN,
    UP,
    CENTER
}

pub fn collide_with_window(a: &Kcoord, w: f64, h: f64) -> WindowBorders{
    if a.x < 0. {return WindowBorders::LEFT;}
    if a.x > w {return WindowBorders::RIGHT;}
    if a.y < 0. {return WindowBorders::UP;}
    if a.y > h {return WindowBorders::DOWN;}
    return WindowBorders::CENTER;
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
