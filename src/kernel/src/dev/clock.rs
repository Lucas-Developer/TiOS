/*  System clock module
 *  Written by Andrew Jianzhong Liu
 *  All Rights Reserved
 */

use super::*;

const CMOS_ADDR: u8 = 0x70;
const CMOS_DATA: u8 = 0x71;

pub struct RealTimeClock {
    address_port: Port<u8>,
    data_port: Port<u8>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct DateTime {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

impl DateTime {
    pub fn update(&mut self, rhs: DateTime){
        self.year = rhs.year;
        self.month = rhs.month;
        self.day = rhs.day;
        self.hour = rhs.hour;
        self.min = rhs.min;
        self.sec = rhs.sec;
    }
}

use core::ops::Sub;
impl Sub for DateTime{
    type Output=isize;
    fn sub(self, rhs: DateTime) -> Self::Output {
        let dyear = self.year - rhs.year;
        let dmonth = self.month - rhs.month;
        let dday = self.day - rhs.day;
        let dhour = self.hour - rhs.hour;
        let dmin = self.min - rhs.min;
        let dsec = self.sec - rhs.sec;

        let diff_up_to_day = dsec as isize + (dmin as isize) * 60 + (dhour as isize) * 60 * 60 + (dday as isize) * 60 * 60 * 24;
        if dmonth != 0 || dyear != 0 {
            unimplemented!()
        }

        diff_up_to_day
    }
}

pub use core::fmt;

impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:02}:{:02}:{:02} {}/{}/{}", self.hour, self.min, self.sec, self.day, self.month, self.year)
    }
}

impl RealTimeClock {
    pub const fn new() -> RealTimeClock{
        RealTimeClock{
            address_port: unsafe{ Port::new(CMOS_ADDR as u16) },
            data_port: unsafe{ Port::new(CMOS_DATA as u16) },
        }
    }

    pub fn read_rtc(&mut self) -> DateTime {
        // Check if clock is currently getting updated.

        let mut last_second;
        let mut last_minute;
        let mut last_hour;
        let mut last_day;
        let mut last_month;
        let mut last_year;

        while self.get_update_in_progress() {}
        let mut second = self.get_rtc_reg(0x00);
        let mut minute = self.get_rtc_reg(0x02);
        let mut hour = self.get_rtc_reg(0x04);
        let mut day = self.get_rtc_reg(0x07);
        let mut month = self.get_rtc_reg(0x08);
        let mut year = self.get_rtc_reg(0x09);

        let registerB = self.get_rtc_reg(0x0b);

        while {
            last_second = second;
            last_minute = minute;
            last_hour = hour;
            last_day = day;
            last_month = month;
            last_year = year;
            while self.get_update_in_progress() {}
            second = self.get_rtc_reg(0x00);
            minute = self.get_rtc_reg(0x02);
            hour = self.get_rtc_reg(0x04);
            day = self.get_rtc_reg(0x07);
            month = self.get_rtc_reg(0x08);
            year = self.get_rtc_reg(0x09);
            (last_second != second) || (last_minute != minute) || (last_hour != hour) ||
               (last_day != day) || (last_month != month) || (last_year != year)
        }{}

        // Convert BCD to binary values if necessary
        if !(registerB & 0x04 != 0) {
            second = (second & 0x0F) + ((second / 16) * 10);
            minute = (minute & 0x0F) + ((minute / 16) * 10);
            hour = ( (hour & 0x0F) + (((hour & 0x70) / 16) * 10) ) | (hour & 0x80);
            day = (day & 0x0F) + ((day / 16) * 10);
            month = (month & 0x0F) + ((month / 16) * 10);
            year = (year & 0x0F) + ((year / 16) * 10);
        }

        DateTime{
            year: year,
            month: month,
            day: day,
            hour: hour,
            min: minute,
            sec: second,
        }
    }

    fn get_rtc_reg(&mut self, reg: u8) -> u8 {
        self.address_port.write(reg);
        self.data_port.read()
    }

    fn get_update_in_progress(&mut self) -> bool {
        self.address_port.write(0x0A);
        self.data_port.read() as i32 & 0x80 != 0
    }
}

use spin::Mutex;
pub static RTC: Mutex<RealTimeClock> = Mutex::new(RealTimeClock::new());