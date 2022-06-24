#[doc = "Register `TSUTIMERMSBSEC` reader"]
pub struct R(crate::R<TSUTIMERMSBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERMSBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERMSBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERMSBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERMSBSEC` writer"]
pub struct W(crate::W<TSUTIMERMSBSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERMSBSEC_SPEC>;
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
impl From<crate::W<TSUTIMERMSBSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERMSBSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - MSB 16 bits of seconds timer count."]
pub type TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER` writer - MSB 16 bits of seconds timer count."]
pub type TIMER_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERMSBSEC_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
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
#[doc = "1588 Timer Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimermsbsec](index.html) module"]
pub struct TSUTIMERMSBSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERMSBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimermsbsec::R](R) reader structure"]
impl crate::Readable for TSUTIMERMSBSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimermsbsec::W](W) writer structure"]
impl crate::Writable for TSUTIMERMSBSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERMSBSEC to value 0"]
impl crate::Resettable for TSUTIMERMSBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
