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

// TODO: Extend this to all integer types u8, u16, u32, u64, i8, i16, i32, i64
impl IpldConvert for u8 {
    fn try_from_ipld(data: &Ipld) -> Result<Self, IpldConvertError>
    where
        Self: Sized,
    {
        match data {
            Ipld::Integer(val) => {
                u8::try_from(*val).map_err(|_| IpldConvertError::IntegerRangeError("u8"))
            }
            val => Err(IpldConvertError::InvalidType("u8", format!("{:?}", val))),
        }
    }

    fn to_ipld(&self) -> Ipld {
        Ipld::Integer(i128::from(*self))
    }
}

// TODO: Convert for Floats (Test, weather macro can be used)
// TODO: Convert for stringlikes
// TODO: Convert for Bytes
