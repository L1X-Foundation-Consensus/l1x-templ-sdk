use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

type AddressArray = [u8; 20];

#[derive(
    Debug,
    BorshSerialize,
    BorshDeserialize,
    Hash,
    BorshSchema,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct Address(AddressArray);

impl Default for Address {
    fn default() -> Self {
        Self([0u8; 20])
    }
}

impl Address {
    pub fn to_string(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    #[cfg(test)]
    pub fn test_create_address(address: &Vec<u8>) -> Self {
        let address: AddressArray = address.clone().try_into().unwrap();
        Address(address)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&hex::encode(self.0), f)
    }
}

impl TryFrom<Vec<u8>> for Address {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let len = value.len();
        match <AddressArray>::try_from(value) {
            Ok(address) => Ok(Self(address)),
            Err(_) => {
                Err(format!("Can't create address from vector length={}", len))
            }
        }
    }
}

impl TryFrom<&str> for Address {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match hex::decode(value) {
            Ok(val) => match <AddressArray>::try_from(val) {
                Ok(address) => Ok(Self(address)),
                Err(_) => Err(format!("Can't create address from vector")),
            },
            Err(_) => {
                Err(format!("Can't create address from string {}", value))
            }
        }
    }
}

impl TryFrom<String> for Address {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Address {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Serialize for Address {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Address::try_from(s).map_err(|err| serde::de::Error::custom(err))?)
    }
}

#[cfg(test)]
mod test {
    use crate::types::Address;

    #[test]
    pub fn address_try_from() {
        let addr_vec: Vec<u8> = vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44,
        ];
        let addr_str = hex::encode(addr_vec.clone());
        let test_addr = Address::test_create_address(&addr_vec);

        assert_eq!(Address::try_from(addr_str.clone()), Ok(test_addr));
        assert_eq!(Address::try_from(addr_str.as_str()), Ok(test_addr));
        assert_eq!(Address::try_from(addr_vec), Ok(test_addr));
    }

    #[test]
    pub fn address_try_from_incorrect() {
        let addr_vec_too_short: Vec<u8> = vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33,
        ];
        let addr_vec_too_long: Vec<u8> = vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44, 0xff,
        ];
        let addr_str_short = hex::encode(addr_vec_too_short.clone());
        let addr_str_long = hex::encode(addr_vec_too_long.clone());

        assert!(Address::try_from(addr_vec_too_short).is_err());
        assert!(Address::try_from(addr_vec_too_long).is_err());

        assert!(Address::try_from(addr_str_short.clone()).is_err());
        assert!(Address::try_from(addr_str_long.clone()).is_err());

        assert!(Address::try_from(addr_str_short.as_str()).is_err());
        assert!(Address::try_from(addr_str_long.as_str()).is_err());
    }

    #[test]
    pub fn address_to_string_vec() {
        let addr_vec: Vec<u8> = vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44,
        ];
        let address = Address::test_create_address(&addr_vec);

        assert_eq!(
            address.to_string(),
            "112233445566778899aabbccddeeff0011223344"
        );
        assert_eq!(address.to_vec(), addr_vec);
    }
}
