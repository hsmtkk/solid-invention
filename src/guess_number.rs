enum Operator {
    GreaterThan,
    LessThan,
    DividedBy,
}

struct Condition {
    operator: Operator,
    operand: i32,
}

impl Condition {
    fn new(operator:Operator, operand:i32) -> Condition {
        Condition{operator, operand}
    }

    fn satisfy(&self, number:i32) -> bool {
        match self.operator {
            Operator::GreaterThan => {
                number > self.operand
            },
            Operator::LessThan => {
                number < self.operand
            },
            Operator::DividedBy => {
                number % self.operand == 0
            },
        }
    }
}

fn guess_number(conditions:&[Condition]) -> i32 {
    for i in 1..101 {
        if !satisfy_conditions(conditions, i){
            continue;
        }
        return i;
    }
    return 0;
}

fn satisfy_conditions(conditions:&[Condition], number:i32) -> bool {
    for c in conditions {
        if !c.satisfy(number){
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::{Condition, Operator};
    #[test]
    fn test0(){
        let conditions = vec![
            Condition::new(Operator::GreaterThan, 30),
            Condition::new(Operator::LessThan, 40),
            Condition::new(Operator::DividedBy, 5),
        ];
        let want = 35;
        let got = super::guess_number(&conditions);
        assert_eq!(want,got);
    }
    #[test]
    fn test1(){
        let conditions = vec![
            Condition::new(Operator::DividedBy, 4),
            Condition::new(Operator::LessThan, 90),
            Condition::new(Operator::DividedBy, 6),
            Condition::new(Operator::GreaterThan, 77),
        ];
        let want = 84;
        let got = super::guess_number(&conditions);
        assert_eq!(want,got);
    }
}