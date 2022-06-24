#[doc = "Register `DEVDELAY` reader"]
pub struct R(crate::R<DEVDELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVDELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVDELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVDELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVDELAY` writer"]
pub struct W(crate::W<DEVDELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVDELAY_SPEC>;
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
impl From<crate::W<DEVDELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVDELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DINIT` reader - Clock Delay for CS"]
pub type DINIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DINIT` writer - Clock Delay for CS"]
pub type DINIT_W<'a> = crate::FieldWriter<'a, u32, DEVDELAY_SPEC, u8, u8, 8, 0>;
#[doc = "Field `DAFTER` reader - Clock Delay for Last Transaction Bit"]
pub type DAFTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAFTER` writer - Clock Delay for Last Transaction Bit"]
pub type DAFTER_W<'a> = crate::FieldWriter<'a, u32, DEVDELAY_SPEC, u8, u8, 8, 8>;
#[doc = "Field `DBTWN` reader - Clock Delay Between Two Chip Selects"]
pub type DBTWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBTWN` writer - Clock Delay Between Two Chip Selects"]
pub type DBTWN_W<'a> = crate::FieldWriter<'a, u32, DEVDELAY_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DNSS` reader - Clock Delay for Chip Select Deassert"]
pub type DNSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNSS` writer - Clock Delay for Chip Select Deassert"]
pub type DNSS_W<'a> = crate::FieldWriter<'a, u32, DEVDELAY_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&self) -> DINIT_R {
        DINIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&self) -> DAFTER_R {
        DAFTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&self) -> DBTWN_R {
        DBTWN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&self) -> DNSS_R {
        DNSS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&mut self) -> DINIT_W {
        DINIT_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&mut self) -> DAFTER_W {
        DAFTER_W::new(self)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&mut self) -> DBTWN_W {
        DBTWN_W::new(self)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&mut self) -> DNSS_W {
        DNSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdelay](index.html) module"]
pub struct DEVDELAY_SPEC;
impl crate::RegisterSpec for DEVDELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devdelay::R](R) reader structure"]
impl crate::Readable for DEVDELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devdelay::W](W) writer structure"]
impl crate::Writable for DEVDELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVDELAY to value 0"]
impl crate::Resettable for DEVDELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
