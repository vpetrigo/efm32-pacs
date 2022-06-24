#[doc = "Register `PBUFTXCUTTHRU` reader"]
pub struct R(crate::R<PBUFTXCUTTHRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBUFTXCUTTHRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBUFTXCUTTHRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBUFTXCUTTHRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBUFTXCUTTHRU` writer"]
pub struct W(crate::W<PBUFTXCUTTHRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBUFTXCUTTHRU_SPEC>;
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
impl From<crate::W<PBUFTXCUTTHRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBUFTXCUTTHRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATXCUTTHRUTHR` reader - Watermark value"]
pub type DMATXCUTTHRUTHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMATXCUTTHRUTHR` writer - Watermark value"]
pub type DMATXCUTTHRUTHR_W<'a> = crate::FieldWriter<'a, u32, PBUFTXCUTTHRU_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DMATXCUTTHRU` reader - Enable TX partial store and forward operation"]
pub type DMATXCUTTHRU_R = crate::BitReader<bool>;
#[doc = "Field `DMATXCUTTHRU` writer - Enable TX partial store and forward operation"]
pub type DMATXCUTTHRU_W<'a> = crate::BitWriter<'a, u32, PBUFTXCUTTHRU_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&self) -> DMATXCUTTHRUTHR_R {
        DMATXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&self) -> DMATXCUTTHRU_R {
        DMATXCUTTHRU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&mut self) -> DMATXCUTTHRUTHR_W {
        DMATXCUTTHRUTHR_W::new(self)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&mut self) -> DMATXCUTTHRU_W {
        DMATXCUTTHRU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbuftxcutthru](index.html) module"]
pub struct PBUFTXCUTTHRU_SPEC;
impl crate::RegisterSpec for PBUFTXCUTTHRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbuftxcutthru::R](R) reader structure"]
impl crate::Readable for PBUFTXCUTTHRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbuftxcutthru::W](W) writer structure"]
impl crate::Writable for PBUFTXCUTTHRU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBUFTXCUTTHRU to value 0x03ff"]
impl crate::Resettable for PBUFTXCUTTHRU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
