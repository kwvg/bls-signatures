#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const __API_TO_BE_DEPRECATED: u32 = 100000;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __MAC_10_14: u32 = 101400;
pub const __MAC_10_14_1: u32 = 101401;
pub const __MAC_10_14_4: u32 = 101404;
pub const __MAC_10_14_6: u32 = 101406;
pub const __MAC_10_15: u32 = 101500;
pub const __MAC_10_15_1: u32 = 101501;
pub const __MAC_10_15_4: u32 = 101504;
pub const __MAC_10_16: u32 = 101600;
pub const __MAC_11_0: u32 = 110000;
pub const __MAC_11_1: u32 = 110100;
pub const __MAC_11_3: u32 = 110300;
pub const __MAC_11_4: u32 = 110400;
pub const __MAC_11_5: u32 = 110500;
pub const __MAC_11_6: u32 = 110600;
pub const __MAC_12_0: u32 = 120000;
pub const __MAC_12_1: u32 = 120100;
pub const __MAC_12_2: u32 = 120200;
pub const __MAC_12_3: u32 = 120300;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __IPHONE_11_4: u32 = 110400;
pub const __IPHONE_12_0: u32 = 120000;
pub const __IPHONE_12_1: u32 = 120100;
pub const __IPHONE_12_2: u32 = 120200;
pub const __IPHONE_12_3: u32 = 120300;
pub const __IPHONE_12_4: u32 = 120400;
pub const __IPHONE_13_0: u32 = 130000;
pub const __IPHONE_13_1: u32 = 130100;
pub const __IPHONE_13_2: u32 = 130200;
pub const __IPHONE_13_3: u32 = 130300;
pub const __IPHONE_13_4: u32 = 130400;
pub const __IPHONE_13_5: u32 = 130500;
pub const __IPHONE_13_6: u32 = 130600;
pub const __IPHONE_13_7: u32 = 130700;
pub const __IPHONE_14_0: u32 = 140000;
pub const __IPHONE_14_1: u32 = 140100;
pub const __IPHONE_14_2: u32 = 140200;
pub const __IPHONE_14_3: u32 = 140300;
pub const __IPHONE_14_5: u32 = 140500;
pub const __IPHONE_14_6: u32 = 140600;
pub const __IPHONE_14_7: u32 = 140700;
pub const __IPHONE_14_8: u32 = 140800;
pub const __IPHONE_15_0: u32 = 150000;
pub const __IPHONE_15_1: u32 = 150100;
pub const __IPHONE_15_2: u32 = 150200;
pub const __IPHONE_15_3: u32 = 150300;
pub const __IPHONE_15_4: u32 = 150400;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __TVOS_11_4: u32 = 110400;
pub const __TVOS_12_0: u32 = 120000;
pub const __TVOS_12_1: u32 = 120100;
pub const __TVOS_12_2: u32 = 120200;
pub const __TVOS_12_3: u32 = 120300;
pub const __TVOS_12_4: u32 = 120400;
pub const __TVOS_13_0: u32 = 130000;
pub const __TVOS_13_2: u32 = 130200;
pub const __TVOS_13_3: u32 = 130300;
pub const __TVOS_13_4: u32 = 130400;
pub const __TVOS_14_0: u32 = 140000;
pub const __TVOS_14_1: u32 = 140100;
pub const __TVOS_14_2: u32 = 140200;
pub const __TVOS_14_3: u32 = 140300;
pub const __TVOS_14_5: u32 = 140500;
pub const __TVOS_14_6: u32 = 140600;
pub const __TVOS_14_7: u32 = 140700;
pub const __TVOS_15_0: u32 = 150000;
pub const __TVOS_15_1: u32 = 150100;
pub const __TVOS_15_2: u32 = 150200;
pub const __TVOS_15_3: u32 = 150300;
pub const __TVOS_15_4: u32 = 150400;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __WATCHOS_5_0: u32 = 50000;
pub const __WATCHOS_5_1: u32 = 50100;
pub const __WATCHOS_5_2: u32 = 50200;
pub const __WATCHOS_5_3: u32 = 50300;
pub const __WATCHOS_6_0: u32 = 60000;
pub const __WATCHOS_6_1: u32 = 60100;
pub const __WATCHOS_6_2: u32 = 60200;
pub const __WATCHOS_7_0: u32 = 70000;
pub const __WATCHOS_7_1: u32 = 70100;
pub const __WATCHOS_7_2: u32 = 70200;
pub const __WATCHOS_7_3: u32 = 70300;
pub const __WATCHOS_7_4: u32 = 70400;
pub const __WATCHOS_7_5: u32 = 70500;
pub const __WATCHOS_7_6: u32 = 70600;
pub const __WATCHOS_8_0: u32 = 80000;
pub const __WATCHOS_8_1: u32 = 80100;
pub const __WATCHOS_8_3: u32 = 80300;
pub const __WATCHOS_8_4: u32 = 80400;
pub const __WATCHOS_8_5: u32 = 80500;
pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
pub const MAC_OS_VERSION_11_0: u32 = 110000;
pub const MAC_OS_VERSION_12_0: u32 = 120000;
pub const __DRIVERKIT_19_0: u32 = 190000;
pub const __DRIVERKIT_20_0: u32 = 200000;
pub const __DRIVERKIT_21_0: u32 = 210000;
pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 120300;
pub const __ENABLE_LEGACY_MAC_AVAILABILITY: u32 = 1;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _ARM_SIGNAL_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_V5: u32 = 5;
pub const RUSAGE_INFO_CURRENT: u32 = 5;
pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        128usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    fn test_field___mbstate8() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__mbstate8) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t),
                "::",
                stringify!(__mbstate8)
            )
        );
    }
    test_field___mbstate8();
    fn test_field__mbstateL() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._mbstateL) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t),
                "::",
                stringify!(_mbstateL)
            )
        );
    }
    test_field__mbstateL();
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[test]
fn bindgen_test_layout___darwin_pthread_handler_rec() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_pthread_handler_rec>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_pthread_handler_rec>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_pthread_handler_rec))
    );
    fn test_field___routine() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_pthread_handler_rec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__routine) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_pthread_handler_rec),
                "::",
                stringify!(__routine)
            )
        );
    }
    test_field___routine();
    fn test_field___arg() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_pthread_handler_rec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__arg) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_pthread_handler_rec),
                "::",
                stringify!(__arg)
            )
        );
    }
    test_field___arg();
    fn test_field___next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_pthread_handler_rec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_pthread_handler_rec),
                "::",
                stringify!(__next)
            )
        );
    }
    test_field___next();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_attr_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_attr_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_attr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_attr_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_attr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_attr_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_cond_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_cond_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_cond_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_cond_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_cond_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_condattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_condattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_condattr_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_condattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_condattr_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_condattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_condattr_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_mutex_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_mutex_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_mutex_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_mutex_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutexattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutexattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutexattr_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_mutexattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_mutexattr_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_mutexattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_mutexattr_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_once_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_once_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_once_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_once_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_once_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_once_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_once_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_once_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlock_t>(),
        200usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlock_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_rwlock_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_rwlock_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_rwlock_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_rwlock_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlockattr_t>(),
        24usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_rwlockattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_rwlockattr_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_rwlockattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_rwlockattr_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_t>(),
        8192usize,
        concat!("Size of: ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_t))
    );
    fn test_field___sig() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_t),
                "::",
                stringify!(__sig)
            )
        );
    }
    test_field___sig();
    fn test_field___cleanup_stack() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__cleanup_stack) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_t),
                "::",
                stringify!(__cleanup_stack)
            )
        );
    }
    test_field___cleanup_stack();
    fn test_field___opaque() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_opaque_pthread_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(_opaque_pthread_t),
                "::",
                stringify!(__opaque)
            )
        );
    }
    test_field___opaque();
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state {
    pub __exception: __uint32_t,
    pub __fsr: __uint32_t,
    pub __far: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_exception_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_exception_state>(),
        12usize,
        concat!("Size of: ", stringify!(__darwin_arm_exception_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_exception_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_exception_state))
    );
    fn test_field___exception() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__exception) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state),
                "::",
                stringify!(__exception)
            )
        );
    }
    test_field___exception();
    fn test_field___fsr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fsr) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state),
                "::",
                stringify!(__fsr)
            )
        );
    }
    test_field___fsr();
    fn test_field___far() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__far) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state),
                "::",
                stringify!(__far)
            )
        );
    }
    test_field___far();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64 {
    pub __far: __uint64_t,
    pub __esr: __uint32_t,
    pub __exception: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_exception_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_exception_state64>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_arm_exception_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_exception_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_exception_state64))
    );
    fn test_field___far() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__far) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state64),
                "::",
                stringify!(__far)
            )
        );
    }
    test_field___far();
    fn test_field___esr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__esr) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state64),
                "::",
                stringify!(__esr)
            )
        );
    }
    test_field___esr();
    fn test_field___exception() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_exception_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__exception) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_exception_state64),
                "::",
                stringify!(__exception)
            )
        );
    }
    test_field___exception();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [__uint32_t; 13usize],
    pub __sp: __uint32_t,
    pub __lr: __uint32_t,
    pub __pc: __uint32_t,
    pub __cpsr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_thread_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_thread_state>(),
        68usize,
        concat!("Size of: ", stringify!(__darwin_arm_thread_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_thread_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_thread_state))
    );
    fn test_field___r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__r) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state),
                "::",
                stringify!(__r)
            )
        );
    }
    test_field___r();
    fn test_field___sp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sp) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state),
                "::",
                stringify!(__sp)
            )
        );
    }
    test_field___sp();
    fn test_field___lr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__lr) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state),
                "::",
                stringify!(__lr)
            )
        );
    }
    test_field___lr();
    fn test_field___pc() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pc) as usize - ptr as usize
            },
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state),
                "::",
                stringify!(__pc)
            )
        );
    }
    test_field___pc();
    fn test_field___cpsr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__cpsr) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state),
                "::",
                stringify!(__cpsr)
            )
        );
    }
    test_field___cpsr();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [__uint64_t; 29usize],
    pub __fp: __uint64_t,
    pub __lr: __uint64_t,
    pub __sp: __uint64_t,
    pub __pc: __uint64_t,
    pub __cpsr: __uint32_t,
    pub __pad: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_thread_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_thread_state64>(),
        272usize,
        concat!("Size of: ", stringify!(__darwin_arm_thread_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_thread_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_thread_state64))
    );
    fn test_field___x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__x)
            )
        );
    }
    test_field___x();
    fn test_field___fp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fp) as usize - ptr as usize
            },
            232usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__fp)
            )
        );
    }
    test_field___fp();
    fn test_field___lr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__lr) as usize - ptr as usize
            },
            240usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__lr)
            )
        );
    }
    test_field___lr();
    fn test_field___sp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sp) as usize - ptr as usize
            },
            248usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__sp)
            )
        );
    }
    test_field___sp();
    fn test_field___pc() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pc) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__pc)
            )
        );
    }
    test_field___pc();
    fn test_field___cpsr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__cpsr) as usize - ptr as usize
            },
            264usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__cpsr)
            )
        );
    }
    test_field___cpsr();
    fn test_field___pad() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_thread_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad) as usize - ptr as usize
            },
            268usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_thread_state64),
                "::",
                stringify!(__pad)
            )
        );
    }
    test_field___pad();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_vfp_state {
    pub __r: [__uint32_t; 64usize],
    pub __fpscr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_vfp_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_vfp_state>(),
        260usize,
        concat!("Size of: ", stringify!(__darwin_arm_vfp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_vfp_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_vfp_state))
    );
    fn test_field___r() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_vfp_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__r) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_vfp_state),
                "::",
                stringify!(__r)
            )
        );
    }
    test_field___r();
    fn test_field___fpscr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_vfp_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fpscr) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_vfp_state),
                "::",
                stringify!(__fpscr)
            )
        );
    }
    test_field___fpscr();
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state64 {
    pub __v: [__uint128_t; 32usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_neon_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_neon_state64>(),
        528usize,
        concat!("Size of: ", stringify!(__darwin_arm_neon_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_neon_state64>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_arm_neon_state64))
    );
    fn test_field___v() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__v) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state64),
                "::",
                stringify!(__v)
            )
        );
    }
    test_field___v();
    fn test_field___fpsr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fpsr) as usize - ptr as usize
            },
            512usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state64),
                "::",
                stringify!(__fpsr)
            )
        );
    }
    test_field___fpsr();
    fn test_field___fpcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fpcr) as usize - ptr as usize
            },
            516usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state64),
                "::",
                stringify!(__fpcr)
            )
        );
    }
    test_field___fpcr();
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state {
    pub __v: [__uint128_t; 16usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_neon_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_neon_state>(),
        272usize,
        concat!("Size of: ", stringify!(__darwin_arm_neon_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_neon_state>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_arm_neon_state))
    );
    fn test_field___v() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__v) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state),
                "::",
                stringify!(__v)
            )
        );
    }
    test_field___v();
    fn test_field___fpsr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fpsr) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state),
                "::",
                stringify!(__fpsr)
            )
        );
    }
    test_field___fpsr();
    fn test_field___fpcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_neon_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fpcr) as usize - ptr as usize
            },
            260usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_neon_state),
                "::",
                stringify!(__fpcr)
            )
        );
    }
    test_field___fpcr();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_pagein_state {
    pub __pagein_error: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___arm_pagein_state() {
    assert_eq!(
        ::std::mem::size_of::<__arm_pagein_state>(),
        4usize,
        concat!("Size of: ", stringify!(__arm_pagein_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__arm_pagein_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__arm_pagein_state))
    );
    fn test_field___pagein_error() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__arm_pagein_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pagein_error) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__arm_pagein_state),
                "::",
                stringify!(__pagein_error)
            )
        );
    }
    test_field___pagein_error();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_legacy_debug_state {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
}
#[test]
fn bindgen_test_layout___arm_legacy_debug_state() {
    assert_eq!(
        ::std::mem::size_of::<__arm_legacy_debug_state>(),
        256usize,
        concat!("Size of: ", stringify!(__arm_legacy_debug_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__arm_legacy_debug_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__arm_legacy_debug_state))
    );
    fn test_field___bvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__arm_legacy_debug_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__arm_legacy_debug_state),
                "::",
                stringify!(__bvr)
            )
        );
    }
    test_field___bvr();
    fn test_field___bcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__arm_legacy_debug_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(__arm_legacy_debug_state),
                "::",
                stringify!(__bcr)
            )
        );
    }
    test_field___bcr();
    fn test_field___wvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__arm_legacy_debug_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(__arm_legacy_debug_state),
                "::",
                stringify!(__wvr)
            )
        );
    }
    test_field___wvr();
    fn test_field___wcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__arm_legacy_debug_state>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(__arm_legacy_debug_state),
                "::",
                stringify!(__wcr)
            )
        );
    }
    test_field___wcr();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state32 {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_debug_state32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_debug_state32>(),
        264usize,
        concat!("Size of: ", stringify!(__darwin_arm_debug_state32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_debug_state32>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_debug_state32))
    );
    fn test_field___bvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state32),
                "::",
                stringify!(__bvr)
            )
        );
    }
    test_field___bvr();
    fn test_field___bcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state32),
                "::",
                stringify!(__bcr)
            )
        );
    }
    test_field___bcr();
    fn test_field___wvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state32),
                "::",
                stringify!(__wvr)
            )
        );
    }
    test_field___wvr();
    fn test_field___wcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state32),
                "::",
                stringify!(__wcr)
            )
        );
    }
    test_field___wcr();
    fn test_field___mdscr_el1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__mdscr_el1) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state32),
                "::",
                stringify!(__mdscr_el1)
            )
        );
    }
    test_field___mdscr_el1();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state64 {
    pub __bvr: [__uint64_t; 16usize],
    pub __bcr: [__uint64_t; 16usize],
    pub __wvr: [__uint64_t; 16usize],
    pub __wcr: [__uint64_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_debug_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_debug_state64>(),
        520usize,
        concat!("Size of: ", stringify!(__darwin_arm_debug_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_debug_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_debug_state64))
    );
    fn test_field___bvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state64),
                "::",
                stringify!(__bvr)
            )
        );
    }
    test_field___bvr();
    fn test_field___bcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state64),
                "::",
                stringify!(__bcr)
            )
        );
    }
    test_field___bcr();
    fn test_field___wvr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state64),
                "::",
                stringify!(__wvr)
            )
        );
    }
    test_field___wvr();
    fn test_field___wcr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize
            },
            384usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state64),
                "::",
                stringify!(__wcr)
            )
        );
    }
    test_field___wcr();
    fn test_field___mdscr_el1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_debug_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__mdscr_el1) as usize - ptr as usize
            },
            512usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_debug_state64),
                "::",
                stringify!(__mdscr_el1)
            )
        );
    }
    test_field___mdscr_el1();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[test]
fn bindgen_test_layout___darwin_arm_cpmu_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_cpmu_state64>(),
        128usize,
        concat!("Size of: ", stringify!(__darwin_arm_cpmu_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_cpmu_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_cpmu_state64))
    );
    fn test_field___ctrs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_arm_cpmu_state64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__ctrs) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_arm_cpmu_state64),
                "::",
                stringify!(__ctrs)
            )
        );
    }
    test_field___ctrs();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_arm_exception_state,
    pub __ss: __darwin_arm_thread_state,
    pub __fs: __darwin_arm_vfp_state,
}
#[test]
fn bindgen_test_layout___darwin_mcontext32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext32>(),
        340usize,
        concat!("Size of: ", stringify!(__darwin_mcontext32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext32))
    );
    fn test_field___es() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__es) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext32),
                "::",
                stringify!(__es)
            )
        );
    }
    test_field___es();
    fn test_field___ss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__ss) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext32),
                "::",
                stringify!(__ss)
            )
        );
    }
    test_field___ss();
    fn test_field___fs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fs) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext32),
                "::",
                stringify!(__fs)
            )
        );
    }
    test_field___fs();
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_arm_exception_state64,
    pub __ss: __darwin_arm_thread_state64,
    pub __ns: __darwin_arm_neon_state64,
}
#[test]
fn bindgen_test_layout___darwin_mcontext64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext64>(),
        816usize,
        concat!("Size of: ", stringify!(__darwin_mcontext64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext64>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext64))
    );
    fn test_field___es() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__es) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext64),
                "::",
                stringify!(__es)
            )
        );
    }
    test_field___es();
    fn test_field___ss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__ss) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext64),
                "::",
                stringify!(__ss)
            )
        );
    }
    test_field___ss();
    fn test_field___ns() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_mcontext64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__ns) as usize - ptr as usize
            },
            288usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_mcontext64),
                "::",
                stringify!(__ns)
            )
        );
    }
    test_field___ns();
}
pub type mcontext_t = *mut __darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___darwin_sigaltstack() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_sigaltstack>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_sigaltstack))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_sigaltstack>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_sigaltstack))
    );
    fn test_field_ss_sp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_sigaltstack>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ss_sp) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_sigaltstack),
                "::",
                stringify!(ss_sp)
            )
        );
    }
    test_field_ss_sp();
    fn test_field_ss_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_sigaltstack>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ss_size) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_sigaltstack),
                "::",
                stringify!(ss_size)
            )
        );
    }
    test_field_ss_size();
    fn test_field_ss_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_sigaltstack>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ss_flags) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_sigaltstack),
                "::",
                stringify!(ss_flags)
            )
        );
    }
    test_field_ss_flags();
}
pub type stack_t = __darwin_sigaltstack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::std::os::raw::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext64,
}
#[test]
fn bindgen_test_layout___darwin_ucontext() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_ucontext>(),
        56usize,
        concat!("Size of: ", stringify!(__darwin_ucontext))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_ucontext>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_ucontext))
    );
    fn test_field_uc_onstack() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_onstack) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_onstack)
            )
        );
    }
    test_field_uc_onstack();
    fn test_field_uc_sigmask() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_sigmask) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_sigmask)
            )
        );
    }
    test_field_uc_sigmask();
    fn test_field_uc_stack() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_stack) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_stack)
            )
        );
    }
    test_field_uc_stack();
    fn test_field_uc_link() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_link) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_link)
            )
        );
    }
    test_field_uc_link();
    fn test_field_uc_mcsize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_mcsize) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_mcsize)
            )
        );
    }
    test_field_uc_mcsize();
    fn test_field_uc_mcontext() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__darwin_ucontext>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).uc_mcontext) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(__darwin_ucontext),
                "::",
                stringify!(uc_mcontext)
            )
        );
    }
    test_field_uc_mcontext();
}
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_sigval() {
    assert_eq!(
        ::std::mem::size_of::<sigval>(),
        8usize,
        concat!("Size of: ", stringify!(sigval))
    );
    assert_eq!(
        ::std::mem::align_of::<sigval>(),
        8usize,
        concat!("Alignment of ", stringify!(sigval))
    );
    fn test_field_sival_int() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sival_int) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigval),
                "::",
                stringify!(sival_int)
            )
        );
    }
    test_field_sival_int();
    fn test_field_sival_ptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sival_ptr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigval),
                "::",
                stringify!(sival_ptr)
            )
        );
    }
    test_field_sival_ptr();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::std::os::raw::c_int,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
#[test]
fn bindgen_test_layout_sigevent() {
    assert_eq!(
        ::std::mem::size_of::<sigevent>(),
        32usize,
        concat!("Size of: ", stringify!(sigevent))
    );
    assert_eq!(
        ::std::mem::align_of::<sigevent>(),
        8usize,
        concat!("Alignment of ", stringify!(sigevent))
    );
    fn test_field_sigev_notify() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigevent>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sigev_notify) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigevent),
                "::",
                stringify!(sigev_notify)
            )
        );
    }
    test_field_sigev_notify();
    fn test_field_sigev_signo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigevent>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sigev_signo) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(sigevent),
                "::",
                stringify!(sigev_signo)
            )
        );
    }
    test_field_sigev_signo();
    fn test_field_sigev_value() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigevent>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sigev_value) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(sigevent),
                "::",
                stringify!(sigev_value)
            )
        );
    }
    test_field_sigev_value();
    fn test_field_sigev_notify_function() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigevent>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sigev_notify_function) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(sigevent),
                "::",
                stringify!(sigev_notify_function)
            )
        );
    }
    test_field_sigev_notify_function();
    fn test_field_sigev_notify_attributes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigevent>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sigev_notify_attributes) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(sigevent),
                "::",
                stringify!(sigev_notify_attributes)
            )
        );
    }
    test_field_sigev_notify_attributes();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_value: sigval,
    pub si_band: ::std::os::raw::c_long,
    pub __pad: [::std::os::raw::c_ulong; 7usize],
}
#[test]
fn bindgen_test_layout___siginfo() {
    assert_eq!(
        ::std::mem::size_of::<__siginfo>(),
        104usize,
        concat!("Size of: ", stringify!(__siginfo))
    );
    assert_eq!(
        ::std::mem::align_of::<__siginfo>(),
        8usize,
        concat!("Alignment of ", stringify!(__siginfo))
    );
    fn test_field_si_signo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_signo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_signo)
            )
        );
    }
    test_field_si_signo();
    fn test_field_si_errno() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_errno) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_errno)
            )
        );
    }
    test_field_si_errno();
    fn test_field_si_code() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_code) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_code)
            )
        );
    }
    test_field_si_code();
    fn test_field_si_pid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_pid) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_pid)
            )
        );
    }
    test_field_si_pid();
    fn test_field_si_uid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_uid) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_uid)
            )
        );
    }
    test_field_si_uid();
    fn test_field_si_status() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_status) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_status)
            )
        );
    }
    test_field_si_status();
    fn test_field_si_addr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_addr) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_addr)
            )
        );
    }
    test_field_si_addr();
    fn test_field_si_value() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_value) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_value)
            )
        );
    }
    test_field_si_value();
    fn test_field_si_band() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).si_band) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(si_band)
            )
        );
    }
    test_field_si_band();
    fn test_field___pad() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__siginfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(__siginfo),
                "::",
                stringify!(__pad)
            )
        );
    }
    test_field___pad();
}
pub type siginfo_t = __siginfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub __sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[test]
fn bindgen_test_layout___sigaction_u() {
    assert_eq!(
        ::std::mem::size_of::<__sigaction_u>(),
        8usize,
        concat!("Size of: ", stringify!(__sigaction_u))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction_u>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction_u))
    );
    fn test_field___sa_handler() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction_u>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sa_handler) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction_u),
                "::",
                stringify!(__sa_handler)
            )
        );
    }
    test_field___sa_handler();
    fn test_field___sa_sigaction() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction_u>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sa_sigaction) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction_u),
                "::",
                stringify!(__sa_sigaction)
            )
        );
    }
    test_field___sa_sigaction();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sigaction() {
    assert_eq!(
        ::std::mem::size_of::<__sigaction>(),
        24usize,
        concat!("Size of: ", stringify!(__sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction))
    );
    fn test_field___sigaction_u() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sigaction_u) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction),
                "::",
                stringify!(__sigaction_u)
            )
        );
    }
    test_field___sigaction_u();
    fn test_field_sa_tramp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sa_tramp) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction),
                "::",
                stringify!(sa_tramp)
            )
        );
    }
    test_field_sa_tramp();
    fn test_field_sa_mask() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sa_mask) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction),
                "::",
                stringify!(sa_mask)
            )
        );
    }
    test_field_sa_mask();
    fn test_field_sa_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sa_flags) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigaction),
                "::",
                stringify!(sa_flags)
            )
        );
    }
    test_field_sa_flags();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigaction() {
    assert_eq!(
        ::std::mem::size_of::<sigaction>(),
        16usize,
        concat!("Size of: ", stringify!(sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(sigaction))
    );
    fn test_field___sigaction_u() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__sigaction_u) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigaction),
                "::",
                stringify!(__sigaction_u)
            )
        );
    }
    test_field___sigaction_u();
    fn test_field_sa_mask() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sa_mask) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(sigaction),
                "::",
                stringify!(sa_mask)
            )
        );
    }
    test_field_sa_mask();
    fn test_field_sa_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigaction>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sa_flags) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(sigaction),
                "::",
                stringify!(sa_flags)
            )
        );
    }
    test_field_sa_flags();
}
pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub sv_mask: ::std::os::raw::c_int,
    pub sv_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigvec() {
    assert_eq!(
        ::std::mem::size_of::<sigvec>(),
        16usize,
        concat!("Size of: ", stringify!(sigvec))
    );
    assert_eq!(
        ::std::mem::align_of::<sigvec>(),
        8usize,
        concat!("Alignment of ", stringify!(sigvec))
    );
    fn test_field_sv_handler() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigvec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sv_handler) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigvec),
                "::",
                stringify!(sv_handler)
            )
        );
    }
    test_field_sv_handler();
    fn test_field_sv_mask() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigvec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sv_mask) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(sigvec),
                "::",
                stringify!(sv_mask)
            )
        );
    }
    test_field_sv_mask();
    fn test_field_sv_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigvec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sv_flags) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(sigvec),
                "::",
                stringify!(sv_flags)
            )
        );
    }
    test_field_sv_flags();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_char,
    pub ss_onstack: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigstack() {
    assert_eq!(
        ::std::mem::size_of::<sigstack>(),
        16usize,
        concat!("Size of: ", stringify!(sigstack))
    );
    assert_eq!(
        ::std::mem::align_of::<sigstack>(),
        8usize,
        concat!("Alignment of ", stringify!(sigstack))
    );
    fn test_field_ss_sp() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigstack>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ss_sp) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(sigstack),
                "::",
                stringify!(ss_sp)
            )
        );
    }
    test_field_ss_sp();
    fn test_field_ss_onstack() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<sigstack>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ss_onstack) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(sigstack),
                "::",
                stringify!(ss_onstack)
            )
        );
    }
    test_field_ss_onstack();
}
extern "C" {
    pub fn signal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        ),
    >;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    fn test_field_tv_sec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timeval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(timeval),
                "::",
                stringify!(tv_sec)
            )
        );
    }
    test_field_tv_sec();
    fn test_field_tv_usec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timeval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_usec) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(timeval),
                "::",
                stringify!(tv_usec)
            )
        );
    }
    test_field_tv_usec();
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::std::os::raw::c_long,
    pub ru_ixrss: ::std::os::raw::c_long,
    pub ru_idrss: ::std::os::raw::c_long,
    pub ru_isrss: ::std::os::raw::c_long,
    pub ru_minflt: ::std::os::raw::c_long,
    pub ru_majflt: ::std::os::raw::c_long,
    pub ru_nswap: ::std::os::raw::c_long,
    pub ru_inblock: ::std::os::raw::c_long,
    pub ru_oublock: ::std::os::raw::c_long,
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub ru_nsignals: ::std::os::raw::c_long,
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub ru_nivcsw: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_rusage() {
    assert_eq!(
        ::std::mem::size_of::<rusage>(),
        144usize,
        concat!("Size of: ", stringify!(rusage))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage))
    );
    fn test_field_ru_utime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_utime) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_utime)
            )
        );
    }
    test_field_ru_utime();
    fn test_field_ru_stime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_stime) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_stime)
            )
        );
    }
    test_field_ru_stime();
    fn test_field_ru_maxrss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_maxrss) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_maxrss)
            )
        );
    }
    test_field_ru_maxrss();
    fn test_field_ru_ixrss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_ixrss) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_ixrss)
            )
        );
    }
    test_field_ru_ixrss();
    fn test_field_ru_idrss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_idrss) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_idrss)
            )
        );
    }
    test_field_ru_idrss();
    fn test_field_ru_isrss() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_isrss) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_isrss)
            )
        );
    }
    test_field_ru_isrss();
    fn test_field_ru_minflt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_minflt) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_minflt)
            )
        );
    }
    test_field_ru_minflt();
    fn test_field_ru_majflt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_majflt) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_majflt)
            )
        );
    }
    test_field_ru_majflt();
    fn test_field_ru_nswap() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_nswap) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_nswap)
            )
        );
    }
    test_field_ru_nswap();
    fn test_field_ru_inblock() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_inblock) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_inblock)
            )
        );
    }
    test_field_ru_inblock();
    fn test_field_ru_oublock() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_oublock) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_oublock)
            )
        );
    }
    test_field_ru_oublock();
    fn test_field_ru_msgsnd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_msgsnd) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_msgsnd)
            )
        );
    }
    test_field_ru_msgsnd();
    fn test_field_ru_msgrcv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_msgrcv) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_msgrcv)
            )
        );
    }
    test_field_ru_msgrcv();
    fn test_field_ru_nsignals() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_nsignals) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_nsignals)
            )
        );
    }
    test_field_ru_nsignals();
    fn test_field_ru_nvcsw() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_nvcsw) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_nvcsw)
            )
        );
    }
    test_field_ru_nvcsw();
    fn test_field_ru_nivcsw() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ru_nivcsw) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage),
                "::",
                stringify!(ru_nivcsw)
            )
        );
    }
    test_field_ru_nivcsw();
}
pub type rusage_info_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v0() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v0>(),
        96usize,
        concat!("Size of: ", stringify!(rusage_info_v0))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v0>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v0))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v0>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v0),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v1() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v1>(),
        144usize,
        concat!("Size of: ", stringify!(rusage_info_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v1>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v1))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
    fn test_field_ri_child_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_user_time)
            )
        );
    }
    test_field_ri_child_user_time();
    fn test_field_ri_child_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_system_time)
            )
        );
    }
    test_field_ri_child_system_time();
    fn test_field_ri_child_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_child_pkg_idle_wkups();
    fn test_field_ri_child_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_interrupt_wkups)
            )
        );
    }
    test_field_ri_child_interrupt_wkups();
    fn test_field_ri_child_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_pageins)
            )
        );
    }
    test_field_ri_child_pageins();
    fn test_field_ri_child_elapsed_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v1),
                "::",
                stringify!(ri_child_elapsed_abstime)
            )
        );
    }
    test_field_ri_child_elapsed_abstime();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v2() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v2>(),
        160usize,
        concat!("Size of: ", stringify!(rusage_info_v2))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v2>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v2))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
    fn test_field_ri_child_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_user_time)
            )
        );
    }
    test_field_ri_child_user_time();
    fn test_field_ri_child_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_system_time)
            )
        );
    }
    test_field_ri_child_system_time();
    fn test_field_ri_child_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_child_pkg_idle_wkups();
    fn test_field_ri_child_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_interrupt_wkups)
            )
        );
    }
    test_field_ri_child_interrupt_wkups();
    fn test_field_ri_child_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_pageins)
            )
        );
    }
    test_field_ri_child_pageins();
    fn test_field_ri_child_elapsed_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_child_elapsed_abstime)
            )
        );
    }
    test_field_ri_child_elapsed_abstime();
    fn test_field_ri_diskio_bytesread() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize
            },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_diskio_bytesread)
            )
        );
    }
    test_field_ri_diskio_bytesread();
    fn test_field_ri_diskio_byteswritten() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v2),
                "::",
                stringify!(ri_diskio_byteswritten)
            )
        );
    }
    test_field_ri_diskio_byteswritten();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v3() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v3>(),
        232usize,
        concat!("Size of: ", stringify!(rusage_info_v3))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v3>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v3))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
    fn test_field_ri_child_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_user_time)
            )
        );
    }
    test_field_ri_child_user_time();
    fn test_field_ri_child_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_system_time)
            )
        );
    }
    test_field_ri_child_system_time();
    fn test_field_ri_child_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_child_pkg_idle_wkups();
    fn test_field_ri_child_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_interrupt_wkups)
            )
        );
    }
    test_field_ri_child_interrupt_wkups();
    fn test_field_ri_child_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_pageins)
            )
        );
    }
    test_field_ri_child_pageins();
    fn test_field_ri_child_elapsed_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_child_elapsed_abstime)
            )
        );
    }
    test_field_ri_child_elapsed_abstime();
    fn test_field_ri_diskio_bytesread() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize
            },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_diskio_bytesread)
            )
        );
    }
    test_field_ri_diskio_bytesread();
    fn test_field_ri_diskio_byteswritten() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_diskio_byteswritten)
            )
        );
    }
    test_field_ri_diskio_byteswritten();
    fn test_field_ri_cpu_time_qos_default() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize
            },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_default)
            )
        );
    }
    test_field_ri_cpu_time_qos_default();
    fn test_field_ri_cpu_time_qos_maintenance() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize
            },
            168usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_maintenance)
            )
        );
    }
    test_field_ri_cpu_time_qos_maintenance();
    fn test_field_ri_cpu_time_qos_background() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize
            },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_background)
            )
        );
    }
    test_field_ri_cpu_time_qos_background();
    fn test_field_ri_cpu_time_qos_utility() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize
            },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_utility)
            )
        );
    }
    test_field_ri_cpu_time_qos_utility();
    fn test_field_ri_cpu_time_qos_legacy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_legacy)
            )
        );
    }
    test_field_ri_cpu_time_qos_legacy();
    fn test_field_ri_cpu_time_qos_user_initiated() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
            },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_user_initiated)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_initiated();
    fn test_field_ri_cpu_time_qos_user_interactive() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize
                    - ptr as usize
            },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_cpu_time_qos_user_interactive)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_interactive();
    fn test_field_ri_billed_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize
            },
            216usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_billed_system_time)
            )
        );
    }
    test_field_ri_billed_system_time();
    fn test_field_ri_serviced_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize
            },
            224usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v3),
                "::",
                stringify!(ri_serviced_system_time)
            )
        );
    }
    test_field_ri_serviced_system_time();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v4() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v4>(),
        296usize,
        concat!("Size of: ", stringify!(rusage_info_v4))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v4>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v4))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
    fn test_field_ri_child_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_user_time)
            )
        );
    }
    test_field_ri_child_user_time();
    fn test_field_ri_child_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_system_time)
            )
        );
    }
    test_field_ri_child_system_time();
    fn test_field_ri_child_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_child_pkg_idle_wkups();
    fn test_field_ri_child_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_interrupt_wkups)
            )
        );
    }
    test_field_ri_child_interrupt_wkups();
    fn test_field_ri_child_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_pageins)
            )
        );
    }
    test_field_ri_child_pageins();
    fn test_field_ri_child_elapsed_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_child_elapsed_abstime)
            )
        );
    }
    test_field_ri_child_elapsed_abstime();
    fn test_field_ri_diskio_bytesread() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize
            },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_diskio_bytesread)
            )
        );
    }
    test_field_ri_diskio_bytesread();
    fn test_field_ri_diskio_byteswritten() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_diskio_byteswritten)
            )
        );
    }
    test_field_ri_diskio_byteswritten();
    fn test_field_ri_cpu_time_qos_default() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize
            },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_default)
            )
        );
    }
    test_field_ri_cpu_time_qos_default();
    fn test_field_ri_cpu_time_qos_maintenance() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize
            },
            168usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_maintenance)
            )
        );
    }
    test_field_ri_cpu_time_qos_maintenance();
    fn test_field_ri_cpu_time_qos_background() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize
            },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_background)
            )
        );
    }
    test_field_ri_cpu_time_qos_background();
    fn test_field_ri_cpu_time_qos_utility() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize
            },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_utility)
            )
        );
    }
    test_field_ri_cpu_time_qos_utility();
    fn test_field_ri_cpu_time_qos_legacy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_legacy)
            )
        );
    }
    test_field_ri_cpu_time_qos_legacy();
    fn test_field_ri_cpu_time_qos_user_initiated() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
            },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_user_initiated)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_initiated();
    fn test_field_ri_cpu_time_qos_user_interactive() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize
                    - ptr as usize
            },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cpu_time_qos_user_interactive)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_interactive();
    fn test_field_ri_billed_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize
            },
            216usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_billed_system_time)
            )
        );
    }
    test_field_ri_billed_system_time();
    fn test_field_ri_serviced_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize
            },
            224usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_serviced_system_time)
            )
        );
    }
    test_field_ri_serviced_system_time();
    fn test_field_ri_logical_writes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_logical_writes) as usize - ptr as usize
            },
            232usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_logical_writes)
            )
        );
    }
    test_field_ri_logical_writes();
    fn test_field_ri_lifetime_max_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_lifetime_max_phys_footprint) as usize - ptr as usize
            },
            240usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_lifetime_max_phys_footprint)
            )
        );
    }
    test_field_ri_lifetime_max_phys_footprint();
    fn test_field_ri_instructions() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_instructions) as usize - ptr as usize
            },
            248usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_instructions)
            )
        );
    }
    test_field_ri_instructions();
    fn test_field_ri_cycles() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cycles) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_cycles)
            )
        );
    }
    test_field_ri_cycles();
    fn test_field_ri_billed_energy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_billed_energy) as usize - ptr as usize
            },
            264usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_billed_energy)
            )
        );
    }
    test_field_ri_billed_energy();
    fn test_field_ri_serviced_energy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_serviced_energy) as usize - ptr as usize
            },
            272usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_serviced_energy)
            )
        );
    }
    test_field_ri_serviced_energy();
    fn test_field_ri_interval_max_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interval_max_phys_footprint) as usize - ptr as usize
            },
            280usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_interval_max_phys_footprint)
            )
        );
    }
    test_field_ri_interval_max_phys_footprint();
    fn test_field_ri_runnable_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_runnable_time) as usize - ptr as usize
            },
            288usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v4),
                "::",
                stringify!(ri_runnable_time)
            )
        );
    }
    test_field_ri_runnable_time();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v5 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v5() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v5>(),
        304usize,
        concat!("Size of: ", stringify!(rusage_info_v5))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v5>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v5))
    );
    fn test_field_ri_uuid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_uuid)
            )
        );
    }
    test_field_ri_uuid();
    fn test_field_ri_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_user_time)
            )
        );
    }
    test_field_ri_user_time();
    fn test_field_ri_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_system_time)
            )
        );
    }
    test_field_ri_system_time();
    fn test_field_ri_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_pkg_idle_wkups();
    fn test_field_ri_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_interrupt_wkups)
            )
        );
    }
    test_field_ri_interrupt_wkups();
    fn test_field_ri_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_pageins)
            )
        );
    }
    test_field_ri_pageins();
    fn test_field_ri_wired_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_wired_size)
            )
        );
    }
    test_field_ri_wired_size();
    fn test_field_ri_resident_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_resident_size)
            )
        );
    }
    test_field_ri_resident_size();
    fn test_field_ri_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_phys_footprint)
            )
        );
    }
    test_field_ri_phys_footprint();
    fn test_field_ri_proc_start_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_proc_start_abstime)
            )
        );
    }
    test_field_ri_proc_start_abstime();
    fn test_field_ri_proc_exit_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_proc_exit_abstime)
            )
        );
    }
    test_field_ri_proc_exit_abstime();
    fn test_field_ri_child_user_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_user_time)
            )
        );
    }
    test_field_ri_child_user_time();
    fn test_field_ri_child_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_system_time)
            )
        );
    }
    test_field_ri_child_system_time();
    fn test_field_ri_child_pkg_idle_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_pkg_idle_wkups)
            )
        );
    }
    test_field_ri_child_pkg_idle_wkups();
    fn test_field_ri_child_interrupt_wkups() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_interrupt_wkups)
            )
        );
    }
    test_field_ri_child_interrupt_wkups();
    fn test_field_ri_child_pageins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_pageins)
            )
        );
    }
    test_field_ri_child_pageins();
    fn test_field_ri_child_elapsed_abstime() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_child_elapsed_abstime)
            )
        );
    }
    test_field_ri_child_elapsed_abstime();
    fn test_field_ri_diskio_bytesread() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize
            },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_diskio_bytesread)
            )
        );
    }
    test_field_ri_diskio_bytesread();
    fn test_field_ri_diskio_byteswritten() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_diskio_byteswritten)
            )
        );
    }
    test_field_ri_diskio_byteswritten();
    fn test_field_ri_cpu_time_qos_default() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize
            },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_default)
            )
        );
    }
    test_field_ri_cpu_time_qos_default();
    fn test_field_ri_cpu_time_qos_maintenance() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize
            },
            168usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_maintenance)
            )
        );
    }
    test_field_ri_cpu_time_qos_maintenance();
    fn test_field_ri_cpu_time_qos_background() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize
            },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_background)
            )
        );
    }
    test_field_ri_cpu_time_qos_background();
    fn test_field_ri_cpu_time_qos_utility() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize
            },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_utility)
            )
        );
    }
    test_field_ri_cpu_time_qos_utility();
    fn test_field_ri_cpu_time_qos_legacy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_legacy)
            )
        );
    }
    test_field_ri_cpu_time_qos_legacy();
    fn test_field_ri_cpu_time_qos_user_initiated() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
            },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_user_initiated)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_initiated();
    fn test_field_ri_cpu_time_qos_user_interactive() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize
                    - ptr as usize
            },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cpu_time_qos_user_interactive)
            )
        );
    }
    test_field_ri_cpu_time_qos_user_interactive();
    fn test_field_ri_billed_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize
            },
            216usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_billed_system_time)
            )
        );
    }
    test_field_ri_billed_system_time();
    fn test_field_ri_serviced_system_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize
            },
            224usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_serviced_system_time)
            )
        );
    }
    test_field_ri_serviced_system_time();
    fn test_field_ri_logical_writes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_logical_writes) as usize - ptr as usize
            },
            232usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_logical_writes)
            )
        );
    }
    test_field_ri_logical_writes();
    fn test_field_ri_lifetime_max_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_lifetime_max_phys_footprint) as usize - ptr as usize
            },
            240usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_lifetime_max_phys_footprint)
            )
        );
    }
    test_field_ri_lifetime_max_phys_footprint();
    fn test_field_ri_instructions() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_instructions) as usize - ptr as usize
            },
            248usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_instructions)
            )
        );
    }
    test_field_ri_instructions();
    fn test_field_ri_cycles() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_cycles) as usize - ptr as usize
            },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_cycles)
            )
        );
    }
    test_field_ri_cycles();
    fn test_field_ri_billed_energy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_billed_energy) as usize - ptr as usize
            },
            264usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_billed_energy)
            )
        );
    }
    test_field_ri_billed_energy();
    fn test_field_ri_serviced_energy() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_serviced_energy) as usize - ptr as usize
            },
            272usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_serviced_energy)
            )
        );
    }
    test_field_ri_serviced_energy();
    fn test_field_ri_interval_max_phys_footprint() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_interval_max_phys_footprint) as usize - ptr as usize
            },
            280usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_interval_max_phys_footprint)
            )
        );
    }
    test_field_ri_interval_max_phys_footprint();
    fn test_field_ri_runnable_time() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_runnable_time) as usize - ptr as usize
            },
            288usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_runnable_time)
            )
        );
    }
    test_field_ri_runnable_time();
    fn test_field_ri_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rusage_info_v5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ri_flags) as usize - ptr as usize
            },
            296usize,
            concat!(
                "Offset of field: ",
                stringify!(rusage_info_v5),
                "::",
                stringify!(ri_flags)
            )
        );
    }
    test_field_ri_flags();
}
pub type rusage_info_current = rusage_info_v5;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[test]
fn bindgen_test_layout_rlimit() {
    assert_eq!(
        ::std::mem::size_of::<rlimit>(),
        16usize,
        concat!("Size of: ", stringify!(rlimit))
    );
    assert_eq!(
        ::std::mem::align_of::<rlimit>(),
        8usize,
        concat!("Alignment of ", stringify!(rlimit))
    );
    fn test_field_rlim_cur() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rlimit>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rlim_cur) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rlimit),
                "::",
                stringify!(rlim_cur)
            )
        );
    }
    test_field_rlim_cur();
    fn test_field_rlim_max() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rlimit>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rlim_max) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rlimit),
                "::",
                stringify!(rlim_max)
            )
        );
    }
    test_field_rlim_max();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
#[test]
fn bindgen_test_layout_proc_rlimit_control_wakeupmon() {
    assert_eq!(
        ::std::mem::size_of::<proc_rlimit_control_wakeupmon>(),
        8usize,
        concat!("Size of: ", stringify!(proc_rlimit_control_wakeupmon))
    );
    assert_eq!(
        ::std::mem::align_of::<proc_rlimit_control_wakeupmon>(),
        4usize,
        concat!("Alignment of ", stringify!(proc_rlimit_control_wakeupmon))
    );
    fn test_field_wm_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<proc_rlimit_control_wakeupmon>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).wm_flags) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(proc_rlimit_control_wakeupmon),
                "::",
                stringify!(wm_flags)
            )
        );
    }
    test_field_wm_flags();
    fn test_field_wm_rate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<proc_rlimit_control_wakeupmon>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).wm_rate) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(proc_rlimit_control_wakeupmon),
                "::",
                stringify!(wm_rate)
            )
        );
    }
    test_field_wm_rate();
}
extern "C" {
    pub fn getpriority(arg1: ::std::os::raw::c_int, arg2: id_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit(arg1: ::std::os::raw::c_int, arg2: *mut rlimit) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrusage(arg1: ::std::os::raw::c_int, arg2: *mut rusage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpriority(
        arg1: ::std::os::raw::c_int,
        arg2: id_t,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit(arg1: ::std::os::raw::c_int, arg2: *const rlimit) -> ::std::os::raw::c_int;
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU16 {
    pub __val: u16,
}
#[test]
fn bindgen_test_layout__OSUnalignedU16() {
    assert_eq!(
        ::std::mem::size_of::<_OSUnalignedU16>(),
        2usize,
        concat!("Size of: ", stringify!(_OSUnalignedU16))
    );
    assert_eq!(
        ::std::mem::align_of::<_OSUnalignedU16>(),
        1usize,
        concat!("Alignment of ", stringify!(_OSUnalignedU16))
    );
    fn test_field___val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_OSUnalignedU16>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_OSUnalignedU16),
                "::",
                stringify!(__val)
            )
        );
    }
    test_field___val();
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU32 {
    pub __val: u32,
}
#[test]
fn bindgen_test_layout__OSUnalignedU32() {
    assert_eq!(
        ::std::mem::size_of::<_OSUnalignedU32>(),
        4usize,
        concat!("Size of: ", stringify!(_OSUnalignedU32))
    );
    assert_eq!(
        ::std::mem::align_of::<_OSUnalignedU32>(),
        1usize,
        concat!("Alignment of ", stringify!(_OSUnalignedU32))
    );
    fn test_field___val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_OSUnalignedU32>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_OSUnalignedU32),
                "::",
                stringify!(__val)
            )
        );
    }
    test_field___val();
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU64 {
    pub __val: u64,
}
#[test]
fn bindgen_test_layout__OSUnalignedU64() {
    assert_eq!(
        ::std::mem::size_of::<_OSUnalignedU64>(),
        8usize,
        concat!("Size of: ", stringify!(_OSUnalignedU64))
    );
    assert_eq!(
        ::std::mem::align_of::<_OSUnalignedU64>(),
        1usize,
        concat!("Alignment of ", stringify!(_OSUnalignedU64))
    );
    fn test_field___val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_OSUnalignedU64>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_OSUnalignedU64),
                "::",
                stringify!(__val)
            )
        );
    }
    test_field___val();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_1))
    );
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_uint,
        w_Coredump: ::std::os::raw::c_uint,
        w_Retcode: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_2))
    );
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_uint,
        w_Stopsig: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_wait() {
    assert_eq!(
        ::std::mem::size_of::<wait>(),
        4usize,
        concat!("Size of: ", stringify!(wait))
    );
    assert_eq!(
        ::std::mem::align_of::<wait>(),
        4usize,
        concat!("Alignment of ", stringify!(wait))
    );
    fn test_field_w_status() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<wait>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).w_status) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(wait),
                "::",
                stringify!(w_status)
            )
        );
    }
    test_field_w_status();
    fn test_field_w_T() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<wait>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).w_T) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_T))
        );
    }
    test_field_w_T();
    fn test_field_w_S() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<wait>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).w_S) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_S))
        );
    }
    test_field_w_S();
}
extern "C" {
    pub fn wait(arg1: *mut ::std::os::raw::c_int) -> pid_t;
}
extern "C" {
    pub fn waitpid(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> pid_t;
}
extern "C" {
    pub fn waitid(
        arg1: idtype_t,
        arg2: id_t,
        arg3: *mut siginfo_t,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wait3(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn wait4(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn alloca(arg1: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<div_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(div_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<div_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(div_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ldiv_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ldiv_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lldiv_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lldiv_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
extern "C" {
    pub static mut __mb_cur_max: ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __count: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn aligned_alloc(
        __alignment: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn abs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atof(arg1: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn div(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn labs(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ldiv(arg1: ::std::os::raw::c_long, arg2: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn llabs(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lldiv(arg1: ::std::os::raw::c_longlong, arg2: ::std::os::raw::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(arg1: *mut wchar_t, arg2: *const ::std::os::raw::c_char, arg3: usize) -> usize;
}
extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn strtod(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtold(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtoll(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoul(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoull(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstombs(arg1: *mut ::std::os::raw::c_char, arg2: *const wchar_t, arg3: usize) -> usize;
}
extern "C" {
    pub fn wctomb(arg1: *mut ::std::os::raw::c_char, arg2: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _Exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn a64l(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn ecvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn erand48(arg1: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn fcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const *mut ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate(
        arg1: ::std::os::raw::c_uint,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn jrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn l64a(arg1: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lcong48(arg1: *mut ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mktemp(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn posix_openpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        fildes: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putenv(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rand_r(arg1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
    pub fn realpath(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn seed48(arg1: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __overwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setkey(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn setstate(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn srand48(arg1: ::std::os::raw::c_long);
}
extern "C" {
    pub fn srandom(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn unlockpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
extern "C" {
    pub fn arc4random() -> u32;
}
extern "C" {
    pub fn arc4random_addrandom(arg1: *mut ::std::os::raw::c_uchar, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __nbytes: usize);
}
extern "C" {
    pub fn arc4random_stir();
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: u32) -> u32;
}
extern "C" {
    pub fn atexit_b(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bsearch_b(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn cgetcap(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cgetclose() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetent(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetfirst(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetmatch(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnext(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnum(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetset(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetstr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetustr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn daemon(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn devname_r(
        arg1: dev_t,
        arg2: mode_t,
        buf: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getbsize(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_long,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getloadavg(arg1: *mut f64, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprogname() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn setprogname(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn heapsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn heapsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn psort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn psort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn psort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn qsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn radixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rpmatch(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sradixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sranddev();
}
extern "C" {
    pub fn srandomdev();
}
extern "C" {
    pub fn reallocf(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strtonum(
        __numstr: *const ::std::os::raw::c_char,
        __minval: ::std::os::raw::c_longlong,
        __maxval: ::std::os::raw::c_longlong,
        __errstrp: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub static mut suboptarg: *mut ::std::os::raw::c_char;
}
pub type CG1Element = *mut ::std::os::raw::c_void;
pub type CG2Element = *mut ::std::os::raw::c_void;
pub type CPrivateKey = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn CG1ElementSize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CG1ElementFromBytes(
        data: *const ::std::os::raw::c_void,
        didErr: *mut bool,
    ) -> CG1Element;
}
extern "C" {
    pub fn CG1ElementGenerator() -> CG1Element;
}
extern "C" {
    pub fn CG1ElementIsValid(el: CG1Element) -> bool;
}
extern "C" {
    pub fn CG1ElementGetFingerprint(el: CG1Element) -> u32;
}
extern "C" {
    pub fn CG1ElementIsEqual(el1: CG1Element, el2: CG1Element) -> bool;
}
extern "C" {
    pub fn CG1ElementAdd(el1: CG1Element, el2: CG1Element) -> CG1Element;
}
extern "C" {
    pub fn CG1ElementMul(el: CG1Element, sk: CPrivateKey) -> CG1Element;
}
extern "C" {
    pub fn CG1ElementNegate(el: CG1Element) -> CG1Element;
}
extern "C" {
    pub fn CG1ElementSerialize(el: CG1Element) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn CG1ElementFree(el: CG1Element);
}
extern "C" {
    pub fn CG2ElementSize() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CG2ElementFromBytes(
        data: *const ::std::os::raw::c_void,
        didErr: *mut bool,
    ) -> CG2Element;
}
extern "C" {
    pub fn CG2ElementGenerator() -> CG2Element;
}
extern "C" {
    pub fn CG2ElementIsValid(el: CG2Element) -> bool;
}
extern "C" {
    pub fn CG2ElementIsEqual(el1: CG2Element, el2: CG2Element) -> bool;
}
extern "C" {
    pub fn CG2ElementAdd(el1: CG2Element, el2: CG2Element) -> CG2Element;
}
extern "C" {
    pub fn CG2ElementMul(el: CG2Element, sk: CPrivateKey) -> CG2Element;
}
extern "C" {
    pub fn CG2ElementNegate(el: CG2Element) -> CG2Element;
}
extern "C" {
    pub fn CG2ElementSerialize(el: CG2Element) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn CG2ElementFree(el: CG2Element);
}
extern "C" {
    pub fn CPrivateKeyFromBytes(
        data: *const ::std::os::raw::c_void,
        modOrder: bool,
        didErr: *mut bool,
    ) -> CPrivateKey;
}
extern "C" {
    pub fn CPrivateKeyAggregate(sks: *mut *mut ::std::os::raw::c_void, len: usize) -> CPrivateKey;
}
extern "C" {
    pub fn CPrivateKeyGetG1Element(sk: CPrivateKey, didErr: *mut bool) -> CG1Element;
}
extern "C" {
    pub fn CPrivateKeyGetG2Element(sk: CPrivateKey, didErr: *mut bool) -> CG2Element;
}
extern "C" {
    pub fn CPrivateKeyGetG2Power(sk: CPrivateKey, el: CG2Element) -> CG2Element;
}
extern "C" {
    pub fn CPrivateKeyIsEqual(sk1: CPrivateKey, sk2: CPrivateKey) -> bool;
}
extern "C" {
    pub fn CPrivateKeySerialize(sk: CPrivateKey) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn CPrivateKeyFree(sk: CPrivateKey);
}
extern "C" {
    pub fn CPrivateKeySizeBytes() -> usize;
}
extern "C" {
    pub fn SecFree(p: *mut ::std::os::raw::c_void);
}
pub type carr = *mut *mut ::std::os::raw::c_void;
extern "C" {
    pub fn AllocPtrArray(len: usize) -> *mut *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn SetPtrArray(
        arrPtr: *mut *mut ::std::os::raw::c_void,
        elemPtr: *mut ::std::os::raw::c_void,
        index: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn FreePtrArray(inPtr: *mut *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn GetPtrAtIndex(
        arrPtr: *mut *mut ::std::os::raw::c_void,
        index: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn SecAllocBytes(len: usize) -> *mut u8;
}
extern "C" {
    pub fn GetAddressAtIndex(
        ptr: *mut u8,
        index: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn GetLastErrorMsg() -> *const ::std::os::raw::c_char;
}
pub type CCoreMPL = *mut ::std::os::raw::c_void;
pub type CBasicSchemeMPL = CCoreMPL;
pub type CAugSchemeMPL = CCoreMPL;
pub type CPopSchemeMPL = CCoreMPL;
extern "C" {
    pub fn CCoreMPLKeyGen(
        scheme: CCoreMPL,
        seed: *const ::std::os::raw::c_void,
        seedLen: usize,
        didErr: *mut bool,
    ) -> CPrivateKey;
}
extern "C" {
    pub fn CCoreMPSkToG1(scheme: CCoreMPL, sk: CPrivateKey) -> CG1Element;
}
extern "C" {
    pub fn CCoreMPLSign(
        scheme: CCoreMPL,
        sk: CPrivateKey,
        msg: *const ::std::os::raw::c_void,
        msgLen: usize,
    ) -> CG2Element;
}
extern "C" {
    pub fn CCoreMPLVerify(
        scheme: CBasicSchemeMPL,
        pk: CG1Element,
        msg: *const ::std::os::raw::c_void,
        msgLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn CCoreMPLAggregatePubKeys(
        scheme: CCoreMPL,
        pubKeys: *mut *mut ::std::os::raw::c_void,
        pkLen: usize,
    ) -> CG1Element;
}
extern "C" {
    pub fn CCoreMPLAggregateSigs(
        scheme: CCoreMPL,
        sigs: *mut *mut ::std::os::raw::c_void,
        sigLen: usize,
    ) -> CG2Element;
}
extern "C" {
    pub fn CCoreMPLDeriveChildSk(scheme: CCoreMPL, sk: CPrivateKey, index: u32) -> CPrivateKey;
}
extern "C" {
    pub fn CCoreMPLDeriveChildSkUnhardened(
        scheme: CCoreMPL,
        sk: CPrivateKey,
        index: u32,
    ) -> CPrivateKey;
}
extern "C" {
    pub fn CCoreMPLDeriveChildPkUnhardened(
        scheme: CCoreMPL,
        sk: CG1Element,
        index: u32,
    ) -> CG1Element;
}
extern "C" {
    pub fn CCoreMPLAggregateVerify(
        scheme: CCoreMPL,
        pks: *mut *mut ::std::os::raw::c_void,
        pkLen: usize,
        msgs: *mut *mut ::std::os::raw::c_void,
        msgLens: *const ::std::os::raw::c_void,
        msgLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn NewCBasicSchemeMPL() -> CBasicSchemeMPL;
}
extern "C" {
    pub fn CBasicSchemeMPLAggregateVerify(
        scheme: CBasicSchemeMPL,
        pks: *mut *mut ::std::os::raw::c_void,
        pksLen: usize,
        msgs: *mut *mut ::std::os::raw::c_void,
        msgsLens: *const ::std::os::raw::c_void,
        msgsLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn CBasicSchemeMPLFree(scheme: CBasicSchemeMPL);
}
extern "C" {
    pub fn NewCAugSchemeMPL() -> CAugSchemeMPL;
}
extern "C" {
    pub fn CAugSchemeMPLSign(
        scheme: CAugSchemeMPL,
        sk: CPrivateKey,
        msg: *const ::std::os::raw::c_void,
        msgLen: usize,
    ) -> CG2Element;
}
extern "C" {
    pub fn CAugSchemeMPLSignPrepend(
        scheme: CAugSchemeMPL,
        sk: CPrivateKey,
        msg: *const ::std::os::raw::c_void,
        msgLen: usize,
        prepPk: CG1Element,
    ) -> CG2Element;
}
extern "C" {
    pub fn CAugSchemeMPLVerify(
        scheme: CAugSchemeMPL,
        pk: CG1Element,
        msg: *const ::std::os::raw::c_void,
        msgLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn CAugSchemeMPLAggregateVerify(
        scheme: CAugSchemeMPL,
        pks: *mut *mut ::std::os::raw::c_void,
        pksLen: usize,
        msgs: *mut *mut ::std::os::raw::c_void,
        msgsLens: *const ::std::os::raw::c_void,
        msgsLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn CAugSchemeMPLFree(scheme: CAugSchemeMPL);
}
extern "C" {
    pub fn NewCPopSchemeMPL() -> CPopSchemeMPL;
}
extern "C" {
    pub fn CPopSchemeMPLPopProve(scheme: CPopSchemeMPL, sk: CPrivateKey) -> CG2Element;
}
extern "C" {
    pub fn CPopSchemeMPLPopVerify(scheme: CPopSchemeMPL, pk: CG1Element, sig: CG2Element) -> bool;
}
extern "C" {
    pub fn CPopSchemeMPLFastAggregateVerify(
        scheme: CPopSchemeMPL,
        pks: *mut *mut ::std::os::raw::c_void,
        pksLen: usize,
        msgs: *const ::std::os::raw::c_void,
        msgsLen: usize,
        sig: CG2Element,
    ) -> bool;
}
extern "C" {
    pub fn CPopSchemeMPLFree(scheme: CPopSchemeMPL);
}
pub const HashSize: ::std::os::raw::c_int = 32;
extern "C" {
    pub fn CThresholdPrivateKeyShare(
        sks: *mut *mut ::std::os::raw::c_void,
        sksLen: usize,
        hash: *const ::std::os::raw::c_void,
        didErr: *mut bool,
    ) -> CPrivateKey;
}
extern "C" {
    pub fn CThresholdPrivateKeyRecover(
        sks: *mut *mut ::std::os::raw::c_void,
        sksLen: usize,
        hashes: *mut *mut ::std::os::raw::c_void,
        hashesLen: usize,
        didErr: *mut bool,
    ) -> CPrivateKey;
}
extern "C" {
    pub fn CThresholdPublicKeyShare(
        pks: *mut *mut ::std::os::raw::c_void,
        pksLen: usize,
        hash: *const ::std::os::raw::c_void,
        didErr: *mut bool,
    ) -> CG1Element;
}
extern "C" {
    pub fn CThresholdPublicKeyRecover(
        pks: *mut *mut ::std::os::raw::c_void,
        pksLen: usize,
        hashes: *mut *mut ::std::os::raw::c_void,
        hashesLen: usize,
        didErr: *mut bool,
    ) -> CG1Element;
}
extern "C" {
    pub fn CThresholdSignatureShare(
        sigs: *mut *mut ::std::os::raw::c_void,
        sigsLen: usize,
        hash: *const ::std::os::raw::c_void,
        didErr: *mut bool,
    ) -> CG2Element;
}
extern "C" {
    pub fn CThresholdSignatureRecover(
        sigs: *mut *mut ::std::os::raw::c_void,
        sigsLen: usize,
        hashes: *mut *mut ::std::os::raw::c_void,
        hashesLen: usize,
        didErr: *mut bool,
    ) -> CG2Element;
}
extern "C" {
    pub fn CThresholdSign(sk: CPrivateKey, hash: *const ::std::os::raw::c_void) -> CG2Element;
}
extern "C" {
    pub fn CThresholdVerify(
        pk: CG1Element,
        hash: *const ::std::os::raw::c_void,
        sig: CG2Element,
    ) -> bool;
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type __uint128_t = u128;
