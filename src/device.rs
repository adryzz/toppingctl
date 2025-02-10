use hidapi::HidError;
const VID: u16 = 0x152a;
const PID: u16 = 0x8752;

pub struct E2x2 {
    dev: hidapi::HidDevice,
}

impl E2x2 {
    pub fn new(api: hidapi::HidApi) -> Result<Self, hidapi::HidError> {
        Ok(Self {
            dev: api.open(VID, PID)?,
        })
    }

    fn send_command(&mut self, cmd: &[u8]) -> Result<usize, HidError> {
        self.dev.write(&cmd)
    }

    pub fn auto_standby(&mut self, state: bool) -> Result<(), HidError> {
        if state {
            self.send_command(&commands::AUTO_STANDBY_ON)?;
        } else {
            self.send_command(&commands::AUTO_STANDBY_OFF)?;
        }
        Ok(())
    }

    pub fn mobile_mode(&mut self, state: bool) -> Result<(), HidError> {
        if state {
            self.send_command(&commands::MOBILE_MODE_ON)?;
        } else {
            self.send_command(&commands::MOBILE_MODE_OFF)?;
        }
        Ok(())
    }

    pub fn brightness(&mut self, value: usize) -> Result<(), HidError> {
        match value {
            0 => self.send_command(&commands::BRIGHTNESS_0)?,
            1 => self.send_command(&commands::BRIGHTNESS_1)?,
            2 => self.send_command(&commands::BRIGHTNESS_2)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn input_monitor(&mut self, input: usize, state: bool) -> Result<(), HidError> {
        match (input, state) {
            (0, true) => self.send_command(&commands::IN1_MONITOR_ON)?,
            (1, true) => self.send_command(&commands::IN2_MONITOR_ON)?,
            (0, false) => self.send_command(&commands::IN1_MONITOR_OFF)?,
            (1, false) => self.send_command(&commands::IN2_MONITOR_OFF)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn input_48v(&mut self, input: usize, state: bool) -> Result<(), HidError> {
        match (input, state) {
            (0, true) => self.send_command(&commands::IN1_48V_ON)?,
            (1, true) => self.send_command(&commands::IN2_48V_ON)?,
            (0, false) => self.send_command(&commands::IN1_48V_OFF)?,
            (1, false) => self.send_command(&commands::IN2_48V_OFF)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn input_inst(&mut self, input: usize, state: bool) -> Result<(), HidError> {
        match (input, state) {
            (0, true) => self.send_command(&commands::IN1_INST_ON)?,
            (1, true) => self.send_command(&commands::IN2_INST_ON)?,
            (0, false) => self.send_command(&commands::IN1_INST_OFF)?,
            (1, false) => self.send_command(&commands::IN2_INST_OFF)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn input_mute(&mut self, input: usize, state: bool) -> Result<(), HidError> {
        match (input, state) {
            (0, true) => self.send_command(&commands::IN1_MUTE_ON)?,
            (1, true) => self.send_command(&commands::IN2_MUTE_ON)?,
            (0, false) => self.send_command(&commands::IN1_MUTE_OFF)?,
            (1, false) => self.send_command(&commands::IN2_MUTE_OFF)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn input_o(&mut self, input: usize, state: bool) -> Result<(), HidError> {
        match (input, state) {
            (0, true) => self.send_command(&commands::IN1_O_ON)?,
            (1, true) => self.send_command(&commands::IN2_O_ON)?,
            (0, false) => self.send_command(&commands::IN1_O_OFF)?,
            (1, false) => self.send_command(&commands::IN2_O_OFF)?,
            _ => return Err(HidError::HidApiErrorEmpty),
        };

        Ok(())
    }

    pub fn headphones_gain(&mut self, state: bool) -> Result<(), HidError> {
        if state {
            self.send_command(&commands::HEADPHONES_GAIN_ON)?;
        } else {
            self.send_command(&commands::HEADPHONES_GAIN_OFF)?;
        }
        Ok(())
    }

    pub fn headphones_out(&mut self, state: bool) -> Result<(), HidError> {
        if state {
            self.send_command(&commands::HEADPHONES_ON_0)?;
            self.send_command(&commands::HEADPHONES_ON_1)?;
            self.send_command(&commands::HEADPHONES_ON_2)?;
            self.send_command(&commands::HEADPHONES_ON_3)?;
        } else {
            self.send_command(&commands::HEADPHONES_OFF_0)?;
            self.send_command(&commands::HEADPHONES_OFF_1)?;
            self.send_command(&commands::HEADPHONES_OFF_2)?;
            self.send_command(&commands::HEADPHONES_OFF_3)?;
        }
        Ok(())
    }

    pub fn line_out(&mut self, state: bool) -> Result<(), HidError> {
        if state {
            self.send_command(&commands::LINE_ON_0)?;
            self.send_command(&commands::LINE_ON_1)?;
            self.send_command(&commands::LINE_ON_2)?;
            self.send_command(&commands::LINE_ON_3)?;
        } else {
            self.send_command(&commands::LINE_OFF_0)?;
            self.send_command(&commands::LINE_OFF_1)?;
            self.send_command(&commands::LINE_OFF_2)?;
            self.send_command(&commands::LINE_OFF_3)?;
        }
        Ok(())
    }
}

mod commands {
    use hex_literal::hex;

    pub(crate) const AUTO_STANDBY_ON: [u8; 16] = hex!("2233200101100b000000010000667700");
    pub(crate) const AUTO_STANDBY_OFF: [u8; 16] = hex!("2233200101100b000000000000667700");

    pub(crate) const BRIGHTNESS_0: [u8; 16] = hex!("2233200101100a000000000000667700");
    pub(crate) const BRIGHTNESS_1: [u8; 16] = hex!("2233200101100a000000010000667700");
    pub(crate) const BRIGHTNESS_2: [u8; 16] = hex!("2233200101100a000000020000667700");

    pub(crate) const MOBILE_MODE_ON: [u8; 16] = hex!("2233200101100c000000010000667700");
    pub(crate) const MOBILE_MODE_OFF: [u8; 16] = hex!("2233200101100c000000000000667700");

    pub(crate) const IN1_MONITOR_ON: [u8; 16] = hex!("22332001012101000000010000667700");
    pub(crate) const IN1_MONITOR_OFF: [u8; 16] = hex!("22332001012101000000000000667700");

    pub(crate) const IN2_MONITOR_ON: [u8; 16] = hex!("22332001012201000000010000667700");
    pub(crate) const IN2_MONITOR_OFF: [u8; 16] = hex!("22332001012201000000000000667700");

    pub(crate) const IN1_48V_ON: [u8; 16] = hex!("22332001012102000000010000667700");
    pub(crate) const IN1_48V_OFF: [u8; 16] = hex!("22332001012102000000000000667700");

    pub(crate) const IN2_48V_ON: [u8; 16] = hex!("22332001012202000000010000667700");
    pub(crate) const IN2_48V_OFF: [u8; 16] = hex!("22332001012202000000000000667700");

    pub(crate) const IN1_INST_ON: [u8; 16] = hex!("22332001012103000000010000667700");
    pub(crate) const IN1_INST_OFF: [u8; 16] = hex!("22332001012103000000000000667700");

    pub(crate) const IN2_INST_ON: [u8; 16] = hex!("22332001012203000000010000667700");
    pub(crate) const IN2_INST_OFF: [u8; 16] = hex!("22332001012203000000000000667700");

    pub(crate) const IN1_MUTE_ON: [u8; 16] = hex!("22332001012104000000000000667700");
    pub(crate) const IN1_MUTE_OFF: [u8; 16] = hex!("22332001012104020000000000667700");

    pub(crate) const IN2_MUTE_ON: [u8; 16] = hex!("22332001012204000000000000667700");
    pub(crate) const IN2_MUTE_OFF: [u8; 16] = hex!("22332001012204020000000000667700");

    pub(crate) const IN1_O_ON: [u8; 16] = hex!("22332001012104fe0000000000667700");
    pub(crate) const IN1_O_OFF: [u8; 16] = hex!("22332001012104020000000000667700");

    pub(crate) const IN2_O_ON: [u8; 16] = hex!("22332001012204fe0000000000667700");
    pub(crate) const IN2_O_OFF: [u8; 16] = hex!("22332001012204020000000000667700");

    pub(crate) const HEADPHONES_GAIN_ON: [u8; 16] = hex!("22332001011102000000010000667700");
    pub(crate) const HEADPHONES_GAIN_OFF: [u8; 16] = hex!("22332001011102000000000000667700");

    pub(crate) const HEADPHONES_OFF_0: [u8; 16] = hex!("22332001013401000000000000667700");
    pub(crate) const HEADPHONES_OFF_1: [u8; 16] = hex!("22332001013402000000010000667700");
    pub(crate) const HEADPHONES_OFF_2: [u8; 16] = hex!("2233200101310101c8520e0000667700");
    pub(crate) const HEADPHONES_OFF_3: [u8; 16] = hex!("2233200101310201c8520e0000667700");

    pub(crate) const HEADPHONES_ON_0: [u8; 16] = hex!("22332001013401000000010000667700");
    pub(crate) const HEADPHONES_ON_1: [u8; 16] = hex!("22332001013402000000010000667700");
    pub(crate) const HEADPHONES_ON_2: [u8; 16] = hex!("2233200101310101c8520e0000667700");
    pub(crate) const HEADPHONES_ON_3: [u8; 16] = hex!("2233200101310201c8520e0000667700");

    pub(crate) const LINE_OFF_0: [u8; 16] = hex!("22332001013401000000010000667700");
    pub(crate) const LINE_OFF_1: [u8; 16] = hex!("22332001013402000000000000667700");
    pub(crate) const LINE_OFF_2: [u8; 16] = hex!("2233200101310101c8520e0000667700");
    pub(crate) const LINE_OFF_3: [u8; 16] = hex!("2233200101310201c8520e0000667700");

    pub(crate) const LINE_ON_0: [u8; 16] = hex!("22332001013401000000010000667700");
    pub(crate) const LINE_ON_1: [u8; 16] = hex!("22332001013402000000010000667700");
    pub(crate) const LINE_ON_2: [u8; 16] = hex!("2233200101310101c8520e0000667700");
    pub(crate) const LINE_ON_3: [u8; 16] = hex!("2233200101310201c8520e0000667700");

    pub(crate) const IN1_VOLUME_0:  [u8; 16] = hex!("22332001012104020000000000667700");
    pub(crate) const IN1_VOLUME_5:  [u8; 16] = hex!("22332001012104038e7a980000667700");
    pub(crate) const IN1_VOLUME_10: [u8; 16] = hex!("22332001012104065316180000667700");
    pub(crate) const IN1_VOLUME_15: [u8; 16] = hex!("223320010121040b3f30000000667700");
    pub(crate) const IN1_VOLUME_20: [u8; 16] = hex!("22332001012104140000000000667700");

    pub(crate) const IN2_VOLUME_0:  [u8; 16] = hex!("22332001012204020000000000667700");
    pub(crate) const IN2_VOLUME_5:  [u8; 16] = hex!("22332001012204038e7a980000667700");
    pub(crate) const IN2_VOLUME_10: [u8; 16] = hex!("22332001012204065316180000667700");
    pub(crate) const IN2_VOLUME_15: [u8; 16] = hex!("223320010122040b3f30000000667700");
    pub(crate) const IN2_VOLUME_20: [u8; 16] = hex!("22332001012204140000000000667700");
}
