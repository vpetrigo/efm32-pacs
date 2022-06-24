#[doc = "Register `PBUFRXCUTTHRU` reader"]
pub struct R(crate::R<PBUFRXCUTTHRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBUFRXCUTTHRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBUFRXCUTTHRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBUFRXCUTTHRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBUFRXCUTTHRU` writer"]
pub struct W(crate::W<PBUFRXCUTTHRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBUFRXCUTTHRU_SPEC>;
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
impl From<crate::W<PBUFRXCUTTHRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBUFRXCUTTHRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARXCUTTHRUTHR` reader - Watermark value"]
pub type DMARXCUTTHRUTHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMARXCUTTHRUTHR` writer - Watermark value"]
pub type DMARXCUTTHRUTHR_W<'a> = crate::FieldWriter<'a, u32, PBUFRXCUTTHRU_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DMARXCUTTHRU` reader - Enable RX partial store and forward operation"]
pub type DMARXCUTTHRU_R = crate::BitReader<bool>;
#[doc = "Field `DMARXCUTTHRU` writer - Enable RX partial store and forward operation"]
pub type DMARXCUTTHRU_W<'a> = crate::BitWriter<'a, u32, PBUFRXCUTTHRU_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&self) -> DMARXCUTTHRUTHR_R {
        DMARXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&self) -> DMARXCUTTHRU_R {
        DMARXCUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&mut self) -> DMARXCUTTHRUTHR_W {
        DMARXCUTTHRUTHR_W::new(self)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&mut self) -> DMARXCUTTHRU_W {
        DMARXCUTTHRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbufrxcutthru](index.html) module"]
pub struct PBUFRXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUFRXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbufrxcutthru::R](R) reader structure"]
impl crate::Readable for PBUFRXCUTTHRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbufrxcutthru::W](W) writer structure"]
impl crate::Writable for PBUFRXCUTTHRU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBUFRXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PBUFRXCUTTHRU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
