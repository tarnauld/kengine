use input::keys::Keys;
use std::collections::HashMap;

pub struct Keyboard{
    h: HashMap<Keys, ()>
}

impl Keyboard{
    pub fn keyboard_settings() -> Keyboard{
        Keyboard{
            h: HashMap::new()
        }
    }
    pub fn binding(&mut self, k: Keys, f : ()){
        self.h.insert(k, f);
    }
}

#[cfg(test)]
mod test{
    use input::keyboard::Keyboard;
    use input::keyboard::Keys;
    #[test]
    fn should_insert_function(){
        let mut k: Keyboard = Keyboard::keyboard_settings();
        let key = Keys::A;
        let mut size = 0;
        let closure = println!("Hello");
        k.binding(key, closure);
        for(_k, f) in k.h{
            f;
            size += 1;
        }
        assert_eq!(size, 1);
    }
}
