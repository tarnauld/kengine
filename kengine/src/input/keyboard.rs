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
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn should_insert_function(){
        let mut k: Keyboard = Keyboard::keyboard_settings();
        let key = Keys::A;
        let mut rc = Rc::new(RefCell::new(0));
        let t = |s: Rc<RefCell<i32>>| test(s.clone());
        k.binding(key, t(rc.clone()));

        for(_k, f) in k.h{
            f;
        }

        assert_eq!(*Rc::get_mut(&mut rc).unwrap().borrow(), 1);
    }
    fn test(s: Rc<RefCell<i32>>){
        *s.borrow_mut() = 1;
    }
}
