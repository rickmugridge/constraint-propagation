use std::cell::RefCell;
use std::rc::Rc;
use crate::connector::Connector;
use crate::constraint::Constraint;

pub struct Adder {
    in1: Rc<RefCell<Connector>>,
    in2: Rc<RefCell<Connector>>,
    sum: Rc<RefCell<Connector>>,
}

impl Adder {
    pub fn new(in1: &Rc<RefCell<Connector>>,
               in2: &Rc<RefCell<Connector>>,
               sum: &Rc<RefCell<Connector>>) -> Rc<RefCell<dyn Constraint>> {
        let myself: Rc<RefCell<dyn Constraint>> = Rc::new(RefCell::new(
            Self { in1: in1.clone(), in2: in2.clone(), sum: sum.clone() }));
        <dyn Constraint>::register(vec![in1, in2, sum], &myself);
        myself
    }
}

impl Constraint for Adder {
    fn updated(&self, is_none: bool) {
        // println!("Update Adder {} from connector {}", self.name, connector);
        if is_none {
            <dyn Constraint>::clear(vec![&self.in1, &self.in2, &self.sum]);
            return;
        }
        let mut action: Option<ConnectorAction> = None;
        if let Some(in1) = self.in1.borrow().value {
            if let Some(in2) = self.in2.borrow().value {
                action = Some(ConnectorAction::new(&self.sum, Some(in1 + in2))); // todo <------ fails
            } else if let Some(sum) = self.sum.borrow().value {
                action = Some(ConnectorAction::new(&self.in2, Some(sum - in1)));
            }
        } else if let Some(in2) = self.in2.borrow().value {
            if let Some(sum) = self.sum.borrow().value {
                action = Some(ConnectorAction::new(&self.in1, Some(sum - in2)));
            }
        }
        if let Some(act) = action {
            Connector::set_value(act.connector, act.value);
        }
    }
}

pub struct ConnectorAction<'a> {
    pub connector: &'a Rc<RefCell<Connector>>,
    pub value: Option<f64>,
}

impl<'a> ConnectorAction<'a> {
    pub fn new(connector: &'a Rc<RefCell<Connector>>,
           value: Option<f64>) -> Self {
        Self { connector, value }
    }
}
