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
