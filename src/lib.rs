// NULL    = 'null'
// BOOLEAN = 'boolean'
// STRING  = 'string'
// BYTES   = 'bytes'
// INT     = 'int'
// LONG    = 'long'
// FLOAT   = 'float'
// DOUBLE  = 'double'
// FIXED   = 'fixed'
// ENUM    = 'enum'
// RECORD  = 'record'
// ERROR   = 'error'
// ARRAY   = 'array'
// MAP     = 'map'
// UNION   = 'union'

mod schema {
  enum Primitive {
    Null,
    Boolean,
    String,
    Bytes,
    Int,
    Long,
    Float,
    Double
  }
  enum Named {
    Fixed,
    Enum,
    Record,
    Error
  }
  enum Complex {
    Array,
    Map,
    Union,
    Request,
    ErrorUnion
  }
  enum Type {
    Primitive(Primitive),
    Named(Named),
    Complex(Complex)
  }
  enum Sort {
    Ascending,
    Descending,
    Ignore
  }

  static SCHEMA_RESERVED_PROPS: [&'static str; 9] = ["type",
  "name",
  "namespace",
  "fields",
  "items",
  "size",
  "symbols",
  "values",
  "doc"];
  static FIELD_RESERVED_PROPS: [&'static str; 5] = [
  "default",
   "name",
   "doc",
   "order",
  "type"];
struct Message {
  doc: Option<String>,
  request: Vec<Field>,
  response: Type,
  errors: Option<Vec<Type>>,
  one_way: Option<bool>
}
struct Field {
  name: String,
  doc: Option<String>,
  default: String,
  order: Option<Sort>
}
struct Record {
    name: String,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    fields: Vec<Field>
}
struct Enum {
    name: String,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    symbols: Vec<String>
}
struct Array {
  items: Box<Type>
}
struct Fixed {
  name: String,
  namespace: Option<String>,
  aliases: Option<Vec<String>>,
  size: i32
}


}

#[test]
#[should_panic]
fn it_works() {
}
