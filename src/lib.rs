#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]
use std::str::FromStr;
use std::error::Error;
use std::default;
use std::fmt;
use std::vec;
use std::collections::HashMap;
extern crate serde;


extern crate serde_json;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Protocol {
  name: String,
  protocol: Option<String>,
  doc: Option<String>,
  types: Vec<Type>,
  messages: Option<HashMap<String, Message>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  doc: Option<String>,
  request: Vec<Field>,
  response: Type,
  errors: Vec<Type>,
  one_way: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="order")]
enum Order {
  #[serde(rename="ascending")]
  Ascending,
  #[serde(rename="descending")]
  Descending,
  #[serde(rename="ignore")]
  Ignore
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="type")]
enum Type {
  #[serde(rename="null")]
  Null,
  #[serde(rename="boolean")]
  Boolean,
  #[serde(rename="int")]
  Int,
  #[serde(rename="long")]
  Long,
  #[serde(rename="float")]
  Float,
  #[serde(rename="double")]
  Double,
  #[serde(rename="bytes")]
  Bytes,
  #[serde(rename="string")]
  String,
  #[serde(rename="record")]
  Record(Record),
  #[serde(rename="enum")]
  Enum(Enum),
  #[serde(rename="array")]
  Array(Array),
  #[serde(rename="map")]
  Map(Map),
  #[serde(rename="union")]
  Union(Vec<Type>),
  #[serde(rename="fixed")]
  Fixed(Fixed)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="field")]
struct Field {
  name: String,
  doc: Option<String>,
  #[serde(rename="type")]
  av_type: Type,
  default: String,
  order: Option<Order>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="record")]
struct Record {
    name: String,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    fields: Vec<Field>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="enum")]
struct Enum {
    name: String,
    namespace: Option<String>,
    doc: Option<String>,
    aliases: Option<Vec<String>>,
    symbols: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="array")]
struct Array {
  items: Box<Type>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="map")]
struct Map {
  values: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename="fixed")]
struct Fixed {
  name: String,
  namespace: Option<String>,
  aliases: Option<Vec<String>>,
  size: i32
}

#[test]
#[should_panic]
fn it_works() {
  let arr: Array = Array {
    items: Box::new(Type::Null)
  };
  let field: Field = Field {
    name: String::from("null_array"),
    doc: None,
    av_type: Type::Array(arr),
    default: String::from("null"),
    order: Some(Order::Ascending)
  };
  let record: Record = Record {
    name: String::from("record"),
    namespace: None,
    doc: None,
    aliases: None,
    fields: vec![field]
  };

  let serialized = serde_json::to_string(&record).unwrap();
  println!("{}", serialized);
  let test = String::from("{\"namespace\":\"com.acme\",\"protocol\":\"HelloWorld\",\"doc\":\"ProtocolGreetings\",\"types\":[{\"name\":\"Greeting\",\"type\":\"record\",\"fields\":[{\"name\":\"message\",\"type\":\"string\"}]},{\"name\":\"Curse\",\"type\":\"error\",\"fields\":[{\"name\":\"message\",\"type\":\"string\"}]}],\"messages\":{\"hello\":{\"doc\":\"Sayhello.\",\"request\":[{\"name\":\"greeting\",\"type\":\"Greeting\"}],\"response\":\"Greeting\",\"errors\":[\"Curse\"]}}");
  let deserialized: Record = serde_json::from_str(&serialized).unwrap();
  println!("{:?}", deserialized);
  println!("{}",test);
}
