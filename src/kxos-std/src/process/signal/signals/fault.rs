use kxos_frame::cpu::TrapInformation;
use kxos_frame::trap::{
    ALIGNMENT_CHECK, BOUND_RANGE_EXCEEDED, DIVIDE_BY_ZERO, GENERAL_PROTECTION_FAULT,
    INVALID_OPCODE, PAGE_FAULT, SIMD_FLOATING_POINT_EXCEPTION, X87_FLOATING_POINT_EXCEPTION,
};

use crate::prelude::*;
use crate::process::signal::c_types::siginfo_t;
use crate::process::signal::constants::*;
use crate::process::signal::sig_num::SigNum;

use super::Signal;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaultSignal {
    num: SigNum,
    code: i32,
    addr: Option<u64>,
}

impl FaultSignal {
    pub fn new(trap_info: &TrapInformation) -> FaultSignal {
        debug!("Trap id: {}", trap_info.id);
        let (num, code, addr) = match trap_info.id {
            DIVIDE_BY_ZERO => (SIGFPE, FPE_INTDIV, None),
            X87_FLOATING_POINT_EXCEPTION | SIMD_FLOATING_POINT_EXCEPTION => {
                (SIGFPE, FPE_FLTDIV, None)
            }
            BOUND_RANGE_EXCEEDED => (SIGSEGV, SEGV_BNDERR, None),
            ALIGNMENT_CHECK => (SIGBUS, BUS_ADRALN, None),
            INVALID_OPCODE => (SIGILL, ILL_ILLOPC, None),
            GENERAL_PROTECTION_FAULT => (SIGBUS, BUS_ADRERR, None),
            PAGE_FAULT => {
                const PF_ERR_FLAG_PRESENT: u64 = 1u64 << 0;
                let code = if trap_info.err & PF_ERR_FLAG_PRESENT != 0 {
                    SEGV_ACCERR
                } else {
                    SEGV_MAPERR
                };
                let addr = Some(trap_info.cr2);
                (SIGSEGV, code, addr)
            }
            _ => panic!("Exception cannnot be a signal"),
        };
        FaultSignal { num, code, addr }
    }
}

impl Signal for FaultSignal {
    fn num(&self) -> SigNum {
        self.num
    }

    fn to_info(&self) -> siginfo_t {
        siginfo_t::new(self.num, self.code)
        // info.set_si_addr(self.addr.unwrap_or_default() as *const c_void);
        // info
    }
}