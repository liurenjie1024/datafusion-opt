mod logical;
pub use logical::*;
mod physical;
pub use physical::*;
mod limit;
pub use limit::*;
mod projection;
pub use projection::*;
mod table_scan;
pub use table_scan::*;
mod join;
use enum_as_inner::EnumAsInner;
pub use join::*;
pub use physical::*;

use crate::error::OptResult;
use crate::optimizer::Optimizer;
use crate::properties::LogicalProperty;

#[derive(Clone, Debug, Hash, Eq, PartialEq, EnumAsInner)]
pub enum Operator {
    Logical(LogicalOperator),
    Physical(PhysicalOperator),
}

pub trait OperatorTrait {
    fn derive_logical_prop<O: Optimizer>(
        &self,
        _handle: O::ExprHandle,
        optimizer: &O,
    ) -> OptResult<LogicalProperty>;
}

impl OperatorTrait for Operator {
    fn derive_logical_prop<O: Optimizer>(
        &self,
        _handle: O::ExprHandle,
        _optimizer: &O,
    ) -> OptResult<LogicalProperty> {
        todo!()
    }
}
