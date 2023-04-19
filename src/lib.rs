pub mod rfkill;
use anyhow::Result;
#[cfg(feature = "serialization")]
use serde::Serialize;
#[cfg(feature = "serialization")]
use zvariant::{SerializeDict, Type};

#[cfg_attr(feature = "serialization", derive(Serialize, Type), zvariant(signature = "s"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RfkillType {
    All,
    Wlan,
    Bluetooth,
    Uwb,
    Wimax,
    Wwan,
    Gps,
    Fm,
    Nfc,
    NumRfkillTypes,
}

impl TryFrom<&str> for RfkillType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let res = match s {
            "All" => RfkillType::All,
            "Wireless LAN" => RfkillType::Wlan,
            "Bluetooth" => RfkillType::Bluetooth,
            "Ultra-Wideband" => RfkillType::Uwb,
            "WiMAX" => RfkillType::Wimax,
            "Wireless WAN" => RfkillType::Wwan,
            "GPS" => RfkillType::Gps,
            "FM" => RfkillType::Fm,
            "NFC" => RfkillType::Nfc,
            "NumRfkillTypes" => RfkillType::NumRfkillTypes,
            _ => return Err(()),
        };
        Ok(res)
    }
}

impl ToString for RfkillType {
    fn to_string(&self) -> std::string::String {
        let stringed = match self {
            RfkillType::All => "All",
            RfkillType::Wlan => "Wireless LAN",
            RfkillType::Bluetooth => "Bluetooth",
            RfkillType::Uwb => "Ultra-Wideband",
            RfkillType::Wimax => "WiMAX",
            RfkillType::Wwan => "Wireless WAN",
            RfkillType::Gps => "GPS",
            RfkillType::Fm => "FM",
            RfkillType::Nfc => "NFC",
            RfkillType::NumRfkillTypes => "NumRfkillTypes",
        };
        stringed.to_owned()
    }
}

use std::convert::TryFrom;
impl TryFrom<u8> for RfkillType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RfkillType::All),
            1 => Ok(RfkillType::Wlan),
            2 => Ok(RfkillType::Bluetooth),
            3 => Ok(RfkillType::Uwb),
            4 => Ok(RfkillType::Wimax),
            5 => Ok(RfkillType::Wwan),
            6 => Ok(RfkillType::Gps),
            7 => Ok(RfkillType::Fm),
            8 => Ok(RfkillType::Nfc),
            9 => Ok(RfkillType::NumRfkillTypes),
            _ => Err("Invalid value for RfkillType"),
        }
    }
}
use std::mem::size_of;

use crate::rfkill::RfKill;

pub const RFKILL_EVENT_SIZE: usize = size_of::<CRfKillEvent>();

///
/// Struct needed to be sent to rfkill
/// 
/// this is the legacy rfkill struct
/// the rfkill_ext is not implemented
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CRfKillEvent {
    idx: u32,
    event_type: u8,
    op: u8,
    soft: u8,
    hard: u8,
}

impl CRfKillEvent {
    pub fn set_idx(mut self, idx: u32) -> Self {
        self.idx = idx;
        self
    }
    pub fn set_event_type(mut self, event_type: RfkillType) -> Self {
        self.event_type = event_type as u8;
        self
    }
    pub fn set_op(mut self, op: RfkillOperations) -> Self {
        self.op = op as u8;
        self
    }
    pub fn soft_block(mut self) -> Self {
        self.soft = true as u8;
        self
    }
    pub fn soft_unblock(mut self) -> Self {
        self.soft = false as u8;
        self
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Type), zvariant(signature = "s"))]
#[derive( Debug, Clone, Copy, PartialEq,)]
///
/// Enum mapping the `op` field in rfkill
pub enum RfkillOperations {
    RfKillOpAdd,
    RfKillOpDel,
    RfKillOpChange,
    RfKillOpChangeAll,
}

impl TryFrom<u8> for RfkillOperations {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RfkillOperations::RfKillOpAdd),
            1 => Ok(RfkillOperations::RfKillOpDel),
            2 => Ok(RfkillOperations::RfKillOpChange),
            3 => Ok(RfkillOperations::RfKillOpChangeAll),
            _ => Err("Invalid value for RfkillOperations"),
        }
    }
}
#[cfg_attr(feature = "serialization", derive(SerializeDict, Type), zvariant(signature = "a{sv}"))]
#[derive(Clone)]
/// Enum mapping the `op` field in rfkill in a more rusty
/// accessible way
pub struct RfKillEvent {
    pub device_index: u32,
    pub device: RfkillType,
    pub operation: RfkillOperations,
    pub is_soft: bool,
    pub is_hard: bool,
}

impl RfKillEvent{
    pub fn get_name(&self) -> Result<String> {
        RfKill::get_name(self.device_index)
    }
}

impl TryFrom<CRfKillEvent> for RfKillEvent {
    type Error = &'static str;

    fn try_from(s: CRfKillEvent) -> Result<Self, Self::Error> {
        Ok(RfKillEvent {
            device_index: s.idx,
            device: RfkillType::try_from(s.event_type)?,
            // Disabling for now because it's not valid when this struct is used
            // for listing devices
            operation: RfkillOperations::try_from(s.op)?,
            is_soft: s.soft == 1,
            is_hard: s.hard == 2,
        })
    }
}

pub mod consts;