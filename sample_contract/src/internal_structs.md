# Internal structs
  

### enum wrapping

Because of the way information is stored in the NEAR protocol, it is a best practice to always wrap structs that are directly stored into the state in an enum.  
That is for upgradability purposes, being that in an application V2 we might need to change the data structure and we cannot risk having new and old implementations cohabiting the applications state.  
Wrapping the struct into an enum allows us to handle V1 and V2 data differently, effectively making a smooth contract upgrade.  
  
#### whenever you're storing a struct directly into the trie (that is, as a Contract field or inside a near_sdk collection such LookupMap, Vector, etc.) you should wrap it inside an enum.  
  
### Example
```
#[derive(BorshSerialize, BorshDeserialize)]
pub enum PersonalizedStruct {
    V1(PersonalizedStructV1)
}

#[allow(unreachable_patterns)]
impl PersonalizedStruct {
    pub fn new(data: u128, name: String) -> Self {
        PersonalizedStruct::V1(PersonalizedStructV1::new(data, name))
    }

    pub fn change_data(&mut self, new_data: u128) -> u128 {
        match self {
            PersonalizedStruct::V1(personalized_struct_v1) => {
                personalized_struct_v1.change_data(new_data)
            },
            _ => panic!("{}", ERR_201)
        }
    }

}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PersonalizedStructV1 {
    pub data: u128,
    pub last_data: Option<u128>,
    pub name: String
}

impl PersonalizedStructV1 {
    pub fn new(data: u128, name: String) -> Self {
        Self {
            data,
            last_data: None,
            name
        }
    }
