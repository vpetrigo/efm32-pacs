#[doc = "Register `DEVINSTRWRCONFIG` reader"]
pub struct R(crate::R<DEVINSTRWRCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVINSTRWRCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVINSTRWRCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVINSTRWRCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVINSTRWRCONFIG` writer"]
pub struct W(crate::W<DEVINSTRWRCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVINSTRWRCONFIG_SPEC>;
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
impl From<crate::W<DEVINSTRWRCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVINSTRWRCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WROPCODE` reader - Write Opcode"]
pub type WROPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WROPCODE` writer - Write Opcode"]
pub type WROPCODE_W<'a> = crate::FieldWriter<'a, u32, DEVINSTRWRCONFIG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `WELDIS` reader - WEL Disable"]
pub type WELDIS_R = crate::BitReader<bool>;
#[doc = "Field `WELDIS` writer - WEL Disable"]
pub type WELDIS_W<'a> = crate::BitWriter<'a, u32, DEVINSTRWRCONFIG_SPEC, bool, 8>;
#[doc = "Field `ADDRXFERTYPESTDMODE` reader - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRXFERTYPESTDMODE` writer - Address Transfer Type for Standard SPI Modes"]
pub type ADDRXFERTYPESTDMODE_W<'a> =
    crate::FieldWriter<'a, u32, DEVINSTRWRCONFIG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DATAXFERTYPEEXTMODE` reader - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAXFERTYPEEXTMODE` writer - Data Transfer Type for Standard SPI Modes"]
pub type DATAXFERTYPEEXTMODE_W<'a> =
    crate::FieldWriter<'a, u32, DEVINSTRWRCONFIG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `DUMMYWRCLKCYCLES` reader - Dummy Write Clock Cycles"]
pub type DUMMYWRCLKCYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUMMYWRCLKCYCLES` writer - Dummy Write Clock Cycles"]
pub type DUMMYWRCLKCYCLES_W<'a> = crate::FieldWriter<'a, u32, DEVINSTRWRCONFIG_SPEC, u8, u8, 5, 24>;
impl R {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&self) -> WROPCODE_R {
        WROPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&self) -> WELDIS_R {
        WELDIS_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&self) -> DUMMYWRCLKCYCLES_R {
        DUMMYWRCLKCYCLES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&mut self) -> WROPCODE_W {
        WROPCODE_W::new(self)
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldis(&mut self) -> WELDIS_W {
        WELDIS_W::new(self)
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
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline(always)]
    pub fn dummywrclkcycles(&mut self) -> DUMMYWRCLKCYCLES_W {
        DUMMYWRCLKCYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Write Instruction Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinstrwrconfig](index.html) module"]
pub struct DEVINSTRWRCONFIG_SPEC;
impl crate::RegisterSpec for DEVINSTRWRCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devinstrwrconfig::R](R) reader structure"]
impl crate::Readable for DEVINSTRWRCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devinstrwrconfig::W](W) writer structure"]
impl crate::Writable for DEVINSTRWRCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVINSTRWRCONFIG to value 0x02"]
impl crate::Resettable for DEVINSTRWRCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
