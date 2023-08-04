use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph {
  name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
  article: String,
  author: String,
  paragraph: Vec<Paragraph>
}

fn read_json_typed(raw_json:&str) -> Article{
    let parsed:Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}

fn main(){

    let json = 
    r#"
{
  "article": "how to work with json in Rust",
  "author": "Vishal Saroj",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;

let parsed:Article = read_json_typed(json);
println!("The name of the middle paragraph is : {}",parsed.paragraph[1].name);
}

