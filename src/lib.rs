
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
  name: &'static str,
  doc: Option<String>,
  av_type: Type<'a>,
  default: &'static str,
  order: Option<Order>
}

struct Record<'a> {
    name: &'static str,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<&'a[String]>,
    fields: &'a[Field<'a>]
}

struct Enum<'a> {
    name: &'static str,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<&'a[String]>,
    symbols: &'a[String]
}

struct Array<'a> {
  items: Box<Type<'a>>
}

struct Map<'a> {
  values: &'a[String]
}

struct Fixed<'a> {
  name: &'static str,
  namespace: Option<String>,
  aliases: Option<&'a[String]>,
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
