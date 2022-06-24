#[doc = "Register `PORTD_CTRL` reader"]
pub struct R(crate::R<PORTD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTD_CTRL` writer"]
pub struct W(crate::W<PORTD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTD_CTRL_SPEC>;
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
impl From<crate::W<PORTD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEWRATE` reader - Slew Rate"]
pub type SLEWRATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEWRATE` writer - Slew Rate"]
pub type SLEWRATE_W<'a> = crate::FieldWriter<'a, u32, PORTD_CTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `DINDIS` reader - Data In Disable"]
pub type DINDIS_R = crate::BitReader<bool>;
#[doc = "Field `DINDIS` writer - Data In Disable"]
pub type DINDIS_W<'a> = crate::BitWriter<'a, u32, PORTD_CTRL_SPEC, bool, 12>;
#[doc = "Field `SLEWRATEALT` reader - Slew Rate Alt"]
pub type SLEWRATEALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEWRATEALT` writer - Slew Rate Alt"]
pub type SLEWRATEALT_W<'a> = crate::FieldWriter<'a, u32, PORTD_CTRL_SPEC, u8, u8, 3, 20>;
#[doc = "Field `DINDISALT` reader - Data In Disable Alt"]
pub type DINDISALT_R = crate::BitReader<bool>;
#[doc = "Field `DINDISALT` writer - Data In Disable Alt"]
pub type DINDISALT_W<'a> = crate::BitWriter<'a, u32, PORTD_CTRL_SPEC, bool, 28>;
impl R {
    #[doc = "Bits 4:6 - Slew Rate"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data In Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DINDIS_R {
        DINDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Slew Rate Alt"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SLEWRATEALT_R {
        SLEWRATEALT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Data In Disable Alt"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DINDISALT_R {
        DINDISALT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Slew Rate"]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SLEWRATE_W {
        SLEWRATE_W::new(self)
    }
    #[doc = "Bit 12 - Data In Disable"]
    #[inline(always)]
    pub fn dindis(&mut self) -> DINDIS_W {
        DINDIS_W::new(self)
    }
    #[doc = "Bits 20:22 - Slew Rate Alt"]
    #[inline(always)]
    pub fn slewratealt(&mut self) -> SLEWRATEALT_W {
        SLEWRATEALT_W::new(self)
    }
    #[doc = "Bit 28 - Data In Disable Alt"]
    #[inline(always)]
    pub fn dindisalt(&mut self) -> DINDISALT_W {
        DINDISALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portd_ctrl](index.html) module"]
pub struct PORTD_CTRL_SPEC;
impl crate::RegisterSpec for PORTD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portd_ctrl::R](R) reader structure"]
impl crate::Readable for PORTD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portd_ctrl::W](W) writer structure"]
impl crate::Writable for PORTD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTD_CTRL to value 0x0040_0040"]
impl crate::Resettable for PORTD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0040
    }
}
