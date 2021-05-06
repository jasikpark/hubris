#![no_std]

use num_traits::float::FloatCore;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Command {
    Page = 0x00,
    Operation = 0x01,
    OnOffConfig = 0x02,
    ClearFaults = 0x03,
    Phase = 0x04,
    PagePlusWrite = 0x05,
    PagePlusRead = 0x06,
    ZoneConfig = 0x07,
    ZoneActive = 0x08,
    WriteProtect = 0x10,
    StoreDefaultAll = 0x11,
    RestoreDefaultAll = 0x12,
    StoreDefaultCode = 0x13,
    RestoreDefaultCode = 0x14,
    StoreUserAll = 0x15,
    RestoreUserAll = 0x16,
    StoreUserCode = 0x17,
    RestoreUserCode = 0x18,
    Capability = 0x19,
    VOutMode = 0x20,
    VOutCommand = 0x21,
    VOutTrim = 0x22,
    VOutCalOffset = 0x23,
    VOutMax = 0x24,
    VOutMarginHigh = 0x25,
    VOutMarginLow = 0x26,
    VOutTransitionRate = 0x27,
    VOutDroop = 0x28,
    VOutScaleLoop = 0x29,
    VOutScaleMonitor = 0x2a,
    VOutMin = 0x2b,
    Coefficients = 0x30,
    POutMax = 0x31,
    MaxDuty = 0x32,
    FrequencySwitch = 0x33,
    PowerMode = 0x34,
    VInOn = 0x35,
    VInOff = 0x36,
    Interleave = 0x37,
    IOutCalGain = 0x38,
    IOutCalOffset = 0x39,
    FanConfig12 = 0x3a,
    FanCommand1 = 0x3b,
    FanCommand2 = 0x3c,
    FanCommand3 = 0x3e,
    FanCommand4 = 0x3f,
    VOutOVFaultLimit = 0x40,
    VOutOVFaultResponse = 0x41,
    VOutOVWarnLimit = 0x42,
    VOutUVWarnLimit = 0x43,
    VOutUVFaultLimit = 0x44,
    VOutUVFaultResponse = 0x45,
    IOutOCFaultLimit = 0x46,
    IOutOCFaultResponse = 0x47,
    IOutOCLVFaultLimit = 0x48,
    IOutOCLVFaultResponse = 0x49,
    IOutOCWarnLimit = 0x4a,
    IOutUCFaultLimit = 0x4b,
    IOutUCFaultResponse = 0x4c,
    OTFaultLimit = 0x4f,
    OTFaultResponse = 0x50,
    OTWarnLimit = 0x51,
    UTWarnLimit = 0x52,
    UTFaultLimit = 0x53,
    UTFaultResponse = 0x54,
    VInOVFaultLimit = 0x55,
    VInOVFaultResponse = 0x56,
    VInOVWarnLimit = 0x57,
    VInUVWarnLimit = 0x58,
    VInUVFaultLimit = 0x59,
    VInUVFaultResponse = 0x5a,
    IInOCFaultLimit = 0x5b,
    IInOCFaultReponse = 0x5c,
    IInOCCWarnLimit = 0x5d,
    PowerGoodOn = 0x5e,
    PowerGoodOff = 0x5f,
    TOnDelay = 0x60,
    TOnRise = 0x61,
    TOnMaxFaultLimit = 0x62,
    TOnMaxFaultResponse = 0x63,
    TOffDelay = 0x64,
    TOffFall = 0x65,
    TOffMaxWarnLimit = 0x66,
    Deprecated = 0x67,
    POutOPFaultLimit = 0x68,
    POutOPFaultResponse = 0x69,
    POutOPWarnLimit = 0x6a,
    PInOPWarnLimit = 0x6b,
    StatusByte = 0x78,
    StatusWord = 0x79,
    StatusVOut = 0x7a,
    StatusIOut = 0x7b,
    StatusInput = 0x7c,
    StatusTemperature = 0x7d,
    StatusCML = 0x7e,
    StatusOther = 0x7f,
    StatusManufacturerSpecific = 0x80,
    StatusFans12 = 0x81,
    StautsFans34 = 0x82,
    ReadKWHIn = 0x83,
    ReadKWHOut = 0x84,
    ReadHWHConfig = 0x85,
    ReadEIn = 0x86,
    ReadEOut = 0x87,
    ReadVIn = 0x88,
    ReadIIn = 0x89,
    ReadVCap = 0x8a,
    ReadVOut = 0x8b,
    ReadIOut = 0x8c,
    ReadTemperature1 = 0x8d,
    ReadTemperature2 = 0x8e,
    ReadTemperature3 = 0x8f,
    ReadFanSpeed1 = 0x90,
    ReadFanSpeed2 = 0x91,
    ReadFanSpeed3 = 0x92,
    ReadFanSpeed4 = 0x93,
    ReadDutyCycle = 0x94,
    ReadFrequency = 0x95,
    ReadPOut = 0x96,
    ReadPIn = 0x97,
    PMBusRevision = 0x98,
    ManufacturerID = 0x99,
    ManufacturerModel = 0x9a,
    ManufacturerRevision = 0x9b,
    ManufacturerLocation = 0x9c,
    ManufacturerDate = 0x9d,
    ManufacturerSerial = 0x9e,
    AppProfileSupport = 0x9f,
    ManufacturerVInMin = 0xa0,
    ManufacturerVInMax = 0xa1,
    ManufacturerIInMax = 0xa2,
    ManufacturerPInMax = 0xa3,
    ManufacturerVOutMin = 0xa4,
    ManufacturerVOutMax = 0xa5,
    ManufacturerIOutMax = 0xa6,
    ManufacturerPOutMax = 0xa7,
    ManufacturerTAmbientMax = 0xa8,
    ManufacturerTAmbientMin = 0xa9,
    ManufacturerEfficiencyLL = 0xaa,
    ManufacturerEfficiencyHL = 0xab,
    ManufacturerPinAccuracy = 0xac,
    ICDeviceID = 0xad,
    ICDeviceRev = 0xae,
    UserData00 = 0xb0,
    UserData01 = 0xb1,
    UserData02 = 0xb2,
    UserData03 = 0xb3,
    UserData04 = 0xb4,
    UserData05 = 0xb5,
    UserData06 = 0xb6,
    UserData07 = 0xb7,
    UserData08 = 0xb8,
    UserData09 = 0xb9,
    UserData10 = 0xba,
    UserData11 = 0xbb,
    UserData12 = 0xbc,
    UserData13 = 0xbd,
    UserData14 = 0xbe,
    UserData15 = 0xbf,
    ManufacturerMaxTemp1 = 0xc0,
    ManufacturerMaxTemp2 = 0xc1,
    ManufacturerMaxTemp3 = 0xc2,
    ManufacturerSpecificCommandExtended = 0xfe,
    PMBusCommandExtended = 0xff,
}

///
/// The coefficients spelled out by PMBus for use in th DIRECT data format
/// (Part II, Sec. 7.4). The actual values used will depend on the device and
/// the condition.
///
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Coefficients {
    /// Slope coefficient. Two byte, signed.
    m: i16,
    /// Offset. Two-byte, signed.
    b: i16,
    /// Exponent. One-byte, signed.
    R: i8,
}

///
/// A datum in the DIRECT data format.
///
#[derive(Copy, Clone, Debug)]
struct Direct(u16, Coefficients);

impl Direct {
    #[allow(dead_code)]
    fn to_real(&self) -> f32 {
        let coefficients = &self.1;
        let m: f32 = coefficients.m.into();
        let b: f32 = coefficients.b.into();
        let exp: i32 = coefficients.R.into();
        let y: f32 = self.0.into();

        (y * f32::powi(10.0, -exp) - b) / m
    }

    #[allow(dead_code)]
    fn from_real(x: f32, coefficients: Coefficients) -> Self {
        let m: f32 = coefficients.m.into();
        let b: f32 = coefficients.b.into();
        let exp: i32 = coefficients.R.into();
        let y: f32 = (m * x + b) * f32::powi(10.0, exp);

        Self(y.round() as u16, coefficients)
    }
}
