#[doc = "Register `DEVINSTRRDCONFIG` reader"]
pub struct R(crate::R<DEVINSTRRDCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVINSTRRDCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVINSTRRDCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVINSTRRDCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVINSTRRDCONFIG` writer"]
pub struct W(crate::W<DEVINSTRRDCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINSTRRDCONFIG_SPEC>;
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
impl From<crate::W<DEVINSTRRDCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINSTRRDCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDOPCODENONXIP` reader - Read Opcode in Non-XIP Mode"]
pub type RDOPCODENONXIP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDOPCODENONXIP` writer - Read Opcode in Non-XIP Mode"]
pub type RDOPCODENONXIP_W<'a> = crate::FieldWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `INSTRTYPE` reader - Instruction Type"]
pub type INSTRTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRTYPE` writer - Instruction Type"]
pub type INSTRTYPE_W<'a> = crate::FieldWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `DDREN` reader - DDR Enable"]
pub type DDREN_R = crate::BitReader<bool>;
#[doc = "Field `DDREN` writer - DDR Enable"]
pub type DDREN_W<'a> = crate::BitWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, bool, 10>;
#[doc = "Field `ADDRXFERTYPESTDMODE` reader - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRXFERTYPESTDMODE` writer - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_W<'a> =
    crate::FieldWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DATAXFERTYPEEXTMODE` reader - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAXFERTYPEEXTMODE` writer - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_W<'a> =
    crate::FieldWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `MODEBITENABLE` reader - Mode Bit Enable"]
pub type MODEBITENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MODEBITENABLE` writer - Mode Bit Enable"]
pub type MODEBITENABLE_W<'a> = crate::BitWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, bool, 20>;
#[doc = "Field `DUMMYRDCLKCYCLES` reader - Dummy Read Clock Cycles"]
pub type DUMMYRDCLKCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUMMYRDCLKCYCLES` writer - Dummy Read Clock Cycles"]
pub type DUMMYRDCLKCYCLES_W<'a> = crate::FieldWriter<'a, u32, DEVINSTRRDCONFIG_SPEC, u8, u8, 5, 24>;
impl R {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&self) -> RDOPCODENONXIP_R {
        RDOPCODENONXIP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> INSTRTYPE_R {
        INSTRTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DDREN_R {
        DDREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&self) -> ADDRXFERTYPESTDMODE_R {
        ADDRXFERTYPESTDMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&self) -> DATAXFERTYPEEXTMODE_R {
        DATAXFERTYPEEXTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&self) -> MODEBITENABLE_R {
        MODEBITENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&self) -> DUMMYRDCLKCYCLES_R {
        DUMMYRDCLKCYCLES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&mut self) -> RDOPCODENONXIP_W {
        RDOPCODENONXIP_W::new(self)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&mut self) -> INSTRTYPE_W {
        INSTRTYPE_W::new(self)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&mut self) -> DDREN_W {
        DDREN_W::new(self)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&mut self) -> ADDRXFERTYPESTDMODE_W {
        ADDRXFERTYPESTDMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&mut self) -> DATAXFERTYPEEXTMODE_W {
        DATAXFERTYPEEXTMODE_W::new(self)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&mut self) -> MODEBITENABLE_W {
        MODEBITENABLE_W::new(self)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&mut self) -> DUMMYRDCLKCYCLES_W {
        DUMMYRDCLKCYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Read Instruction Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinstrrdconfig](index.html) module"]
pub struct DEVINSTRRDCONFIG_SPEC;
impl crate::RegisterSpec for DEVINSTRRDCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devinstrrdconfig::R](R) reader structure"]
impl crate::Readable for DEVINSTRRDCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devinstrrdconfig::W](W) writer structure"]
impl crate::Writable for DEVINSTRRDCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINSTRRDCONFIG to value 0x03"]
impl crate::Resettable for DEVINSTRRDCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
