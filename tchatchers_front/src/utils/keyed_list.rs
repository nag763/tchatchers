use derive_more::{Deref, DerefMut, From};
use yew::AttrValue;

#[derive(Deref, DerefMut, From, PartialEq, Eq, Debug)]
pub struct KeyedList(Vec<(AttrValue, AttrValue)>);

impl From<Vec<(i32, String)>> for KeyedList {
    fn from(value: Vec<(i32, String)>) -> Self {
        KeyedList(
            value
                .into_iter()
                .map(|(key, val)| {
                    (
                        AttrValue::from(key.to_string()),
                        AttrValue::from(val.clone()),
                    )
                })
                .collect(),
        )
    }
}
