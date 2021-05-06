use std::fmt::{Display, Formatter};
use std::fmt;
use crate::value::{Value, UIntRange, UIntRangeRange};
use crate::value::Value::UIntValue;
use crate::types::Bindings;
use crate::error::Error;

pub(crate) enum Type {
    UInt,
    UIntRange,
    UIntRangeRange,
}

pub(crate) trait Expression: Display {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error>;
    fn get_type(&self) -> Type;
    fn as_typed(&self) -> AsTyped;
    fn clone_expr(&self) -> Box<dyn Expression>;
}

pub(crate) enum AsTyped<'a> {
    AsUInt(&'a dyn UIntExpression),
    AsUIntRange(&'a dyn UIntRangeExpression),
    AsUIntRangeRange(&'a UIntRangeRangeExpression),
}

impl AsTyped<'_> {
    fn get_type(&self) -> Type {
        match self {
            AsTyped::AsUInt(_) => { Type::UInt }
            AsTyped::AsUIntRange(_) => { Type::UIntRange }
            AsTyped::AsUIntRangeRange(_) => { Type::UIntRangeRange }
        }
    }
    pub(crate) fn as_int_expr(&self) -> Result<&dyn UIntExpression, Error> {
        match self {
            AsTyped::AsUInt(uint_expr) => Ok(*uint_expr),
            AsTyped::AsUIntRange(_) =>
                Err(Error::from("Expected integer expression, but got range expression.")),
            AsTyped::AsUIntRangeRange(_) =>
                Err(Error::from(
                    "Expected integer expression, but got range of ranges expression."
                )),
        }
    }
    pub(crate) fn as_range_expr(&self) -> Result<&dyn UIntRangeExpression, Error> {
        match self {
            AsTyped::AsUInt(_) =>
                Err(Error::from("Expected range expression, but got integer expression.")),
            AsTyped::AsUIntRange(range_expr) =>
                Ok(*range_expr),
            AsTyped::AsUIntRangeRange(_) =>
                Err(Error::from(
                    "Expected range expression, but got range of ranges expression."
                )),
        }
    }
    pub(crate) fn as_range_range_expr(&self) -> Result<&UIntRangeRangeExpression, Error> {
        match self {
            AsTyped::AsUInt(_) =>
                Err(Error::from("Expected range range expression, but got integer expression.")),
            AsTyped::AsUIntRange(_) =>
                Err(Error::from("Expected range range expression, but got range expression.")),
            AsTyped::AsUIntRangeRange(range_range_expr) =>
                Ok(*range_range_expr)
        }
    }
}

pub(crate) trait UIntExpression: Expression {
    fn eval_int(&self, bindings: &Bindings) -> Result<u64, Error>;
    fn clone_int_expr(&self) -> Box<dyn UIntExpression>;
}

pub(crate) trait UIntRangeExpression: Expression {
    fn eval_range(&self, bindings: &Bindings) -> Result<UIntRange, Error>;
    fn clone_range_expr(&self) -> Box<dyn UIntRangeExpression>;
}

pub(crate) struct UIntLiteral {
    value: u64,
}

pub(crate) struct UIntVariable {
    id: String,
}

pub(crate) struct UIntSimpleRangeExpression {
    from: Box<dyn UIntExpression>,
    until: Box<dyn UIntExpression>,
}

pub(crate) struct UIntRangeRangeExpression {
    dividend: Box<dyn UIntRangeExpression>,
    divisor: Box<dyn UIntRangeExpression>,
}

pub(crate) struct UIntPickRangeExpression {
    groups: Box<UIntRangeRangeExpression>,
    pick: Box<dyn UIntExpression>,
}

impl UIntLiteral {
    pub(crate) fn new(value: u64) -> UIntLiteral { UIntLiteral { value } }
}

impl UIntVariable {
    pub(crate) fn new(id: String) -> UIntVariable { UIntVariable { id } }
}

impl UIntSimpleRangeExpression {
    pub(crate) fn new(from: Box<dyn UIntExpression>, until: Box<dyn UIntExpression>)
                      -> UIntSimpleRangeExpression {
        UIntSimpleRangeExpression { from, until }
    }
}

impl UIntRangeRangeExpression {
    pub(crate) fn new(dividend: Box<dyn UIntRangeExpression>,
                      divisor: Box<dyn UIntRangeExpression>)
                      -> UIntRangeRangeExpression {
        UIntRangeRangeExpression { dividend, divisor }
    }
    pub(crate) fn clone_range_range_expr(&self) -> Box<UIntRangeRangeExpression> {
        Box::new(
            UIntRangeRangeExpression::new(self.dividend.clone_range_expr(),
                                          self.divisor.clone_range_expr())
        )
    }
}

impl UIntPickRangeExpression {
    pub(crate) fn new(groups: Box<UIntRangeRangeExpression>,
                      pick: Box<dyn UIntExpression>)
                      -> UIntPickRangeExpression {
        UIntPickRangeExpression { groups, pick }
    }
}

impl Expression for UIntLiteral {
    fn eval(&self, _: &Bindings) -> Result<Value, Error> { Ok(UIntValue(self.value)) }
    fn get_type(&self) -> Type { Type::UInt }
    fn as_typed<'a>(&'a self) -> AsTyped<'a> { AsTyped::AsUInt::<'a>(self) }
    fn clone_expr(&self) -> Box<dyn Expression> { Box::new(UIntLiteral { value: self.value }) }
}

impl UIntExpression for UIntLiteral {
    fn eval_int(&self, _bindings: &Bindings) -> Result<u64, Error> { Ok(self.value) }
    fn clone_int_expr(&self) -> Box<dyn UIntExpression> {
        Box::new(UIntLiteral { value: self.value })
    }
}

impl Expression for UIntVariable {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        match bindings.get(&self.id) {
            Some(value) => Ok(value),
            None => Err(Error::from(format!("Unknown identifier {}.", self.id)))
        }
    }

    fn get_type(&self) -> Type { Type::UInt }
    fn as_typed<'a>(&'a self) -> AsTyped<'a> { AsTyped::AsUInt::<'a>(self) }
    fn clone_expr(&self) -> Box<dyn Expression> {
        Box::new(UIntVariable { id: self.id.clone() })
    }
}

impl UIntExpression for UIntVariable {
    fn eval_int(&self, bindings: &Bindings) -> Result<u64, Error> {
        match bindings.get(&self.id) {
            Some(UIntValue(ui)) => Ok(ui),
            Some(value) =>
                Err(Error::from(format!("Expected unsigned int, but got {}.", value))),
            None => Err(Error::from(format!("Unknown identifier {}.", self.id)))
        }
    }
    fn clone_int_expr(&self) -> Box<dyn UIntExpression> {
        Box::new(UIntVariable { id: self.id.clone() })
    }
}

impl Expression for UIntSimpleRangeExpression {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        let from = self.from.eval(bindings)?.as_int()?;
        let until = self.until.eval(bindings)?.as_int()?;
        Ok(Value::new_range(from, until))
    }

    fn get_type(&self) -> Type { Type::UIntRange }
    fn as_typed(&self) -> AsTyped { AsTyped::AsUIntRange(self) }

    fn clone_expr(&self) -> Box<dyn Expression> {
        Box::new(
            UIntSimpleRangeExpression::new(self.from.clone_int_expr(),
                                           self.until.clone_int_expr())
        )
    }
}

impl UIntRangeExpression for UIntSimpleRangeExpression {
    fn eval_range(&self, bindings: &Bindings) -> Result<UIntRange, Error> {
        let from = self.from.eval(bindings)?.as_int()?;
        let until = self.until.eval(bindings)?.as_int()?;
        Ok(UIntRange::new(from, until))
    }
    fn clone_range_expr(&self) -> Box<dyn UIntRangeExpression> {
        Box::new(
            UIntSimpleRangeExpression::new(self.from.clone_int_expr(),
                                           self.until.clone_int_expr())
        )
    }
}

impl Expression for UIntRangeRangeExpression {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        let dividend = self.dividend.eval(bindings)?.as_range()?;
        let divisor = self.divisor.eval(bindings)?.as_range()?;
        Ok(Value::UIntRangeRangeValue(UIntRangeRange::new(dividend, divisor)))
    }

    fn get_type(&self) -> Type { Type::UIntRangeRange }
    fn as_typed(&self) -> AsTyped { AsTyped::AsUIntRangeRange(self) }

    fn clone_expr(&self) -> Box<dyn Expression> {
        Box::new(UIntRangeRangeExpression::new(self.dividend.clone_range_expr(),
                                               self.divisor.clone_range_expr()))
    }
}

impl Expression for UIntPickRangeExpression {
    fn eval(&self, bindings: &Bindings) -> Result<Value, Error> {
        let groups = self.groups.eval(bindings)?.as_range_range()?;
        let pick = self.pick.eval(bindings)?.as_int()?;
        Ok(Value::UIntRangeValue(groups.pick(pick)?))
    }

    fn get_type(&self) -> Type { Type::UIntRange }
    fn as_typed(&self) -> AsTyped { AsTyped::AsUIntRange(self) }

    fn clone_expr(&self) -> Box<dyn Expression> {
        Box::new(UIntPickRangeExpression::new(self.groups.clone_range_range_expr(),
                                              self.pick.clone_int_expr()))
    }
}

impl UIntRangeExpression for UIntPickRangeExpression {
    fn eval_range(&self, bindings: &Bindings) -> Result<UIntRange, Error> {
        let groups = self.groups.eval(bindings)?.as_range_range()?;
        let pick = self.pick.eval(bindings)?.as_int()?;
        Ok(groups.pick(pick)?)
    }

    fn clone_range_expr(&self) -> Box<dyn UIntRangeExpression> {
        Box::new(UIntPickRangeExpression::new(self.groups.clone_range_range_expr(),
                                              self.pick.clone_int_expr()))
    }
}

impl Display for UIntLiteral {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, formatter)
    }
}

impl Display for UIntVariable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.id, formatter)
    }
}

impl Display for UIntSimpleRangeExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(format!("{} .. {}", self.from, self.until).as_str(), f)
    }
}

impl Display for UIntRangeRangeExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(format!("{} / {}", self.dividend, self.divisor).as_str(), f)
    }
}

impl Display for UIntPickRangeExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(format!("{} $ {}", self.groups, self.pick).as_str(), f)
    }
}