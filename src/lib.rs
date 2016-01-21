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
  
}


#[test]
#[should_panic]
fn it_works() {
}

// # Request and error unions are part of Avro protocols:
// REQUEST = 'request'
// ERROR_UNION = 'error_union'

// PRIMITIVE_TYPES = frozenset([
//   NULL,
//   BOOLEAN,
//   STRING,
//   BYTES,
//   INT,
//   LONG,
//   FLOAT,
//   DOUBLE,
// ])

// NAMED_TYPES = frozenset([
//   FIXED,
//   ENUM,
//   RECORD,
//   ERROR,
// ])

// VALID_TYPES = frozenset.union(
//   PRIMITIVE_TYPES,
//   NAMED_TYPES,
//   [
//     ARRAY,
//     MAP,
//     UNION,
//     REQUEST,
//     ERROR_UNION,
//   ],
// )

// SCHEMA_RESERVED_PROPS = frozenset([
//   'type',
//   'name',
//   'namespace',
//   'fields',     # Record
//   'items',      # Array
//   'size',       # Fixed
//   'symbols',    # Enum
//   'values',     # Map
//   'doc',
// ])

// FIELD_RESERVED_PROPS = frozenset([
//   'default',
//   'name',
//   'doc',
//   'order',
//   'type',
// ])

// VALID_FIELD_SORT_ORDERS = frozenset([
//   'ascending',
//   'descending',
//   'ignore',
// ])
