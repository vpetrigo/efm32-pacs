#[doc = "Register `LEMCTRL` reader"]
pub struct R(crate::R<LEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEMCTRL` writer"]
pub struct W(crate::W<LEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEMCTRL_SPEC>;
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
impl From<crate::W<LEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEBASE` reader - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TIMEBASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEBASE` writer - Set the Number of LFC Clk Counts to Form 3ms"]
pub type TIMEBASE_W<'a> = crate::FieldWriter<'a, u32, LEMCTRL_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TIMEBASE_W {
        TIMEBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LEM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lemctrl](index.html) module"]
pub struct LEMCTRL_SPEC;
impl crate::RegisterSpec for LEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lemctrl::R](R) reader structure"]
impl crate::Readable for LEMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lemctrl::W](W) writer structure"]
impl crate::Writable for LEMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEMCTRL to value 0x67"]
impl crate::Resettable for LEMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x67
    }
}
