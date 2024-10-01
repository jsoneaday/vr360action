use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use std::mem::size_of;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RpcParams {
    //Objects(Vec<Object>),
    Message(HashMap<String, String>),
    Array(Vec<String>)
}

#[derive(Debug)]
#[repr(C)]
pub struct RpcRequest {
    pub id: String,
    pub params: RpcParams
}

#[allow(unused)]
impl RpcRequest {
    pub fn new(id: String, params: RpcParams) -> Self {
        RpcRequest { 
            id, 
            params
        }
    }
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let mut state = serializer.serialize_struct("RpcRequest", size_of::<RpcRequest>())?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        enum Field { Id, Params }

        impl <'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error> 
            where 
                D: serde::Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`id or `params")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E> 
                    where
                        E: serde::de::Error
                    {
                        match value {
                            "id" => Ok(Field::Id),
                            "params" => Ok(Field::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS))
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RpcRequestVisitor;

        impl<'de> serde::de::Visitor<'de> for RpcRequestVisitor {
            type Value = RpcRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct RpcRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RpcRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>
            {
                let mut id = None;
                let mut params = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::Params => {
                            if params.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let params = params.ok_or_else(|| serde::de::Error::missing_field("params"))?;
                Ok(RpcRequest{ id, params })
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "params"];
        deserializer.deserialize_struct("RpcRequest", FIELDS, RpcRequestVisitor)        
    }
}