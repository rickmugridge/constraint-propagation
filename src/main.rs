use std::cell::RefCell;
use std::rc::Rc;
use crate::adder::Adder;
use crate::connector::Connector;
use crate::multiplier::Multiplier;

mod connector;
mod constraint;
mod multiplier;
mod adder;
mod probe;

fn main() {
    println!("Hello, world!");
}

pub fn temperature_converter(c: Rc<RefCell<Connector>>, f: Rc<RefCell<Connector>>) {
    let u = Connector::new("U".to_string());
    let v = Connector::new("v".to_string());
    let w = Connector::new("w".to_string());
    let x = Connector::new("x".to_string());
    let y = Connector::new("y".to_string());
    Multiplier::new(&c, &w, &u);
    Multiplier::new(&v, &x, &u);
    Adder::new(&v, &y, &f);
    Connector::fix(&w, 9.0);
    Connector::fix(&x, 5.0);
    Connector::fix(&y, 32.0);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::adder::Adder;
    use crate::connector::Connector;
    use crate::multiplier::Multiplier;
    use crate::probe::Probe;
    use crate::temperature_converter;

    fn make_multiplier() -> (Rc<RefCell<Connector>>, Rc<RefCell<Connector>>, Rc<RefCell<Connector>>) {
        let in1 = Connector::new("in1".to_string());
        let in2 = Connector::new("in2".to_string());
        let product = Connector::new("product".to_string());
        Multiplier::new(&in1, &in2, &product);
        Probe::new(&in1);
        Probe::new(&in2);
        Probe::new(&product);
        (in1, in2, product)
    }

    #[test]
    fn multiplier_to_product() {
        let (in1, in2, product) = make_multiplier();
        Connector::fix(&in1, 9.0);
        Connector::set_value(&in2.clone(), Some(3.0));
        assert_eq!(product.clone().borrow().value, Some(27.0));
    }

    #[test]
    fn multiplier_to_in2() {
        let (in1, in2, product) = make_multiplier();
        Connector::fix(&in1, 9.0);
        Connector::set_value(&product.clone(), Some(27.0));
        assert_eq!(in2.clone().borrow().value, Some(3.0));
    }

    #[test]
    fn multiplier_to_in1() {
        let (in1, in2, product) = make_multiplier();
        Connector::fix(&in2, 9.0);
        Connector::set_value(&product.clone(), Some(27.0));
        assert_eq!(in1.clone().borrow().value, Some(3.0));
    }

    #[test]
    fn just_adder() {
        let in1 = Connector::new("in1".to_string());
        let in2 = Connector::new("in2".to_string());
        let sum = Connector::new("sum".to_string());
        Adder::new(&in1, &in2, &sum);
        Probe::new(&in1);
        Probe::new(&in2);
        Probe::new(&sum);
        Connector::fix(&in1, 9.0);
        Connector::set_value(&in2.clone(), Some(3.0));
        assert_eq!(sum.clone().borrow().value, Some(12.0));
    }

    #[test]
    fn temperature() {
        let c = Connector::new("c".to_string());
        let f = Connector::new("f".to_string());
        temperature_converter(c.clone(), f.clone());
        Probe::new(&c);
        Probe::new(&f);
        Connector::set_value(&c.clone(), Some(25.0));
        assert_eq!(c.clone().borrow().value, Some(25.0));
        assert_eq!(f.clone().borrow().value, Some(77.0));
        Connector::set_value(&c.clone(), None);
        Connector::set_value(&f.clone(), Some(212.0));
        assert_eq!(c.clone().borrow().value, Some(100.0));
    }
}