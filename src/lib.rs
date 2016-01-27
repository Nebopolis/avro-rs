extern crate serde_json;

mod schema {
  extern crate serde;

  struct Error {
    name: String,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    fields: Vec<Field>
  }

  struct Field {
    name: String,
    doc: Option<String>,
    default: String,
    order: Option<Sort>
  }

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
    Fixed {
      name: String,
      namespace: Option<String>,
      aliases: Option<Vec<String>>,
      size: i32
    },
    Enum {
      name: String,
      namespace: Option<String>,
      doc: Option<String>,
      aliases: Option<Vec<String>>,
      symbols: Vec<String>
    },
    Record {
      name: String,
      namespace: Option<String>,
      doc: Option<String>,
      aliases: Option<Vec<String>>,
      fields: Vec<Field>
    },
    Error(Error)
  }

  enum Complex {
    Array {
     items: Box<Type>
    },
    Map {
      values: Vec<String>
    },
    Union(Vec<Type>),
    Request(Vec<Field>),
    ErrorUnion(Vec<Error>)
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

  struct Message {
    doc: Option<String>,
    request: Vec<Field>,
    response: Type,
    errors: Option<Vec<Type>>,
    one_way: Option<bool>
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

  pub struct Fixed {
    pub name: String,
    pub namespace: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub size: i32
  }

  impl serde::Serialize for Fixed {
      fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
          where S: serde::Serializer
      {
          serializer.visit_struct("fixed", FixedMapVisitor {
              value: self,
              state: 0,
          })
      }
  }

  struct FixedMapVisitor<'a> {
      value: &'a Fixed,
      state: u8,
  }

  impl<'a> serde::ser::MapVisitor for FixedMapVisitor<'a> {
      fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
          where S: serde::Serializer
      {
          match self.state {
              0 => {
                  self.state += 1;
                  Ok(Some(try!(serializer.visit_struct_elt("name", &self.value.name))))
              }
              1 => {
                  self.state += 1;
                  Ok(Some(try!(serializer.visit_struct_elt("namespace", &self.value.namespace))))
              },
              2 => {
                  self.state += 1;
                  Ok(Some(try!(serializer.visit_struct_elt("aliases", &self.value.aliases))))
              },
              3 => {
                  self.state += 1;
                  Ok(Some(try!(serializer.visit_struct_elt("size", &self.value.size))))
              },
              _ => {
                  Ok(None)
              }
          }
      }
  }
}

#[test]
#[should_panic]
fn it_works() {
  let a = schema::Fixed {
    name: "hello".to_string(),
    namespace: None,
    aliases: None,
    size: 8
  };

  let serialized = serde_json::to_string(&a).unwrap();
  println!("{:?}", serialized);

}
