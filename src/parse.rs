
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
    match &s[..] {
      "null" => Ok(Type::Null),
      "true"|"false" => Ok(Type::Boolean),
       _     => Ok(Type::Null),
    }
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
