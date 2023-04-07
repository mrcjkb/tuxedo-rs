use std::{collections::HashMap, sync::Arc};

use crate::{error::IoctlError, read, write};

use super::{HardwareDevice, IoctlResult, TdpDevice};

const MAX_FAN_SPEED: u8 = 0xc8;

const PERF_PROF_STR_BALANCED: &str = "power_save";
const PERF_PROF_STR_ENTHUSIAST: &str = "enthusiast";
const PERF_PROF_STR_OVERBOOST: &str = "overboost";

const PERF_PROFILE_MAP: [(&'static str, u8); 3] = [
    (PERF_PROF_STR_BALANCED, 0x01),
    (PERF_PROF_STR_ENTHUSIAST, 0x02),
    (PERF_PROF_STR_OVERBOOST, 0x03),
];

#[derive(Debug, Clone)]
pub struct UniwillHardware {
    file: Arc<std::fs::File>,
    num_of_fans: u8,
}

impl HardwareDevice for UniwillHardware {
    fn init(file: std::fs::File) -> IoctlResult<Self> {
        if read::uw::hw_check(&file)? == 1 {
            Ok(Self {
                file: Arc::new(file),
                num_of_fans: 2,
            })
        } else {
            Err(IoctlError::DevNotAvailable)
        }
    }

    fn device_interface_id_str(&self) -> IoctlResult<String> {
        Ok("uniwill".to_string())
    }

    fn device_model_id_str(&self) -> IoctlResult<String> {
        read::uw::model_id(&self.file).map(|id| id.to_string())
    }

    fn set_enable_mode_set(&self, enabled: bool) -> IoctlResult<()> {
        write::uw::mode_enable(&self.file, u32::from(enabled))
    }

    fn get_number_fans(&self) -> u8 {
        self.num_of_fans
    }

    fn set_fans_auto(&self) -> IoctlResult<()> {
        write::uw::fan_auto(&self.file, 0)
    }

    fn set_fan_speed_percent(&self, fan: u8, fan_speed_percent: u8) -> IoctlResult<()> {
        let fan_speed_raw =
            (MAX_FAN_SPEED as f64 * fan_speed_percent as f64 / 100.0).round() as u32;

        match fan {
            0 => write::uw::fan_speed_0(&self.file, fan_speed_raw),
            1 => write::uw::fan_speed_1(&self.file, fan_speed_raw),
            _ => Err(IoctlError::DevNotAvailable),
        }
    }

    fn get_fan_speed_percent(&self, fan: u8) -> IoctlResult<u8> {
        let fan_speed_raw = match fan {
            0 => read::uw::fan_speed_0(&self.file),
            1 => read::uw::fan_speed_1(&self.file),
            _ => Err(IoctlError::DevNotAvailable),
        }?;

        Ok((fan_speed_raw as f64 * 100.0 / MAX_FAN_SPEED as f64).round() as u8)
    }

    fn get_fan_temperature(&self, fan: u8) -> IoctlResult<u8> {
        let temp = match fan {
            0 => read::uw::fan_temp_0(&self.file),
            1 => read::uw::fan_temp_1(&self.file),
            _ => Err(IoctlError::DevNotAvailable),
        }?;

        // Also use known set value (0x00) from tccwmi to detect no temp/fan
        if temp == 0 {
            Err(IoctlError::DevNotAvailable)
        } else {
            Ok(temp as u8)
        }
    }

    fn get_fans_min_speed(&self) -> IoctlResult<u8> {
        let speed = read::uw::fans_min_speed(&self.file)?;
        Ok(u8::try_from(speed).unwrap_or_default())
    }

    fn get_fans_off_available(&self) -> IoctlResult<bool> {
        read::uw::fans_off_available(&self.file).map(|res| res == 1)
    }

    fn get_available_odm_performance_profiles(&self) -> IoctlResult<Vec<String>> {
        let available_profs = read::uw::profs_available(&self.file)?;
        Ok(match available_profs {
            2 => {
                vec![
                    PERF_PROF_STR_BALANCED.into(),
                    PERF_PROF_STR_ENTHUSIAST.into(),
                ]
            }
            3 => {
                vec![
                    PERF_PROF_STR_BALANCED.into(),
                    PERF_PROF_STR_ENTHUSIAST.into(),
                    PERF_PROF_STR_OVERBOOST.into(),
                ]
            }
            _ => {
                return Err(IoctlError::DevNotAvailable);
            }
        })
    }

    fn set_odm_performance_profile(&self, performance_profile: String) -> IoctlResult<()> {
        if let Some((_, id)) = PERF_PROFILE_MAP.iter().find(|(name, _)| name == &performance_profile) {
            write::uw::perf_prof(&self.file, *id as u32)
        } else {
            Err(IoctlError::InvalidArgs)
        }
    }

    fn get_default_odm_performance_profile(&self) -> IoctlResult<String> {
        todo!()
    }
}

impl TdpDevice for UniwillHardware {
    fn get_number_tdps(&self) -> IoctlResult<u8> {
        todo!()
    }

    fn get_tdp_descriptors(&self) -> IoctlResult<Vec<String>> {
        todo!()
    }

    fn get_tdp_min(&self, tdp_index: u8) -> IoctlResult<u8> {
        todo!()
    }

    fn get_tdp_max(&self, tdp_index: u8) -> IoctlResult<u8> {
        todo!()
    }

    fn set_tdp(&self, tdp_index: u8, tdp_value: u8) -> IoctlResult<()> {
        todo!()
    }

    fn get_tdp(&self, tdp_index: u8) -> IoctlResult<u8> {
        todo!()
    }
}
