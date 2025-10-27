use alloc::rc::Rc;
use core::fmt::Debug;
use p3_air::{Air, BaseAir};
use p3_field::Field;
use p3_goldilocks::Goldilocks;
use p3_uni_stark::{SymbolicExpression, SymbolicVariable};
use pico_vm::{chips::chips::alu::add_sub::AddSubChip, machine::folder::SymbolicConstraintFolder};

extern crate alloc;

fn main() {
    print_add_sub();
}

pub(crate) fn print_add_sub() {
    let chip = AddSubChip::default();
    let mut builder = SymbolicConstraintFolder::new(chip.width(), chip.width());

    chip.eval(&mut builder);

    builder.to_rocq(0);
    println!("Result 🛍️");
    println!("  tt");
}

trait ToRocq {
    fn to_rocq(&self, indent: usize);
}

enum FlatSymbolicExpression<'a, F> {
    Variable(&'a SymbolicVariable<F>),
    IsFirstRow,
    IsLastRow,
    IsTransition,
    Constant(F),
    Add(Vec<Rc<FlatSymbolicExpression<'a, F>>>),
    Sub {
        x: Rc<FlatSymbolicExpression<'a, F>>,
        y: Rc<FlatSymbolicExpression<'a, F>>,
    },
    Neg {
        x: Rc<FlatSymbolicExpression<'a, F>>,
    },
    Mul(Vec<Rc<FlatSymbolicExpression<'a, F>>>),
}

impl<'a, F> FlatSymbolicExpression<'a, F>
where
    F: Field,
{
    fn from_symbolic_expression(expr: &'a SymbolicExpression<F>) -> Self {
        match expr {
            SymbolicExpression::Variable(v) => Self::Variable(v),
            SymbolicExpression::IsFirstRow => Self::IsFirstRow,
            SymbolicExpression::IsLastRow => Self::IsLastRow,
            SymbolicExpression::IsTransition => Self::IsTransition,
            SymbolicExpression::Constant(c) => Self::Constant(*c),
            SymbolicExpression::Add { x, y, .. } => {
                let x = Self::from_symbolic_expression(x);
                let y = Self::from_symbolic_expression(y);
                match (&x, &y) {
                    (Self::Add(xs), Self::Add(ys)) => {
                        Self::Add([xs.to_vec(), ys.to_vec()].concat())
                    }
                    (Self::Add(xs), _) => Self::Add([xs.to_vec(), vec![Rc::new(y)]].concat()),
                    (_, Self::Add(ys)) => Self::Add([vec![Rc::new(x)], ys.to_vec()].concat()),
                    (_, _) => Self::Add(vec![Rc::new(x), Rc::new(y)]),
                }
            }
            SymbolicExpression::Sub { x, y, .. } => Self::Sub {
                x: Rc::new(Self::from_symbolic_expression(x)),
                y: Rc::new(Self::from_symbolic_expression(y)),
            },
            SymbolicExpression::Neg { x, .. } => Self::Neg {
                x: Rc::new(Self::from_symbolic_expression(x)),
            },
            SymbolicExpression::Mul { x, y, .. } => {
                let x = Self::from_symbolic_expression(x);
                let y = Self::from_symbolic_expression(y);
                match (&x, &y) {
                    (Self::Mul(xs), Self::Mul(ys)) => {
                        Self::Mul([xs.to_vec(), ys.to_vec()].concat())
                    }
                    (Self::Mul(xs), _) => Self::Mul([xs.to_vec(), vec![Rc::new(y)]].concat()),
                    (_, Self::Mul(ys)) => Self::Mul([vec![Rc::new(x)], ys.to_vec()].concat()),
                    (_, _) => Self::Mul(vec![Rc::new(x), Rc::new(y)]),
                }
            }
        }
    }
}

impl<'a, F> ToRocq for FlatSymbolicExpression<'a, F>
where
    F: Debug,
{
    fn to_rocq(&self, indent: usize) {
        match self {
            FlatSymbolicExpression::Variable(v) => {
                println!("{}Variable: {:?}", " ".repeat(indent), v.index);
            }
            FlatSymbolicExpression::IsFirstRow => {
                println!("{}IsFirstRow", " ".repeat(indent));
            }
            FlatSymbolicExpression::IsLastRow => {
                println!("{}IsLastRow", " ".repeat(indent));
            }
            FlatSymbolicExpression::IsTransition => {
                println!("{}IsTransition", " ".repeat(indent));
            }
            FlatSymbolicExpression::Constant(c) => {
                println!("{}Constant: {:?}", " ".repeat(indent), c);
            }
            FlatSymbolicExpression::Add(xs) => {
                println!("{}Add:", " ".repeat(indent));
                for x in xs {
                    x.to_rocq(indent + 2);
                }
            }
            FlatSymbolicExpression::Sub { x, y } => {
                println!("{}Sub:", " ".repeat(indent));
                x.to_rocq(indent + 2);
                y.to_rocq(indent + 2);
            }
            FlatSymbolicExpression::Neg { x } => {
                println!("{}Neg:", " ".repeat(indent));
                x.to_rocq(indent + 2);
            }
            FlatSymbolicExpression::Mul(xs) => {
                println!("{}Mul:", " ".repeat(indent));
                for x in xs {
                    x.to_rocq(indent + 2);
                }
            }
        }
    }
}

impl<F> ToRocq for SymbolicExpression<F>
where
    F: Field,
    F: Debug,
{
    fn to_rocq(&self, indent: usize) {
        FlatSymbolicExpression::from_symbolic_expression(self).to_rocq(indent);
    }
}

impl<T: ToRocq> ToRocq for Option<T> {
    fn to_rocq(&self, indent: usize) {
        match self {
            Some(t) => {
                println!("{}Some:", " ".repeat(indent));
                t.to_rocq(indent + 2);
            }
            None => {
                println!("{}None", " ".repeat(indent));
            }
        }
    }
}

impl<T: ToRocq, const N: usize> ToRocq for [T; N] {
    fn to_rocq(&self, indent: usize) {
        println!("{}Array:", " ".repeat(indent));
        for item in self {
            item.to_rocq(indent + 2);
        }
    }
}

impl ToRocq for SymbolicConstraintFolder<Goldilocks> {
    fn to_rocq(&self, indent: usize) {
        println!("{}Trace 🐾", " ".repeat(indent));
        for constraint in self.constraints_iter() {
            println!("{}AssertZero:", " ".repeat(indent + 2));
            constraint.clone().to_rocq(indent + 4);
        }
    }
}
