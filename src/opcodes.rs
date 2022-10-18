use crate::Processor;

const LOW: &str = "LOW";
const HIGH: &str = "HIGH";

// LD $dest,NN -- loads 2 bytes from NN (PC) into $DEST
macro_rules! LDNN {
    ($proc:ident, $dest:ident) => {{
        LDN!($proc, $dest, "LOW");
        LDN!($proc, $dest, "HIGH");
    }};
}
// LD $dest.$which,N -- loads 1 bytes from N (PC) into $DEST
macro_rules! LDN {
    ($proc:ident, $dest:ident, $which:expr) => {{
        let val = $proc.memory.read($proc.pc.as_u16());
        if $which == LOW {
            $proc.$dest.set_low(val);
        } else {
            $proc.$dest.set_high(val);
        }
        $proc.pc.add(1);
    }};
}

// LD ($dest),$src -- loads 1 byte from N and stores it in the memory $dest
macro_rules! LDNP {
    ($proc:ident, $dest:ident, $src:expr) => {
        $proc.memory.set($proc.$dest.as_u16(), $src)
    };
}

// INC $dest -- adds 1 to the destination 16 bit register
macro_rules! INCNN {
    ($proc:ident, $dest:ident) => {
        $proc.$dest.add(1)
    };
}

// INC $dest.$which -- adds 1 to the destination 8 bit register
// Which should be constants LOW or HIGH
macro_rules! INCN {
    ($proc:ident, $dest:ident, $which:expr) => {
        if $which == LOW {
            $proc.$dest.set_low($proc.$dest.low() + 1);
        } else {
            $proc.$dest.set_high($proc.$dest.high() + 1);
        }
    };
}

// DEC INC $dest.$which -- subs 1 to the destination 8 bit register
// Which should be constants LOW or HIGH
macro_rules! DECN {
    ($proc:ident, $dest:ident, $which:expr) => {
        if $which == LOW {
            $proc.$dest.set_low($proc.$dest.low() - 1);
        } else {
            $proc.$dest.set_high($proc.$dest.high() - 1);
        }
    };
}

macro_rules! RLCN {
    ($proc:ident, $dest:ident, $which:expr) => {
        if $which = LOW {
            let val = $proc.$dest.low();
            $proc.$dest.set_low(val << 1);
            // TODO: this is a bit more complex, and involves setting flags
        }
    };
}

pub(crate) const OPCODES: [fn(&mut Processor); 0xFF] = [
    // 0x00 -- NOOP
    |proc: &mut Processor| {},
    |proc: &mut Processor| LDNN!(proc, bc),
    |proc: &mut Processor| LDNP!(proc, bc, proc.af.high()),
    |proc: &mut Processor| INCNN!(proc, bc),
    |proc: &mut Processor| INCN!(proc, bc, HIGH),
    |proc: &mut Processor| DECN!(proc, bc, HIGH),
    |proc: &mut Processor| LDN!(proc, bc, HIGH),
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    // 0x0A
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
    |proc: &mut Processor| {},
];
