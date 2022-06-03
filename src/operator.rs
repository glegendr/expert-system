#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
        Or,
        Not,
        Xor,
        Material,
        Equal,
        B(u8),
        L(char)
}

/*
 * AB|C&!
 * 
 *      !
 *     / 
 *    &
 *   / \
 *  C   |
 *     / \
 *    A   B
 * 
 * 
 * DAC=B|&!
 * 
 *    !
 *    |
 *    &
 *   / \
 *  D   |
 *     / \
 *    B   =
 *       / \
 *      A   C
 */

#[derive(Debug, Clone)]
pub struct BTree {
    pub c1: Option<Box<BTree>>,
        pub c2: Option<Box<BTree>>,
        pub node: Operator
}

impl BTree {
    pub fn new(node: Operator) -> BTree {
        BTree {
c1: None,
        c2: None,
        node,
        }
    }

    pub fn not(sub_tree: BTree) -> BTree {
        BTree {
c1: None,
        c2: Some(Box::new(sub_tree)),
        node: Operator::Not
        }
    }

    pub fn create(node: Operator, c1: BTree, c2: BTree) -> BTree {
        BTree {
            node,
                c1: Some(Box::new(c1)),
                c2: Some(Box::new(c2))
        }
    }

    pub fn insert_a(&mut self, sub_tree: BTree) {
        self.c1 = Some(Box::new(sub_tree));
    }

    pub fn insert_b(&mut self, sub_tree: BTree) {
        self.c2 = Some(Box::new(sub_tree));
    }

    pub fn to_string(&self) -> String {
        match (&self.node, &self.c1, &self.c2) {
            (Operator::And, Some(c1), Some(c2)) => format!("{}{}&", c1.to_string(), c2.to_string()),
                (Operator::Or, Some(c1), Some(c2)) => format!("{}{}|", c1.to_string(), c2.to_string()),
                (Operator::Xor, Some(c1), Some(c2)) => format!("{}{}^", c1.to_string(), c2.to_string()),
                (Operator::Equal, Some(c1), Some(c2)) => format!("{}{}=", c1.to_string(), c2.to_string()),
                (Operator::Material, Some(c1), Some(c2)) => format!("{}{}>", c1.to_string(), c2.to_string()),
                (Operator::Not, Some(c1), None) => format!("{}!", c1.to_string()),
                (Operator::Not, None, Some(c2)) => format!("{}!", c2.to_string()),
                (Operator::B(b), _, _) => format!("{}", b),
                (Operator::L(l), _, _) => format!("{}", l),
                _ => format!("{self:?}")
        }
    }
    pub fn to_cnfstring(&self) -> String {
        let tree = self.to_string();
        let mut op: (Option<char>, usize) = (None, 0);
        for c in tree.chars() {
            match c {
                '|' | '&' => {
                    if let Some(curr_op) = op.0 {
                        if curr_op == c {
                            op.1 += 1;
                        } else {
                            return tree
                        }
                    } else {
                        op = (Some(c), 1);
                    }
                },
                    _ => ()
            }
        }
        if let Some(op_c) = op.0 {
            let mut new_tree = tree.replace(op_c, "");
            for _ in 0..op.1 {
                new_tree.push(op_c);
            }
            return new_tree
        }
        tree
    }
}


pub fn eval_formula(formula: &str) -> bool {
    match to_tree(formula) {
        Ok(tree) => {
            match calc_formula(&Box::new(tree)) {
                Ok(res) => res,
                    Err(e) => {
                        println!("{e}");
                        false
                    }
            }
        },
            Err(e) => {
                println!("{e}");
                false
            }
    }
}

pub fn to_tree(formula: &str) -> Result<BTree, String> {
    let mut new_formula = String::from(formula);
    create_tree(&mut new_formula)
}

fn create_tree(formula: &mut String) -> Result<BTree, String> {
    if let Some(last_c) = formula.pop() {
        let mut ret = match last_c {
            '&' => BTree::new(Operator::And),
                '|' => BTree::new(Operator::Or),
                '^' => BTree::new(Operator::Xor),
                '>' => BTree::new(Operator::Material),
                '=' => BTree::new(Operator::Equal),
                '!' => BTree::new(Operator::Not),
                '1' => return Ok(BTree::new(Operator::B(1))),
                '0' => return Ok(BTree::new(Operator::B(0))),
                _ => return Ok(BTree::new(Operator::L(last_c)))
        };
        ret.insert_b(create_tree(formula)?);
        if last_c != '!' {
            ret.insert_a(create_tree(formula)?);
        }
        return Ok(ret)
    }
    Err(String::from("Error while parsing formula"))
}

fn calc_formula(tree: &Box<BTree>) -> Result<bool, String> {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::And, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? & calc_formula(&c2)?),
            (Operator::Or, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? | calc_formula(&c2)?),
            (Operator::Xor, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? ^ calc_formula(&c2)?),
            (Operator::Material, Some(c1), Some(c2)) => Ok(!(calc_formula(&c1)? && !calc_formula(&c2)?)),
            (Operator::Equal, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? == calc_formula(&c2)?),
            (Operator::Not, Some(c1), None) => Ok(!calc_formula(&c1)?),
            (Operator::Not, None, Some(c2)) => Ok(!calc_formula(&c2)?),
            (Operator::B(b), None, None) => Ok(*b == 1),
            _ => {
                return Err(String::from("Error while calculating formula"))
            }
    }
}
