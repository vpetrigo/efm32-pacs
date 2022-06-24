#[doc = "Register `RNGSTATUS` reader"]
pub struct R(crate::R<RNGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNGSTATUS` writer"]
pub struct W(crate::W<RNGSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RNGSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNGSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Test Data Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTDATABUSY_A {
    #[doc = "0: TESTDATA write is finished processing or no test in progress."]
    IDLE = 0,
    #[doc = "1: TESTDATA write is still being processed."]
    BUSY = 1,
}
impl From<TESTDATABUSY_A> for bool {
    #[inline(always)]
    fn from(variant: TESTDATABUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTDATABUSY` reader - Test Data Busy"]
pub type TESTDATABUSY_R = crate::BitReader<TESTDATABUSY_A>;
impl TESTDATABUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTDATABUSY_A {
        match self.bits {
            false => TESTDATABUSY_A::IDLE,
            true => TESTDATABUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TESTDATABUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TESTDATABUSY_A::BUSY
    }
}
#[doc = "State of the control FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: RESET State"]
    RESET = 0,
    #[doc = "1: STARTUP State"]
    STARTUP = 1,
    #[doc = "2: FIFOFULLON State"]
    FIFOFULLON = 2,
    #[doc = "3: FIFOFULLOFF State"]
    FIFOFULLOFF = 3,
    #[doc = "4: RUNNING State"]
    RUNNING = 4,
    #[doc = "5: ERROR State"]
    ERROR = 5,
    #[doc = "6: UNUSED"]
    UNUSED_6 = 6,
    #[doc = "7: UNUSED"]
    UNUSED_7 = 7,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - State of the control FSM"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            0 => STATE_A::RESET,
            1 => STATE_A::STARTUP,
            2 => STATE_A::FIFOFULLON,
            3 => STATE_A::FIFOFULLOFF,
            4 => STATE_A::RUNNING,
            5 => STATE_A::ERROR,
            6 => STATE_A::UNUSED_6,
            7 => STATE_A::UNUSED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == STATE_A::RESET
    }
    #[doc = "Checks if the value of the field is `STARTUP`"]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == STATE_A::STARTUP
    }
    #[doc = "Checks if the value of the field is `FIFOFULLON`"]
    #[inline(always)]
    pub fn is_fifofullon(&self) -> bool {
        *self == STATE_A::FIFOFULLON
    }
    #[doc = "Checks if the value of the field is `FIFOFULLOFF`"]
    #[inline(always)]
    pub fn is_fifofulloff(&self) -> bool {
        *self == STATE_A::FIFOFULLOFF
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == STATE_A::ERROR
    }
    #[doc = "Checks if the value of the field is `UNUSED_6`"]
    #[inline(always)]
    pub fn is_unused_6(&self) -> bool {
        *self == STATE_A::UNUSED_6
    }
    #[doc = "Checks if the value of the field is `UNUSED_7`"]
    #[inline(always)]
    pub fn is_unused_7(&self) -> bool {
        *self == STATE_A::UNUSED_7
    }
}
#[doc = "Field `REPCOUNTIF` reader - Repetition Count Test interrupt status"]
pub type REPCOUNTIF_R = crate::BitReader<bool>;
#[doc = "Field `APT64IF` reader - 64-sample window Adaptive Proportion IF"]
pub type APT64IF_R = crate::BitReader<bool>;
#[doc = "Field `APT4096IF` reader - 4096-sample window Adaptive Prop. IF"]
pub type APT4096IF_R = crate::BitReader<bool>;
#[doc = "Field `FULLIF` reader - FIFO full interrupt status"]
pub type FULLIF_R = crate::BitReader<bool>;
#[doc = "Field `PREIF` reader - AIS31 Preliminary Noise Alarm IF"]
pub type PREIF_R = crate::BitReader<bool>;
#[doc = "Field `PREIF` writer - AIS31 Preliminary Noise Alarm IF"]
pub type PREIF_W<'a> = crate::BitWriter<'a, u32, RNGSTATUS_SPEC, bool, 8>;
#[doc = "Field `ALMIF` reader - AIS31 Noise Alarm interrupt status"]
pub type ALMIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Test Data Busy"]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TESTDATABUSY_R {
        TESTDATABUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Repetition Count Test interrupt status"]
    #[inline(always)]
    pub fn repcountif(&self) -> REPCOUNTIF_R {
        REPCOUNTIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 64-sample window Adaptive Proportion IF"]
    #[inline(always)]
    pub fn apt64if(&self) -> APT64IF_R {
        APT64IF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 4096-sample window Adaptive Prop. IF"]
    #[inline(always)]
    pub fn apt4096if(&self) -> APT4096IF_R {
        APT4096IF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO full interrupt status"]
    #[inline(always)]
    pub fn fullif(&self) -> FULLIF_R {
        FULLIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm IF"]
    #[inline(always)]
    pub fn preif(&self) -> PREIF_R {
        PREIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn almif(&self) -> ALMIF_R {
        ALMIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm IF"]
    #[inline(always)]
    pub fn preif(&mut self) -> PREIF_W {
        PREIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rngstatus](index.html) module"]
pub struct RNGSTATUS_SPEC;
impl crate::RegisterSpec for RNGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rngstatus::R](R) reader structure"]
impl crate::Readable for RNGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rngstatus::W](W) writer structure"]
impl crate::Writable for RNGSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNGSTATUS to value 0"]
impl crate::Resettable for RNGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
