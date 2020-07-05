use std::collections::HashMap;
use std::boxed::Box;


pub enum IntWidth {
    Int8,
    Int16,
    Int32,
    Int64,
}

impl IntWidth {
    pub fn sizeof(&self) -> u64 {
        match self {
            IntWidth::Int8 => 1,
            IntWidth::Int16 => 2,
            IntWidth::Int32 => 4,
            IntWidth::Int64 => 8,
        }
    }
}

pub enum Sign {
    Signed,
    Unsiged
}

pub enum Float {
    Float,
    Double,
}

impl Float {
    pub fn sizeof(&self) -> u64 {
        match self {
            Float::Float => 4,
            Float::Double => 8,
        }
    }
}

pub enum PrimType {
    Int(IntWidth, Sign),
    Flt(Float),
    Bits(u8),
}

impl PrimType {
    pub fn sizeof(&self) -> u64 {
        match self {
            PrimType::Int(width, sign) => width.sizeof(),
            PrimType::Flt(float) => float.sizeof(),
            PrimType::Bits(num_bits) => power_of_2_greater_than(*num_bits as u64),
        }
    }

    pub fn size_bits(&self) -> u64 {
        match self {
            PrimType::Bits(num_bits) => *num_bits as u64,
            prim => prim.sizeof() * 8,
        }
    }
}

pub enum Endianness {
    Little,
    Big,
}

pub struct PrimField {
    name: String,
    typ: PrimType,
    endianness: Endianness,
}

pub struct Struct {
    name: String,
    fields: Vec<Section>,
    packing: Option<u64>,
}

pub struct Union {
    name: String,
    fields: Vec<Section>,
}

pub enum Section {
    Prim(PrimField),
    Struct(Box<Struct>),
    Array(Box<Section>, u64),
    Union(Union),
}

impl Section {
    pub fn sizeof(&self) -> u64 {
        match self {
            Section::Prim(prim) => {
                return prim.typ.sizeof();
            }

            Section::Struct(structure) => {
                // TODO need to deal with bit fields
                // likely once hit bits, accumulate without alignment until
                // aligned again.
                // also make sure to expand to byte alignment at the end, and
                // to ensure stride at the end
                let mut size = 0;
                if structure.fields.len() > 0 {
                    let first_size = structure.fields[0].sizeof();
                    for field in structure.fields.iter() {
                        let field_size = field.sizeof();
                        size = align_to(size, field_size) + field_size;
                    }
                    // align to first field, to ensure stride remains aligned
                    size = align_to(size, first_size);
                 }
                return size;
            }

            Section::Array(section, num_elems) => {
                return section.sizeof() * num_elems;
            }

            Section::Union(union) => {
                let mut largest = 0;
                for field in union.fields.iter() {
                    largest = std::cmp::max(largest, field.sizeof());
                }
                return largest;
            }
        }
    }

    pub fn size_bits(&self) -> u64 {
        match self {
            Section::Prim(prim) => {
                return prim.typ.size_bits();
            }

            Section::Struct(structure) => {
                // TODO need to deal with bit fields
                // they should be packed, and cause next fields to be
                // packed as bit fields, until byte alignment
                let mut size = 0;
                if structure.fields.len() > 0 {
                    let first_size = structure.fields[0].sizeof();
                    for field in structure.fields.iter() {
                        let field_size = field.sizeof();
                        size = align_to(size, field_size) + field_size;
                    }
                    // align to first field, to ensure stride remains aligned
                    size = align_to(size, first_size);
                 }
                return size;
            }

            Section::Array(section, num_elems) => {
                return section.size_bits() * num_elems;
            }

            Section::Union(union) => {
                let mut largest = 0;
                for field in union.fields.iter() {
                    largest = std::cmp::max(largest, field.size_bits());
                }
                return largest;
            }
        }
    }
}

pub struct Field {
    name: String,
    offset: u64,
    typ: Section,
}

pub fn power_of_2_greater_than(num_bits: u64) -> u64 {
    let mut power: u64 = 0;

    while power < 64 && 2u64.pow(power as u32) < num_bits {
        power += 1;
    }

    return power;
}

#[test]
fn test_power_of_2() {
    assert_eq!(0, power_of_2_greater_than(0));
    assert_eq!(0, power_of_2_greater_than(1));
    assert_eq!(5, power_of_2_greater_than(32));
    assert_eq!(6, power_of_2_greater_than(63));
    assert_eq!(6, power_of_2_greater_than(64));
    assert_eq!(7, power_of_2_greater_than(65));
    assert_eq!(63, power_of_2_greater_than(2u64.pow(63)));
    assert_eq!(64, power_of_2_greater_than(2u64.pow(63)) + 1);
    assert_eq!(64, power_of_2_greater_than(0xFFFFFFFFFFFFFFFFu64));
}

pub fn align_to(size: u64, align: u64) -> u64 {
    // ensure align is > 0
    let align = align + (align == 0) as u64;
    let m = size % align;
    let b = m != 0;
    return size + (align - m) * (b as u64);
}

#[test]
pub fn test_align_to() {
    assert_eq!(8, align_to(5, 4));
    assert_eq!(5, align_to(5, 0));
    assert_eq!(5, align_to(5, 1));
    assert_eq!(10, align_to(9, 2));
}

pub enum PrimData {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
