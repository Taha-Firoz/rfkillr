
use anyhow::{Result, anyhow};
use nix::unistd::write;
use tracing::{error, debug};

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, close};

use crate::consts::{RFKILL_PATH, RFKILL_SYS_PATH};
use crate::{CRfKillEvent, RFKILL_EVENT_SIZE, RfKillEvent};
pub struct RfKill {
    pub rfkill_fd: i32
}

impl RfKill{
    pub fn new() -> Result<Self>{
        let fd = open(RFKILL_PATH, OFlag::O_RDWR | OFlag::O_NONBLOCK, Mode::empty())?;
        Ok(RfKill { rfkill_fd: fd })
    }

    /// get name of rfkill device
    /// e.g. hci0
    pub fn get_name(device_idx: u32) -> Result<String> {
        let mut name = [0u8; 128];
    
        let filename = format!("{}{device_idx}/name", RFKILL_SYS_PATH);
        let fd = open(filename.as_str(), OFlag::O_RDONLY, Mode::empty())?;
    
        let read_result = read(fd, &mut name);
        if let Err(err) = read_result {
            close(fd)?;
            return Err(err)?;
        }
    
        let name_string = String::from_utf8_lossy(&name);
        let pos = name_string.find('\n').unwrap_or(name_string.len());
    
        close(fd)?;
        Ok(name_string[..pos].to_string())
    }
    
    /// returns all events made on rfkill since 
    /// rfkill object was created
    pub fn read_event(&self, mut event: CRfKillEvent) -> Result<RfKillEvent> {
        let bytes_to_read = read(self.rfkill_fd, unsafe {
            std::slice::from_raw_parts_mut(
                &mut event as *mut _ as *mut u8,
                RFKILL_EVENT_SIZE,
            )
        })?;
        if bytes_to_read == RFKILL_EVENT_SIZE {
            if let Ok(event) = RfKillEvent::try_from(event) {
                Ok(event)
            } else {
                Err(anyhow!("Failed to parse event"))
            }
        }else{
            Err(anyhow!("Invalid response length: {}", bytes_to_read))
        }
    }

    pub fn update_device(&self, event: CRfKillEvent) -> bool {
        debug!("Writing event to rfkill: {:?}", event);
        let write_result = write(self.rfkill_fd, unsafe {
            std::slice::from_raw_parts(
                (&event as *const CRfKillEvent) as *const u8,
                RFKILL_EVENT_SIZE,
            )
        });
        if let Err(err) = write_result {
            error!("Failed to change RFKILL state");
            error!("{}", err);
            false
        }else{
            true
        }
    }
}


impl Drop for RfKill{
    fn drop(&mut self) {
        close(self.rfkill_fd).unwrap()
    }
}
