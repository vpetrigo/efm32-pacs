#[doc = "Register `PHYCONFIGURATION` reader"]
pub struct R(crate::R<PHYCONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYCONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYCONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYCONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYCONFIGURATION` writer"]
pub struct W(crate::W<PHYCONFIGURATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYCONFIGURATION_SPEC>;
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
impl From<crate::W<PHYCONFIGURATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYCONFIGURATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYCONFIGRXDLLDELAY` reader - RX DLL Delay"]
pub type PHYCONFIGRXDLLDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYCONFIGRXDLLDELAY` writer - RX DLL Delay"]
pub type PHYCONFIGRXDLLDELAY_W<'a> =
    crate::FieldWriter<'a, u32, PHYCONFIGURATION_SPEC, u8, u8, 7, 0>;
#[doc = "Field `PHYCONFIGTXDLLDELAY` reader - TX DLL Delay"]
pub type PHYCONFIGTXDLLDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYCONFIGTXDLLDELAY` writer - TX DLL Delay"]
pub type PHYCONFIGTXDLLDELAY_W<'a> =
    crate::FieldWriter<'a, u32, PHYCONFIGURATION_SPEC, u8, u8, 7, 16>;
#[doc = "Field `PHYCONFIGRESYNC` writer - PHY Config Resync"]
pub type PHYCONFIGRESYNC_W<'a> = crate::BitWriter<'a, u32, PHYCONFIGURATION_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&self) -> PHYCONFIGRXDLLDELAY_R {
        PHYCONFIGRXDLLDELAY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&self) -> PHYCONFIGTXDLLDELAY_R {
        PHYCONFIGTXDLLDELAY_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigrxdlldelay(&mut self) -> PHYCONFIGRXDLLDELAY_W {
        PHYCONFIGRXDLLDELAY_W::new(self)
    }
    #[doc = "Bits 16:22 - TX DLL Delay"]
    #[inline(always)]
    pub fn phyconfigtxdlldelay(&mut self) -> PHYCONFIGTXDLLDELAY_W {
        PHYCONFIGTXDLLDELAY_W::new(self)
    }
    #[doc = "Bit 31 - PHY Config Resync"]
    #[inline(always)]
    pub fn phyconfigresync(&mut self) -> PHYCONFIGRESYNC_W {
        PHYCONFIGRESYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phyconfiguration](index.html) module"]
pub struct PHYCONFIGURATION_SPEC;
impl crate::RegisterSpec for PHYCONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phyconfiguration::R](R) reader structure"]
impl crate::Readable for PHYCONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phyconfiguration::W](W) writer structure"]
impl crate::Writable for PHYCONFIGURATION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHYCONFIGURATION to value 0"]
impl crate::Resettable for PHYCONFIGURATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
