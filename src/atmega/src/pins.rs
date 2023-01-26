use crate::registers::{ PINx, DDRx, PORTx, PINB, DDRB, PORTB, PINC, DDRC, PORTC, PIND, DDRD, PORTD };

#[derive(Debug, Clone)]
pub enum Pin {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
}

impl Pin {
    fn analog(&self) {
        match self {
            Self::D0 => true,
            Self::D1 => true,
            Self::D2 => true,
            Self::D3 => true,
            Self::D4 => true,
            Self::D5 => true,
            Self::D6 => true,
            Self::D7 => true,
            Self::D8 => true,
            Self::D9 => true,
            Self::D10 => true,
            Self::D11 => true,
            Self::D12 => true,
            Self::D13 => true,
            Self::A0 => false,
            Self::A1 => false,
            Self::A2 => false,
            Self::A3 => false,
            Self::A4 => false,
            Self::A5 => false,
        }
    }
}

impl From<Pin> for Registers {
    fn from(value: Pin) -> Registers {
        match value {
            Pin::D0  => Registers::D(0),
            Pin::D1  => Registers::D(1),
            Pin::D2  => Registers::D(2),
            Pin::D3  => Registers::D(3),
            Pin::D4  => Registers::D(4),
            Pin::D5  => Registers::D(5),
            Pin::D6  => Registers::D(6),
            Pin::D7  => Registers::D(7),
            Pin::D8  => Registers::B(0),
            Pin::D9  => Registers::B(1),
            Pin::D10 => Registers::B(2),
            Pin::D11 => Registers::B(3),
            Pin::D12 => Registers::B(4),
            Pin::D13 => Registers::B(5),
            Pin::A0  => Registers::C(0),
            Pin::A1  => Registers::C(1),
            Pin::A2  => Registers::C(2),
            Pin::A3  => Registers::C(3),
            Pin::A4  => Registers::C(4),
            Pin::A5  => Registers::C(5),
        }
    }
}

#[derive(Debug, Clone)]
enum Registers {
    B(u8),
    C(u8),
    D(u8),
}

impl Registers {
    fn pinx(&self) -> PINx {
        match self {
            Self::B(offset) => {
                match offset {
                    0 => PINx::B(PINB::PINB0),
                    1 => PINx::B(PINB::PINB1),
                    2 => PINx::B(PINB::PINB2),
                    3 => PINx::B(PINB::PINB3),
                    4 => PINx::B(PINB::PINB4),
                    5 => PINx::B(PINB::PINB5),
                    6 => PINx::B(PINB::PINB6),
                    7 => PINx::B(PINB::PINB7),
                    _ => unreachable!(),
                }
            },
            Self::C(offset) => {
                match offset {
                    0 => PINx::C(PINC::PINC0),
                    1 => PINx::C(PINC::PINC1),
                    2 => PINx::C(PINC::PINC2),
                    3 => PINx::C(PINC::PINC3),
                    4 => PINx::C(PINC::PINC4),
                    5 => PINx::C(PINC::PINC5),
                    6 => PINx::C(PINC::PINC6),
                    _ => unreachable!(),
                }
            },
            Self::D(offset) => {
                match offset {
                    0 => PINx::D(PIND::PIND0),
                    1 => PINx::D(PIND::PIND1),
                    2 => PINx::D(PIND::PIND2),
                    3 => PINx::D(PIND::PIND3),
                    4 => PINx::D(PIND::PIND4),
                    5 => PINx::D(PIND::PIND5),
                    6 => PINx::D(PIND::PIND6),
                    7 => PINx::D(PIND::PIND7),
                    _ => unreachable!(),
                }
            },
        }
    }

    fn ddrx(&self) -> DDRx {
        match self {
            Self::B(offset) => {
                match offset {
                    0 => DDRx::B(DDRB::DDRB0),
                    1 => DDRx::B(DDRB::DDRB1),
                    2 => DDRx::B(DDRB::DDRB2),
                    3 => DDRx::B(DDRB::DDRB3),
                    4 => DDRx::B(DDRB::DDRB4),
                    5 => DDRx::B(DDRB::DDRB5),
                    6 => DDRx::B(DDRB::DDRB6),
                    7 => DDRx::B(DDRB::DDRB7),
                    _ => unreachable!(),
                }
            },
            Self::C(offset) => {
                match offset {
                    0 => DDRx::C(DDRC::DDRC0),
                    1 => DDRx::C(DDRC::DDRC1),
                    2 => DDRx::C(DDRC::DDRC2),
                    3 => DDRx::C(DDRC::DDRC3),
                    4 => DDRx::C(DDRC::DDRC4),
                    5 => DDRx::C(DDRC::DDRC5),
                    6 => DDRx::C(DDRC::DDRC6),
                    _ => unreachable!(),
                }
            },
            Self::D(offset) => {
                match offset {
                    0 => DDRx::D(DDRD::DDRD0),
                    1 => DDRx::D(DDRD::DDRD1),
                    2 => DDRx::D(DDRD::DDRD2),
                    3 => DDRx::D(DDRD::DDRD3),
                    4 => DDRx::D(DDRD::DDRD4),
                    5 => DDRx::D(DDRD::DDRD5),
                    6 => DDRx::D(DDRD::DDRD6),
                    7 => DDRx::D(DDRD::DDRD7),
                    _ => unreachable!(),
                }
            },
        }
    }

    fn portx(&self) -> PORTx {
        match self {
            Self::B(offset) => {
                match offset {
                    0 => PORTx::B(PORTB::PORTB0),
                    1 => PORTx::B(PORTB::PORTB1),
                    2 => PORTx::B(PORTB::PORTB2),
                    3 => PORTx::B(PORTB::PORTB3),
                    4 => PORTx::B(PORTB::PORTB4),
                    5 => PORTx::B(PORTB::PORTB5),
                    6 => PORTx::B(PORTB::PORTB6),
                    7 => PORTx::B(PORTB::PORTB7),
                    _ => unreachable!(),
                }
            },
            Self::C(offset) => {
                match offset {
                    0 => PORTx::C(PORTC::PORTC0),
                    1 => PORTx::C(PORTC::PORTC1),
                    2 => PORTx::C(PORTC::PORTC2),
                    3 => PORTx::C(PORTC::PORTC3),
                    4 => PORTx::C(PORTC::PORTC4),
                    5 => PORTx::C(PORTC::PORTC5),
                    6 => PORTx::C(PORTC::PORTC6),
                    _ => unreachable!(),
                }
            },
            Self::D(offset) => {
                match offset {
                    0 => PORTx::D(PORTD::PORTD0),
                    1 => PORTx::D(PORTD::PORTD1),
                    2 => PORTx::D(PORTD::PORTD2),
                    3 => PORTx::D(PORTD::PORTD3),
                    4 => PORTx::D(PORTD::PORTD4),
                    5 => PORTx::D(PORTD::PORTD5),
                    6 => PORTx::D(PORTD::PORTD6),
                    7 => PORTx::D(PORTD::PORTD7),
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum PinMode {
    INPUT,
    INPUT_PULLUP,
    OUTPUT,
}

pub const HIGH: bool = true;
pub const LOW: bool = false;

pub fn pin_mode(pin: Pin, value: PinMode) {
    let register = Registers::from(pin.clone()).ddrx();
    match value {
        PinMode::INPUT => unsafe { register.clear(); },
        PinMode::OUTPUT => unsafe { register.set(); },
        PinMode::INPUT_PULLUP => {
            unsafe { register.clear(); }
            digital_write(pin, HIGH);
        },
    }
}

pub fn digital_write(pin: Pin, value: bool) {
    let register = Registers::from(pin).portx();
    unsafe { register.set_value(value); }
}

pub fn digital_read(pin: Pin) -> bool {
    let register = Registers::from(pin).pinx();
    unsafe { register.read() }
}

pub fn digital_toggle(pin: Pin) {
    let register = Registers::from(pin).portx();
    unsafe { register.toggle(); }
}

pub fn analog_read(pin: Pin) -> u8 {
    if pin.digital() {
        let value = digital_read(pin);
        return if value { 255 } else { 0 };
    }
}

pub fn analog_write(pin: Pin, value: u8) {
    
} 
