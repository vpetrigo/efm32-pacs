#[doc = "Register `CAPAB2` reader"]
pub struct R(crate::R<CAPAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDR50SUP` reader - SDR50 Support"]
pub type SDR50SUP_R = crate::BitReader<bool>;
#[doc = "Field `SDR104SUP` reader - SDR104 Support"]
pub type SDR104SUP_R = crate::BitReader<bool>;
#[doc = "Field `DDR50SUP` reader - DDR50 Support"]
pub type DDR50SUP_R = crate::BitReader<bool>;
#[doc = "Field `DRVTYPASUP` reader - Driver Type a Support"]
pub type DRVTYPASUP_R = crate::BitReader<bool>;
#[doc = "Field `DRVTYPCSUP` reader - Driver Type C Support"]
pub type DRVTYPCSUP_R = crate::BitReader<bool>;
#[doc = "Field `DRVTYPDSUP` reader - Driver Type D Support"]
pub type DRVTYPDSUP_R = crate::BitReader<bool>;
#[doc = "Field `TIMCNTRETUN` reader - Timer Count for Re-Tuning"]
pub type TIMCNTRETUN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USETUNSDR50` reader - Use Tuning for SDR50"]
pub type USETUNSDR50_R = crate::BitReader<bool>;
#[doc = "Field `RETUNEMODES` reader - Re-tuning Modes"]
pub type RETUNEMODES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCKKMUL` reader - Clock Multiplier"]
pub type CLOCKKMUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPIMODE` reader - SPI Mode Support"]
pub type SPIMODE_R = crate::BitReader<bool>;
#[doc = "Field `SPIBLOCKMODE` reader - SPI Block Mode Support"]
pub type SPIBLOCKMODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> SDR50SUP_R {
        SDR50SUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> SDR104SUP_R {
        SDR104SUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> DDR50SUP_R {
        DDR50SUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type a Support"]
    #[inline(always)]
    pub fn drvtypasup(&self) -> DRVTYPASUP_R {
        DRVTYPASUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvtypcsup(&self) -> DRVTYPCSUP_R {
        DRVTYPCSUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvtypdsup(&self) -> DRVTYPDSUP_R {
        DRVTYPDSUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn timcntretun(&self) -> TIMCNTRETUN_R {
        TIMCNTRETUN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn usetunsdr50(&self) -> USETUNSDR50_R {
        USETUNSDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning Modes"]
    #[inline(always)]
    pub fn retunemodes(&self) -> RETUNEMODES_R {
        RETUNEMODES_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clockkmul(&self) -> CLOCKKMUL_R {
        CLOCKKMUL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SPI Mode Support"]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI Block Mode Support"]
    #[inline(always)]
    pub fn spiblockmode(&self) -> SPIBLOCKMODE_R {
        SPIBLOCKMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Capabilities Register to Hold Bits 63~32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capab2](index.html) module"]
pub struct CAPAB2_SPEC;
impl crate::RegisterSpec for CAPAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capab2::R](R) reader structure"]
impl crate::Readable for CAPAB2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPAB2 to value 0"]
impl crate::Resettable for CAPAB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
