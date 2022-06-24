#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERASE` reader - Erase Done Interrupt Read Flag"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` reader - Write Done Interrupt Read Flag"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `CHOF` reader - Cache Hits Overflow Interrupt Flag"]
pub type CHOF_R = crate::BitReader<bool>;
#[doc = "Field `CMOF` reader - Cache Misses Overflow Interrupt Flag"]
pub type CMOF_R = crate::BitReader<bool>;
#[doc = "Field `PWRUPF` reader - Flash Power Up Sequence Complete Flag"]
pub type PWRUPF_R = crate::BitReader<bool>;
#[doc = "Field `ICACHERR` reader - ICache RAM Parity Error Flag"]
pub type ICACHERR_R = crate::BitReader<bool>;
#[doc = "Field `WDATAOV` reader - Flash Controller Write Buffer Overflow"]
pub type WDATAOV_R = crate::BitReader<bool>;
#[doc = "Field `LVEWRITE` reader - Flash LVE Write Error Flag"]
pub type LVEWRITE_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR1B` reader - RAM 1-bit ECC Error Interrupt Flag"]
pub type RAMERR1B_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR2B` reader - RAM 2-bit ECC Error Interrupt Flag"]
pub type RAMERR2B_R = crate::BitReader<bool>;
#[doc = "Field `RAM1ERR1B` reader - RAM1 1-bit ECC Error Interrupt Flag"]
pub type RAM1ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `RAM1ERR2B` reader - RAM1 2-bit ECC Error Interrupt Flag"]
pub type RAM1ERR2B_R = crate::BitReader<bool>;
#[doc = "Field `RAM2ERR1B` reader - RAM2 1-bit ECC Error Interrupt Flag"]
pub type RAM2ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `RAM2ERR2B` reader - RAM2 2-bit ECC Error Interrupt Flag"]
pub type RAM2ERR2B_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICache RAM Parity Error Flag"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flash Controller Write Buffer Overflow"]
    #[inline(always)]
    pub fn wdataov(&self) -> WDATAOV_R {
        WDATAOV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash LVE Write Error Flag"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LVEWRITE_R {
        LVEWRITE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> RAMERR1B_R {
        RAMERR1B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> RAMERR2B_R {
        RAMERR2B_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM1 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&self) -> RAM1ERR1B_R {
        RAM1ERR1B_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM1 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&self) -> RAM1ERR2B_R {
        RAM1ERR2B_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RAM2 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram2err1b(&self) -> RAM2ERR1B_R {
        RAM2ERR1B_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RAM2 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram2err2b(&self) -> RAM2ERR2B_R {
        RAM2ERR2B_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
