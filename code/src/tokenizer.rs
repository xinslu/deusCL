pub mod tokenizer{
  pub fn tokenize(expr: String) -> Vec<String> {
    expr
      .replace("(", " ( ")
      .replace(")", " ) ")
      .split_whitespace()
      .map(|x| x.to_string())
      .collect()
    }
}


