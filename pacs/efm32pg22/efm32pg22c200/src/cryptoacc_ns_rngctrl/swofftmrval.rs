#[doc = "Register `SWOFFTMRVAL` reader"]
pub struct R(crate::R<SWOFFTMRVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWOFFTMRVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWOFFTMRVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWOFFTMRVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWOFFTMRVAL` writer"]
pub struct W(crate::W<SWOFFTMRVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWOFFTMRVAL_SPEC>;
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
impl From<crate::W<SWOFFTMRVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWOFFTMRVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWOFFTMRVAL` reader - Switch Off Timer Value"]
pub type SWOFFTMRVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SWOFFTMRVAL` writer - Switch Off Timer Value"]
pub type SWOFFTMRVAL_W<'a> = crate::FieldWriter<'a, u32, SWOFFTMRVAL_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Switch Off Timer Value"]
    #[inline(always)]
    pub fn swofftmrval(&self) -> SWOFFTMRVAL_R {
        SWOFFTMRVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Switch Off Timer Value"]
    #[inline(always)]
    pub fn swofftmrval(&mut self) -> SWOFFTMRVAL_W {
        SWOFFTMRVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of clk cycles to wait before stopping the rings after the FIFO is full\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swofftmrval](index.html) module"]
pub struct SWOFFTMRVAL_SPEC;
impl crate::RegisterSpec for SWOFFTMRVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swofftmrval::R](R) reader structure"]
impl crate::Readable for SWOFFTMRVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swofftmrval::W](W) writer structure"]
impl crate::Writable for SWOFFTMRVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWOFFTMRVAL to value 0xffff"]
impl crate::Resettable for SWOFFTMRVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
