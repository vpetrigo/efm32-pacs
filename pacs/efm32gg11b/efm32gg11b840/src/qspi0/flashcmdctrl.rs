#[doc = "Register `FLASHCMDCTRL` reader"]
pub struct R(crate::R<FLASHCMDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHCMDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHCMDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHCMDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHCMDCTRL` writer"]
pub struct W(crate::W<FLASHCMDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHCMDCTRL_SPEC>;
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
impl From<crate::W<FLASHCMDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHCMDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDEXEC` writer - Execute the Command"]
pub type CMDEXEC_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 0>;
#[doc = "Field `CMDEXECSTATUS` reader - Command Execution in Progress"]
pub type CMDEXECSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `STIGMEMBANKEN` reader - STIG Memory Bank Enable Bit"]
pub type STIGMEMBANKEN_R = crate::BitReader<bool>;
#[doc = "Field `STIGMEMBANKEN` writer - STIG Memory Bank Enable Bit"]
pub type STIGMEMBANKEN_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 2>;
#[doc = "Field `NUMDUMMYCYCLES` reader - Number of Dummy Cycles"]
pub type NUMDUMMYCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMDUMMYCYCLES` writer - Number of Dummy Cycles"]
pub type NUMDUMMYCYCLES_W<'a> = crate::FieldWriter<'a, u32, FLASHCMDCTRL_SPEC, u8, u8, 5, 7>;
#[doc = "Field `NUMWRDATABYTES` reader - Number of Write Data Bytes"]
pub type NUMWRDATABYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMWRDATABYTES` writer - Number of Write Data Bytes"]
pub type NUMWRDATABYTES_W<'a> = crate::FieldWriter<'a, u32, FLASHCMDCTRL_SPEC, u8, u8, 3, 12>;
#[doc = "Field `ENBWRITEDATA` reader - Write Data Enable"]
pub type ENBWRITEDATA_R = crate::BitReader<bool>;
#[doc = "Field `ENBWRITEDATA` writer - Write Data Enable"]
pub type ENBWRITEDATA_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 15>;
#[doc = "Field `NUMADDRBYTES` reader - Number of Address Bytes"]
pub type NUMADDRBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMADDRBYTES` writer - Number of Address Bytes"]
pub type NUMADDRBYTES_W<'a> = crate::FieldWriter<'a, u32, FLASHCMDCTRL_SPEC, u8, u8, 2, 16>;
#[doc = "Field `ENBMODEBIT` reader - Mode Bit Enable"]
pub type ENBMODEBIT_R = crate::BitReader<bool>;
#[doc = "Field `ENBMODEBIT` writer - Mode Bit Enable"]
pub type ENBMODEBIT_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 18>;
#[doc = "Field `ENBCOMDADDR` reader - Command Address Enable"]
pub type ENBCOMDADDR_R = crate::BitReader<bool>;
#[doc = "Field `ENBCOMDADDR` writer - Command Address Enable"]
pub type ENBCOMDADDR_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 19>;
#[doc = "Field `NUMRDDATABYTES` reader - Number of Read Data Bytes"]
pub type NUMRDDATABYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMRDDATABYTES` writer - Number of Read Data Bytes"]
pub type NUMRDDATABYTES_W<'a> = crate::FieldWriter<'a, u32, FLASHCMDCTRL_SPEC, u8, u8, 3, 20>;
#[doc = "Field `ENBREADDATA` reader - Read Data Enable"]
pub type ENBREADDATA_R = crate::BitReader<bool>;
#[doc = "Field `ENBREADDATA` writer - Read Data Enable"]
pub type ENBREADDATA_W<'a> = crate::BitWriter<'a, u32, FLASHCMDCTRL_SPEC, bool, 23>;
#[doc = "Field `CMDOPCODE` reader - Command Opcode"]
pub type CMDOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDOPCODE` writer - Command Opcode"]
pub type CMDOPCODE_W<'a> = crate::FieldWriter<'a, u32, FLASHCMDCTRL_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bit 1 - Command Execution in Progress"]
    #[inline(always)]
    pub fn cmdexecstatus(&self) -> CMDEXECSTATUS_R {
        CMDEXECSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&self) -> STIGMEMBANKEN_R {
        STIGMEMBANKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&self) -> NUMDUMMYCYCLES_R {
        NUMDUMMYCYCLES_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&self) -> NUMWRDATABYTES_R {
        NUMWRDATABYTES_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&self) -> ENBWRITEDATA_R {
        ENBWRITEDATA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&self) -> ENBMODEBIT_R {
        ENBMODEBIT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&self) -> ENBCOMDADDR_R {
        ENBCOMDADDR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&self) -> NUMRDDATABYTES_R {
        NUMRDDATABYTES_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&self) -> ENBREADDATA_R {
        ENBREADDATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CMDOPCODE_R {
        CMDOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the Command"]
    #[inline(always)]
    pub fn cmdexec(&mut self) -> CMDEXEC_W {
        CMDEXEC_W::new(self)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&mut self) -> STIGMEMBANKEN_W {
        STIGMEMBANKEN_W::new(self)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&mut self) -> NUMDUMMYCYCLES_W {
        NUMDUMMYCYCLES_W::new(self)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&mut self) -> NUMWRDATABYTES_W {
        NUMWRDATABYTES_W::new(self)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&mut self) -> ENBWRITEDATA_W {
        ENBWRITEDATA_W::new(self)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W {
        NUMADDRBYTES_W::new(self)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&mut self) -> ENBMODEBIT_W {
        ENBMODEBIT_W::new(self)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&mut self) -> ENBCOMDADDR_W {
        ENBCOMDADDR_W::new(self)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&mut self) -> NUMRDDATABYTES_W {
        NUMRDDATABYTES_W::new(self)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&mut self) -> ENBREADDATA_W {
        ENBREADDATA_W::new(self)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&mut self) -> CMDOPCODE_W {
        CMDOPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Command Control Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcmdctrl](index.html) module"]
pub struct FLASHCMDCTRL_SPEC;
impl crate::RegisterSpec for FLASHCMDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashcmdctrl::R](R) reader structure"]
impl crate::Readable for FLASHCMDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashcmdctrl::W](W) writer structure"]
impl crate::Writable for FLASHCMDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHCMDCTRL to value 0"]
impl crate::Resettable for FLASHCMDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
