#[doc = "Register `ESAUMRB56` reader"]
pub struct R(crate::R<ESAUMRB56_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESAUMRB56_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESAUMRB56_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESAUMRB56_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESAUMRB56` writer"]
pub struct W(crate::W<ESAUMRB56_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESAUMRB56_SPEC>;
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
impl From<crate::W<ESAUMRB56_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESAUMRB56_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESAUMRB56` reader - Moveable Region Boundary 5-6"]
pub type ESAUMRB56_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ESAUMRB56` writer - Moveable Region Boundary 5-6"]
pub type ESAUMRB56_W<'a> = crate::FieldWriter<'a, u32, ESAUMRB56_SPEC, u16, u16, 16, 12>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary 5-6"]
    #[inline(always)]
    pub fn esaumrb56(&self) -> ESAUMRB56_R {
        ESAUMRB56_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary 5-6"]
    #[inline(always)]
    pub fn esaumrb56(&mut self) -> ESAUMRB56_W {
        ESAUMRB56_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specify the boundary between regions 5 and 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esaumrb56](index.html) module"]
pub struct ESAUMRB56_SPEC;
impl crate::RegisterSpec for ESAUMRB56_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esaumrb56::R](R) reader structure"]
impl crate::Readable for ESAUMRB56_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esaumrb56::W](W) writer structure"]
impl crate::Writable for ESAUMRB56_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESAUMRB56 to value 0x0400_0000"]
impl crate::Resettable for ESAUMRB56_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
