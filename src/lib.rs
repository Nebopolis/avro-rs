use std::str::FromStr;
use std::error::Error;
use std::fmt;

enum Order {
  Ascending,
  Descending,
  Ignore
}

enum Type<'a> {
  Null,
  Boolean,
  Int,
  Long,
  Float,
  Double,
  Bytes,
  String,
  Record(Record<'a>),
  Enum(Enum<'a>),
  Array(Array<'a>),
  Map(Map<'a>),
  Union(&'a[Box<Type<'a>>]),
  Fixed(Fixed<'a>)
}

struct Field<'a> {
  name: &'a str,
  doc: Option<&'a str>,
  av_type: Type<'a>,
  default: &'a str,
  order: Option<Order>
}

struct Record<'a> {
    name: &'a str,
    namespace: Option<&'a str>,
    doc: Option<&'a str>,
    aliases: Option<&'a[&'a str]>,
    fields: &'a[Field<'a>]
}

struct Enum<'a> {
    name: &'a str,
    namespace: Option<&'a str>,
    doc: Option<&'a str>,
    aliases: Option<&'a[&'a str]>,
    symbols: &'a[&'a str]
}

struct Array<'a> {
  items: Box<Type<'a>>
}

struct Map<'a> {
  values: &'a[&'a str]
}

struct Fixed<'a> {
  name: &'a str,
  namespace: Option<&'a str>,
  aliases: Option<&'a[&'a str]>,
  size: i32
}

trait Valid {
  fn valid(&self) -> bool;
}

// trait ToType {
//   fn to_type(&self) -> Option<Type>;
// }

// impl <'a>ToType for &'a str {
//   fn to_type(&self) -> Option<Type> {
//     match &self.as_ref() {
//       "null" => Type::Null,
//       _     => Type::Null

//     }
//   }
// }

struct ParseTypeError<'a> {
  cause: Option<&'a Error>
}

impl <'a>fmt::Debug for ParseTypeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi")
    }
}

impl <'a> fmt::Display for ParseTypeError<'a> {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi")
    }
}

impl <'a>Error for ParseTypeError<'a> {

  fn description(&self) -> &str {
    "hi"
  }
}

impl <'a>FromStr for Type<'a> {
  type Err = ParseTypeError<'a>;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Type::Null)
  }
}

impl <'a>Valid for Field<'a> {
  fn valid(&self) -> bool {
    match &self.av_type {
      _ => false
    }
  }
}

impl <'a>Valid for Record<'a> {
  fn valid(&self) -> bool {
    return true;
  }
}

impl <'a>Valid for Enum<'a> {
  fn valid(&self) -> bool {
    return true;
  }
}


impl <'a>Valid for Array<'a> {
  fn valid(&self) -> bool {
    return true;
  }
}

impl <'a>Valid for Map<'a> {
  fn valid(&self) -> bool {
    return true;
  }
}

impl <'a>Valid for Fixed<'a> {
  fn valid(&self) -> bool {
    return true;
  }
}



#[test]
#[should_panic]
fn it_works() {
  let arr: Array = Array {
    items: Box::new(Type::Null)
  };
  let field: Field = Field {
    name: "null_array",
    doc: None,
    av_type: Type::Array(arr),
    default: "null",
    order: Some(Order::Ascending)
  };
  let record: Record = Record {
    name: "record",
    namespace: None,
    doc: None,
    aliases: None,
    fields: &[field]
  };
  println!("hello world");
  assert!(false)
}
