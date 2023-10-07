use crate::packet_objects::basics::FieldType;
use crate::traits::Processable;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize)]
pub struct EthernetHeader {
    pub source_mac: String,
    pub destination_mac: String,
    pub ether_type: FieldType,
    pub payload: Vec<u8>,
}

impl<'a> Processable<'a, EthernetHeader> for EthernetPacket<'a> {
    fn process(&self) -> EthernetHeader {
        EthernetHeader {
            source_mac: self.get_source().to_string(),
            destination_mac : self.get_destination().to_string(),
            ether_type :EthernetHeader::get_ethernet_fieldtype(self.get_ethertype().0),
            payload : self.payload().to_vec(),
        }
    }
}
impl EthernetHeader {
    pub fn get_ethernet_fieldtype(number: u16) -> FieldType {
        let name: String = match &number {
            0x0800 => "IPv4".to_string(),
            0x0806 => "ARP".to_string(),
            0x86DD => "IPv6".to_string(),
            _ => "Unknown".to_string(),
        };
        FieldType {
            field_name: name,
            num: number,
        }
    }
}