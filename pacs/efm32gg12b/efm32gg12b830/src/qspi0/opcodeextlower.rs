#[doc = "Register `OPCODEEXTLOWER` reader"]
pub struct R(crate::R<OPCODEEXTLOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPCODEEXTLOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPCODEEXTLOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPCODEEXTLOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPCODEEXTLOWER` writer"]
pub struct W(crate::W<OPCODEEXTLOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPCODEEXTLOWER_SPEC>;
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
impl From<crate::W<OPCODEEXTLOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPCODEEXTLOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTSTIGOPCODE` reader - STIG Opcode Extension"]
pub type EXTSTIGOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTSTIGOPCODE` writer - STIG Opcode Extension"]
pub type EXTSTIGOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTLOWER_SPEC, u8, u8, 8, 0>;
#[doc = "Field `EXTPOLLOPCODE` reader - Polling Opcode Extension"]
pub type EXTPOLLOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTPOLLOPCODE` writer - Polling Opcode Extension"]
pub type EXTPOLLOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTLOWER_SPEC, u8, u8, 8, 8>;
#[doc = "Field `EXTWRITEOPCODE` reader - Write Opcode Extension"]
pub type EXTWRITEOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTWRITEOPCODE` writer - Write Opcode Extension"]
pub type EXTWRITEOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTLOWER_SPEC, u8, u8, 8, 16>;
#[doc = "Field `EXTREADOPCODE` reader - Read Opcode Extension"]
pub type EXTREADOPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTREADOPCODE` writer - Read Opcode Extension"]
pub type EXTREADOPCODE_W<'a> = crate::FieldWriter<'a, u32, OPCODEEXTLOWER_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&self) -> EXTSTIGOPCODE_R {
        EXTSTIGOPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&self) -> EXTPOLLOPCODE_R {
        EXTPOLLOPCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&self) -> EXTWRITEOPCODE_R {
        EXTWRITEOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&self) -> EXTREADOPCODE_R {
        EXTREADOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&mut self) -> EXTSTIGOPCODE_W {
        EXTSTIGOPCODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&mut self) -> EXTPOLLOPCODE_W {
        EXTPOLLOPCODE_W::new(self)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&mut self) -> EXTWRITEOPCODE_W {
        EXTWRITEOPCODE_W::new(self)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&mut self) -> EXTREADOPCODE_W {
        EXTREADOPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opcode Extension Register (Lower)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcodeextlower](index.html) module"]
pub struct OPCODEEXTLOWER_SPEC;
impl crate::RegisterSpec for OPCODEEXTLOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opcodeextlower::R](R) reader structure"]
impl crate::Readable for OPCODEEXTLOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opcodeextlower::W](W) writer structure"]
impl crate::Writable for OPCODEEXTLOWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPCODEEXTLOWER to value 0x13ed_fa00"]
impl crate::Resettable for OPCODEEXTLOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13ed_fa00
    }
}
