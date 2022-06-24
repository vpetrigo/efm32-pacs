#[doc = "Register `TSUTIMERSEC` reader"]
pub struct R(crate::R<TSUTIMERSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERSEC` writer"]
pub struct W(crate::W<TSUTIMERSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERSEC_SPEC>;
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
impl From<crate::W<TSUTIMERSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - 1588 Timer Seconds Register"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER` writer - 1588 Timer Seconds Register"]
pub type TIMER_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERSEC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimersec](index.html) module"]
pub struct TSUTIMERSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimersec::R](R) reader structure"]
impl crate::Readable for TSUTIMERSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimersec::W](W) writer structure"]
impl crate::Writable for TSUTIMERSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERSEC to value 0"]
impl crate::Resettable for TSUTIMERSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
