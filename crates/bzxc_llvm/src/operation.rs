use bzxc_shared::{DynType, Error, Node, Tokens};
use inkwell::{values::BasicValueEnum, FloatPredicate, IntPredicate};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn binary_op(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node.clone() {
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => {
                let left_val = self.compile_node(*left)?;
                let right_val = self.compile_node(*right)?;

                match op_token.typee {
                    Tokens::DoubleEquals => {
                        return Ok(self
                            .context
                            .bool_type()
                            .const_int((left_val == right_val) as u64, false)
                            .into())
                    }
                    Tokens::NotEquals => {
                        return Ok(self
                            .context
                            .bool_type()
                            .const_int((left_val != right_val) as u64, false)
                            .into())
                    }
                    _ => (),
                }

                if left_val.is_int_value() && right_val.is_int_value() {
                    let lhs = left_val.into_int_value();
                    let rhs = right_val.into_int_value();

                    let ret = match op_token.typee {
                        Tokens::Plus => self.builder.build_int_add(lhs, rhs, "tmpadd"),
                        Tokens::Minus => self.builder.build_int_sub(lhs, rhs, "tmpsub"),
                        Tokens::Multiply => self.builder.build_int_mul(lhs, rhs, "tmpmul"),
                        Tokens::Divide => self.builder.build_int_unsigned_div(lhs, rhs, "tmpdiv"),
                        Tokens::LessThan => {
                            self.builder
                                .build_int_compare(IntPredicate::ULT, lhs, rhs, "tmpcmp")
                        }
                        Tokens::GreaterThan => {
                            self.builder
                                .build_int_compare(IntPredicate::UGT, lhs, rhs, "tmpcmp")
                        }
                        Tokens::LessThanEquals => {
                            self.builder
                                .build_int_compare(IntPredicate::ULE, lhs, rhs, "tmpcmp")
                        }
                        Tokens::GreaterThanEquals => {
                            self.builder
                                .build_int_compare(IntPredicate::UGE, lhs, rhs, "tmpcmp")
                        }
                        _ => {
                            if op_token.matches(Tokens::Keyword, DynType::String("and".to_string()))
                            {
                                lhs.const_and(rhs)
                            } else if op_token
                                .matches(Tokens::Keyword, DynType::String("or".to_string()))
                            {
                                lhs.const_or(rhs)
                            } else {
                                return Err(self.error(node.get_pos(), "Unknown operation"));
                            }
                        }
                    };
                    return Ok(ret.into());
                }

                if left_val.is_float_value() && right_val.is_float_value() {
                    let lhs = left_val.into_float_value();
                    let rhs = right_val.into_float_value();

                    let ret = match op_token.typee {
                        Tokens::Plus => self.builder.build_float_add(lhs, rhs, "tmpadd"),
                        Tokens::Minus => self.builder.build_float_sub(lhs, rhs, "tmpsub"),
                        Tokens::Multiply => self.builder.build_float_mul(lhs, rhs, "tmpmul"),
                        Tokens::Divide => self.builder.build_float_div(lhs, rhs, "tmpdiv"),
                        Tokens::LessThan => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULT,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::GreaterThan => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::UGT,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::LessThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULE,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::GreaterThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::OGE,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        _ => return Err(self.error(node.get_pos(), "Unknown operation")),
                    };
                    return Ok(ret.into());
                }

                Err(self.error(node.get_pos(), "Unknown operation"))
            }
            _ => panic!(),
        }
    }

    pub(crate) fn unary_op(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node.clone() {
            Node::UnaryNode {
                node: child,
                op_token,
            } => {
                let val = self.compile_node(*child)?;

                if val.is_float_value() {
                    let built = val.into_float_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err(self.error(node.get_pos(), "Unknown unary operation")),
                    };
                    return Ok(ret.into());
                }

                if val.is_int_value() {
                    let built = val.into_int_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err(self.error(node.get_pos(), "Unknown unary operation")),
                    };
                    return Ok(ret.into());
                }

                Err(self.error(node.get_pos(), "Unknown unary operation"))
            }
            _ => panic!(),
        }
    }
}