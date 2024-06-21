use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::ops::get::GetOutput;

impl<T> Serialize for GetOutput<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GetOutput", 2)?;
        state.serialize_field(
            "consumed_capacity",
            &self
                .consumed_capacity
                .as_ref()
                .map(crate::aws_sdk::serialize::consumed_capacity_to_value),
        )?;
        state.serialize_field("item", &self.item)?;
        state.end()
    }
}

impl<'de, T> Deserialize<'de> for GetOutput<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            ConsumedCapacity,
            Item,
        }

        const FIELDS: &[&str] = &["consumed_capacity", "item"];

        struct GetOutputVisitor<'de, T>
        where
            T: Deserialize<'de>,
        {
            marker: std::marker::PhantomData<GetOutput<T>>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }

        impl<'de, T> Visitor<'de> for GetOutputVisitor<'de, T>
        where
            T: Deserialize<'de>,
        {
            type Value = GetOutput<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GetOutput")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut consumed_capacity = None;
                let mut item = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ConsumedCapacity => {
                            if consumed_capacity.is_some() {
                                return Err(de::Error::duplicate_field("consumed_capacity"));
                            }

                            let v: Option<serde_json::Value> = map.next_value()?;

                            consumed_capacity = if let Some(v) = v {
                                Some(
                                    crate::aws_sdk::serialize::value_to_consumed_capacity(v)
                                        .map_err(de::Error::custom)?,
                                )
                            } else {
                                None
                            };
                        }
                        Field::Item => {
                            if item.is_some() {
                                return Err(de::Error::duplicate_field("item"));
                            }

                            item = Some(map.next_value()?);
                        }
                    }
                }

                let item = item.ok_or_else(|| de::Error::missing_field("item"))?;

                Ok(GetOutput {
                    consumed_capacity,
                    item,
                })
            }
        }

        deserializer.deserialize_struct(
            "GetOutput",
            FIELDS,
            GetOutputVisitor {
                marker: std::marker::PhantomData::<GetOutput<T>>,
                lifetime: std::marker::PhantomData,
            },
        )
    }
}