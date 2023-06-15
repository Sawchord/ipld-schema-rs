use libipld::Ipld;
use thiserror::Error;

/// Trait indicating that this type can be converted between a schemaless [`Ipld`] type
/// and a rust type using a schema
pub trait IpldConvert {
    /// Try to convert an [`Ipld`] structure into a rust type
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized;

    /// Convert the rust type into an [`Ipld`] structure
    fn to_ipld(&self) -> Ipld;
}

///
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IpldConvertError {
    #[error("expected a value of type {0} but this is a {1}")]
    InvalidType(&'static str, String),

    #[error("the encoded IPLD integer value does not fit into a {0}")]
    IntegerRangeError(&'static str),
}

/// DRY macro for implementing the different integer types
macro_rules! implement_integer {
    ($n: ty, $s: literal) => {
        impl IpldConvert for $n {
            fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
            where
                Self: Sized,
            {
                match data {
                    Ipld::Integer(val) => {
                        <$n>::try_from(*val).map_err(|_| IpldConvertError::IntegerRangeError($s))
                    }
                    val => Err(IpldConvertError::InvalidType(
                        "Integer",
                        format!("{:?}", val),
                    )),
                }
            }

            fn to_ipld(&self) -> Ipld {
                Ipld::Integer(i128::from(*self))
            }
        }
    };
}

implement_integer!(u8, "u8");
implement_integer!(u16, "u16");
implement_integer!(u32, "u32");
implement_integer!(u64, "u64");
implement_integer!(i8, "i8");
implement_integer!(i16, "i16");
implement_integer!(i32, "i32");
implement_integer!(i64, "i64");

impl IpldConvert for () {
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized,
    {
        match data {
            Ipld::Null => Ok(()),
            val => Err(IpldConvertError::InvalidType("Null", format!("{:?}", val))),
        }
    }

    fn to_ipld(&self) -> Ipld {
        Ipld::Null
    }
}

impl IpldConvert for bool {
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized,
    {
        match data {
            Ipld::Bool(val) => Ok(*val),
            val => Err(IpldConvertError::InvalidType("Bool", format!("{:?}", val))),
        }
    }

    fn to_ipld(&self) -> Ipld {
        Ipld::Bool(*self)
    }
}

impl IpldConvert for f32 {
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized,
    {
        match data {
            Ipld::Float(val) => Ok(*val as f32),
            val => Err(IpldConvertError::InvalidType("Float", format!("{:?}", val))),
        }
    }

    fn to_ipld(&self) -> Ipld {
        Ipld::Float(f64::from(*self))
    }
}

impl IpldConvert for f64 {
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized,
    {
        match data {
            Ipld::Float(val) => Ok(*val),
            val => Err(IpldConvertError::InvalidType("Float", format!("{:?}", val))),
        }
    }

    fn to_ipld(&self) -> Ipld {
        Ipld::Float(*self)
    }
}

// TODO: Convert for Bytes
// TODO: Convert for stringlikes
