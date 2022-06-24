#[doc = "Register `SEGD3` reader"]
pub struct R(crate::R<SEGD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD3` writer"]
pub struct W(crate::W<SEGD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD3_SPEC>;
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
impl From<crate::W<SEGD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD3` reader - COM3 Segment Data Low"]
pub type SEGD3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGD3` writer - COM3 Segment Data Low"]
pub type SEGD3_W<'a> = crate::FieldWriter<'a, u32, SEGD3_SPEC, u32, u32, 20, 0>;
impl R {
    #[doc = "Bits 0:19 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3(&self) -> SEGD3_R {
        SEGD3_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3(&mut self) -> SEGD3_W {
        SEGD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd3](index.html) module"]
pub struct SEGD3_SPEC;
impl crate::RegisterSpec for SEGD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd3::R](R) reader structure"]
impl crate::Readable for SEGD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd3::W](W) writer structure"]
impl crate::Writable for SEGD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEGD3 to value 0"]
impl crate::Resettable for SEGD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
