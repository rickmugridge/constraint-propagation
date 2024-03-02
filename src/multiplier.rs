use std::cell::RefCell;
use std::rc::Rc;
use crate::adder::ConnectorAction;
use crate::connector::Connector;
use crate::constraint::Constraint;

pub struct Multiplier {
    in1: Rc<RefCell<Connector>>,
    in2: Rc<RefCell<Connector>>,
    product: Rc<RefCell<Connector>>,
}

impl Multiplier {
    pub fn new(in1: &Rc<RefCell<Connector>>,
               in2: &Rc<RefCell<Connector>>,
               product: &Rc<RefCell<Connector>>) -> Rc<RefCell<dyn Constraint>> {
        let myself:Rc<RefCell<dyn Constraint>> = Rc::new(RefCell::new(
            Self { in1: in1.clone(), in2: in2.clone(), product: product.clone() }));
        <dyn Constraint>::register(vec![in1, in2, product], &myself);
        myself
    }
}

impl Constraint for Multiplier {
    fn updated(&self, is_none: bool) {
        // println!("Update Adder {} from connector {}", self.name, connector);
        if is_none {
            <dyn Constraint>::clear(vec![&self.in1, &self.in2, &self.product]);
            return;
        }
        let mut action: Option<ConnectorAction> = None;
        if let Some(in1) = self.in1.borrow().value {
            if let Some(in2) = self.in2.borrow().value {
                action = Some(ConnectorAction::new(&self.product, Some(in1 * in2))); // todo <------ fails
            } else if let Some(product) = self.product.borrow().value {
                action = Some(ConnectorAction::new(&self.in2, Some(product / in1)));
            }
        } else if let Some(in2) = self.in2.borrow().value {
            if let Some(product) = self.product.borrow().value {
                action = Some(ConnectorAction::new(&self.in1, Some(product / in2)));
            }
        }
        if let Some(act) = action {
            Connector::set_value(act.connector, act.value);
        }
    }
}
