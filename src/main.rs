#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
use serde_xml_rs::deserialize;
use std::io::Read;
#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "vendorids")] pub vendor_ids: VendorIds,
    pub tags: Tags,
    #[serde(rename = "types")] pub types: Types, //types: Vec<Type>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorIds {
    #[serde(rename = "vendorid")] pub vendor_ids: Vec<VendorId>,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VendorId {
    pub name: String,
    pub id: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    #[serde(rename = "tag")] pub tags: Vec<Tag>,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub author: String,
    pub contact: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    //#[serde(rename = "type")]
    #[serde(rename = "$value")] pub types: Vec<Type>,
    pub comment: String,
}
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Category {
    #[serde(rename = "bitmask")] Bitmask,
    #[serde(rename = "basetype")] Basetype,
    #[serde(rename = "define")] Define,
    #[serde(rename = "include")] Include,
    #[serde(rename = "handle")] Handle,
    #[serde(rename = "enum")] Enum,
    #[serde(rename = "funcpointer")] FunctionPointer,
    #[serde(rename = "struct")] Struct,
    #[serde(rename = "union")] Union,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "type")]
    Type {
        #[serde(rename = "category")] category: Option<Category>,
        #[serde(rename = "name")] name: String,
        #[serde(rename = "requires")] requires: Option<String>,
        //#[serde(rename = "type")] ty: Option<String>,
    },
    #[serde(rename = "comment")] Comment(String),
}

fn main() {
    let mut file = std::fs::File::open("vk.xml").unwrap();
    let mut xml = String::new();
    file.read_to_string(&mut xml);
    let root: Registry = deserialize(xml.as_bytes()).expect("des");
    //println!("{:#?}", root.types.types.iter().take(15).collect::<Vec<_>>());
    println!(
        "{:#?}",
        root.types
            .types
            .iter()
            .filter(|t| {
                if let &&Type::Type { category, .. } = t {
                    return category == Some( Category::Basetype );
                }
                false
            })
            .collect::<Vec<_>>()
    );
    //println!("{:#?}", root.types.types);
}
