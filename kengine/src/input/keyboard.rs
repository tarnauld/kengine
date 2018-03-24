use input::kies::Kies;
use std::collections::HashMap;

pub struct Keyboard{
    h: HashMap<Kies, fn()>
}

impl Keyboard{
    pub fn keyboard_settings() -> Keyboard{
        Keyboard{
            h: HashMap::new()
        }
    }
    pub fn binding(&mut self, k: Kies, f : fn()){
        self.h.insert(k, f);
    }
}

#[cfg(test)]
mod test{
    use input::keyboard::Keyboard;
    use input::keyboard::Kies;
    #[test]
    fn should_insert_function(){
        let mut k: Keyboard = Keyboard::keyboard_settings();
        let key = Kies::A;
        let mut size = 0;
        let closure = || println!("Hello");
        k.binding(key, closure);
        for(_k, f) in k.h{
            f();
            size += 1;
        }
        assert_eq!(size, 1);
    }
}
