use std::net;
use uuid::Uuid;
use frame::frame_result::{ColType, ColTypeOptionValue, ColTypeOption};
use types::{CBytes, AsRust};
use types::data_serialization_types::*;
use types::map::Map;
use types::udt::UDT;
use error::Result;

// TODO: consider using pointers to ColTypeOption and Vec<CBytes> instead of owning them.
pub struct List {
    /// column spec of the list, i.e. id should be List as it's a list and value should contain
    /// a type of list items.
    metadata: ColTypeOption,
    data: Vec<CBytes>
}

impl List {
    pub fn new(data: Vec<CBytes>, metadata: ColTypeOption) -> List {
        return List {
            metadata: metadata,
            data: data
        };
    }

    fn map<T, F>(&self, f: F) -> Vec<T> where F: FnMut(&CBytes) -> T {
        return self.data
            .iter()
            .map(f)
            .collect();
    }
}

impl AsRust<Vec<Vec<u8>>> for List {
    /// Converts cassandra list of blobs into Rust `Vec<Vec<u8>>`
    fn as_rust(&self) -> Result<Vec<Vec<u8>>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Blob => Ok(
                            self.map(|bytes| decode_blob(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Blob => Ok(
                            self.map(|bytes| decode_blob(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<String>> for List {
    /// Converts cassandra list of String-like values into Rust `Vec<String>`
    fn as_rust(&self) -> Result<Vec<String>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Custom => Ok(
                        self.map(|bytes| decode_custom(bytes.as_plain()).unwrap())
                    ),
                    ColType::Ascii => Ok(
                        self.map(|bytes| decode_ascii(bytes.as_plain()).unwrap())
                    ),
                    ColType::Varchar => Ok(
                        self.map(|bytes| decode_varchar(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Custom => Ok(
                        self.map(|bytes| decode_custom(bytes.as_plain()).unwrap())
                    ),
                    ColType::Ascii => Ok(
                        self.map(|bytes| decode_ascii(bytes.as_plain()).unwrap())
                    ),
                    ColType::Varchar => Ok(
                        self.map(|bytes| decode_varchar(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<bool>> for List {
    /// Converts cassandra list of boolean-like values into Rust `Vec<bool>`
    fn as_rust(&self) -> Result<Vec<bool>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Boolean => Ok(
                        self.map(|bytes| decode_boolean(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Boolean => Ok(
                        self.map(|bytes| decode_boolean(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<i64>> for List {
    /// Converts cassandra list of i64-like values into Rust `Vec<i64>`
    fn as_rust(&self) -> Result<Vec<i64>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Bigint => Ok(
                        self.map(|bytes| decode_bigint(bytes.as_plain()).unwrap())
                    ),
                    ColType::Timestamp => Ok(
                        self.map(|bytes| decode_timestamp(bytes.as_plain()).unwrap())
                    ),
                    ColType::Time => Ok(
                        self.map(|bytes| decode_time(bytes.as_plain()).unwrap())
                    ),
                    ColType::Varint => Ok(
                        self.map(|bytes| decode_varint(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Bigint => Ok(
                        self.map(|bytes| decode_bigint(bytes.as_plain()).unwrap())
                    ),
                    ColType::Timestamp => Ok(
                        self.map(|bytes| decode_timestamp(bytes.as_plain()).unwrap())
                    ),
                    ColType::Time => Ok(
                        self.map(|bytes| decode_time(bytes.as_plain()).unwrap())
                    ),
                    ColType::Varint => Ok(
                        self.map(|bytes| decode_varint(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<i32>> for List {
    /// Converts cassandra list of i32-like values into Rust `Vec<i32>`
    fn as_rust(&self) -> Result<Vec<i32>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Int => Ok(
                        self.map(|bytes| decode_int(bytes.as_plain()).unwrap())
                    ),
                    ColType::Date => Ok(
                        self.map(|bytes| decode_date(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Int => Ok(
                        self.map(|bytes| decode_int(bytes.as_plain()).unwrap())
                    ),
                    ColType::Date => Ok(
                        self.map(|bytes| decode_date(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<i16>> for List {
    /// Converts cassandra list of i16-like values into Rust `Vec<i16>`
    fn as_rust(&self) -> Result<Vec<i16>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Smallint => Ok(
                        self.map(|bytes| decode_smallint(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Smallint => Ok(
                        self.map(|bytes| decode_smallint(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<f64>> for List {
    /// Converts cassandra list of f64-like values into Rust `Vec<f64>`
    fn as_rust(&self) -> Result<Vec<f64>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Double => Ok(
                        self.map(|bytes| decode_double(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Double => Ok(
                        self.map(|bytes| decode_double(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<f32>> for List {
    /// Converts cassandra list of f32-like values into Rust `Vec<f32>`
    fn as_rust(&self) -> Result<Vec<f32>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Decimal => Ok(
                        self.map(|bytes| decode_decimal(bytes.as_plain()).unwrap())
                    ),
                    ColType::Float => Ok(
                        self.map(|bytes| decode_float(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Decimal => Ok(
                        self.map(|bytes| decode_decimal(bytes.as_plain()).unwrap())
                    ),
                    ColType::Float => Ok(
                        self.map(|bytes| decode_float(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<net::IpAddr>> for List {
    /// Converts cassandra list of Inet values into Rust `Vec<net::IpAddr>`
    fn as_rust(&self) -> Result<Vec<net::IpAddr>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Inet => Ok(
                        self.map(|bytes| decode_inet(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Inet => Ok(
                        self.map(|bytes| decode_inet(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<Uuid>> for List {
    /// Converts cassandra list of UUID values into Rust `Vec<uuid::Uuid>`
    fn as_rust(&self) -> Result<Vec<Uuid>> {
        match self.metadata.value.clone().unwrap() {
            ColTypeOptionValue::CList(ref type_option) => {
                match type_option.id {
                    ColType::Uuid => Ok(
                        self.map(|bytes| decode_timeuuid(bytes.as_plain()).unwrap())
                    ),
                    ColType::Timeuuid => Ok(
                        self.map(|bytes| decode_timeuuid(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            ColTypeOptionValue::CSet(ref type_option) => {
                match type_option.id {
                    ColType::Uuid => Ok(
                        self.map(|bytes| decode_timeuuid(bytes.as_plain()).unwrap())
                    ),
                    ColType::Timeuuid => Ok(
                        self.map(|bytes| decode_timeuuid(bytes.as_plain()).unwrap())
                    ),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<List>> for List {
    /// Converts cassandra list of Inet values into Rust `Vec<List>`
    fn as_rust(&self) -> Result<Vec<List>> {
        match self.metadata.value.clone().unwrap() {
            // convert CList of T-s into List of T-s
            ColTypeOptionValue::CList(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option: &ColTypeOption = list_type_option_box.as_ref();
                match id {
                    // T is another List
                    &ColType::List => Ok(
                        self.map(|bytes| List::new(
                            decode_list(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    // T is another Set
                    &ColType::Set => Ok(
                        self.map(|bytes| List::new(
                            decode_list(bytes.as_plain()).unwrap(),
                            list_type_option.clone()))
                        ),
                    _ => unreachable!()
                }
            },
            // convert CSet of T-s into List of T-s
            ColTypeOptionValue::CSet(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option: &ColTypeOption = list_type_option_box.as_ref();
                match id {
                    // T is another List
                    &ColType::List => Ok(
                        self.map(|bytes| List::new(
                            decode_list(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    // T is another Set
                    &ColType::Set => Ok(
                        self.map(|bytes| List::new(
                            decode_list(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<Map>> for List {
    /// Converts cassandra list of Inet values into Rust `Vec<List>`
    fn as_rust(&self) -> Result<Vec<Map>> {
        match self.metadata.value.clone().unwrap() {
            // convert CList of T-s into List of T-s
            ColTypeOptionValue::CList(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option: &ColTypeOption = list_type_option_box.as_ref();
                match id {
                    // T is Map
                    &ColType::Map => Ok(
                        self.map(|bytes| Map::new(
                            decode_map(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    _ => unreachable!()
                }
            },
            // convert CSet of T-s into List of T-s
            ColTypeOptionValue::CSet(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option: &ColTypeOption = list_type_option_box.as_ref();
                match id {
                    // T is Map
                    &ColType::Map => Ok(
                        self.map(|bytes| Map::new(
                            decode_map(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

impl AsRust<Vec<UDT>> for List {
    /// Converts cassandra list of Inet values into Rust `Vec<List>`
    fn as_rust(&self) -> Result<Vec<UDT>> {
        match self.metadata.value.clone().unwrap() {
            // convert CList of T-s into List of T-s
            ColTypeOptionValue::CList(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option = match list_type_option_box.value {
                    Some(ColTypeOptionValue::UdtType(t)) => t,
                    _ => unreachable!()
                };
                match id {
                    // T is Udt
                    &ColType::Udt => Ok(
                        self.map(|bytes| UDT::new(
                            decode_udt(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    _ => unreachable!()
                }
            },
            // convert CSet of T-s into List of T-s
            ColTypeOptionValue::CSet(ref type_option) => {
                let ref id = type_option.id;
                let list_type_option_box: Box<ColTypeOption> = type_option.clone();
                let list_type_option = match list_type_option_box.value {
                    Some(ColTypeOptionValue::UdtType(t)) => t,
                    _ => return unreachable!()
                };
                match id {
                    // T is Udt
                    &ColType::Udt => Ok(
                        self.map(|bytes| UDT::new(
                            decode_udt(bytes.as_plain()).unwrap(),
                            list_type_option.clone())
                        )
                    ),
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

// TODO: implement for list of uuid
