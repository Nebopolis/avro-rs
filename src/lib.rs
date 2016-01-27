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

  pub struct Field {
    pub name: String,
    pub doc: Option<String>,
    pub default: String,
    pub order: Option<Sort>
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
    Fixed(Fixed),
    Enum(Enum),
    Record(Record),
    Error(Error)
  }

  enum Complex {
    Array(Array),
    Map(Vec<String>),
    Union(Vec<Type>),
    Request(Vec<Field>),
    ErrorUnion(Vec<Error>)
  }

  enum Type {
    Primitive(Primitive),
    Named(Named),
    Complex(Complex)
  }

  pub enum Sort {
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

  pub struct Record {
    pub name: String,
    pub namespace: Option<String>,
    pub doc: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub fields: Vec<Field>
  }

  pub struct Enum {
    pub name: String,
    pub namespace: Option<String>,
    pub doc: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub symbols: Vec<String>
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

  struct MapVisitor<'a, T: 'a> {
    value: &'a T,
    state: u8,
  }

  impl serde::Serialize for Fixed {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer
    {
      serializer.visit_struct("fixed", MapVisitor::<Fixed> {
        value: self,
        state: 0,
      })
    }
  }

  impl<'a> serde::ser::MapVisitor for MapVisitor<'a, Fixed> {
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

  impl serde::Serialize for Enum {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer
    {
      serializer.visit_struct("enum", MapVisitor::<Enum> {
        value: self,
        state: 0,
      })
    }
  }

  impl<'a> serde::ser::MapVisitor for MapVisitor<'a, Enum> {
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
          Ok(Some(try!(serializer.visit_struct_elt("doc", &self.value.doc))))
        },
        3 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("aliases", &self.value.aliases))))
        },
        4 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("symbols", &self.value.symbols))))
        },
        _ => {
          Ok(None)
        }
      }
    }
  }

  impl serde::Serialize for Record {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer
    {
      serializer.visit_struct("record", MapVisitor::<Record> {
        value: self,
        state: 0,
      })
    }
  }

  impl<'a> serde::ser::MapVisitor for MapVisitor<'a, Record> {
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
          Ok(Some(try!(serializer.visit_struct_elt("doc", &self.value.doc))))
        },
        3 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("aliases", &self.value.aliases))))
        },
        4 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("fields", &self.value.fields))))
        },
        _ => {
          Ok(None)
        }
      }
    }
  }

  impl serde::Serialize for Sort {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer
    {
      let val = match *self {
        Sort::Ascending => "ascending",
        Sort::Descending => "descending",
        Sort::Ignore => "ignore",
      };
      serializer.visit_map_elt("sort", val)
    }
  }

  impl serde::Serialize for Field {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
      where S: serde::Serializer
    {
      serializer.visit_struct("field", MapVisitor::<Field> {
        value: self,
        state: 0,
      })
    }
  }

  impl<'a> serde::ser::MapVisitor for MapVisitor<'a, Field> {
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
          Ok(Some(try!(serializer.visit_struct_elt("doc", &self.value.doc))))
        },
        2 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("default", &self.value.default))))
        },
        3 => {
          self.state += 1;
          Ok(Some(try!(serializer.visit_struct_elt("order", &self.value.order))))
        },
        _ => {
          Ok(None)
        }
      }
    }
  }

}

#[test]
fn serialize_fixed() {
  let a = schema::Fixed {
    name: "hello".to_string(),
    namespace: None,
    aliases: None,
    size: 8
  };

  let serialized = serde_json::to_string(&a).unwrap();
  assert_eq!("{\"name\":\"hello\",\"namespace\":null,\"aliases\":null,\"size\":8}", serialized);

}
