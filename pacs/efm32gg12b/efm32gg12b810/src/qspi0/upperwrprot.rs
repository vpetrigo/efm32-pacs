#[doc = "Register `UPPERWRPROT` reader"]
pub struct R(crate::R<UPPERWRPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPPERWRPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPPERWRPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPPERWRPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPPERWRPROT` writer"]
pub struct W(crate::W<UPPERWRPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPPERWRPROT_SPEC>;
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
impl From<crate::W<UPPERWRPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPPERWRPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSECTOR` reader - Upper Block Number"]
pub type SUBSECTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUBSECTOR` writer - Upper Block Number"]
pub type SUBSECTOR_W<'a> = crate::FieldWriter<'a, u32, UPPERWRPROT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SUBSECTOR_R {
        SUBSECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&mut self) -> SUBSECTOR_W {
        SUBSECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper Write Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upperwrprot](index.html) module"]
pub struct UPPERWRPROT_SPEC;
impl crate::RegisterSpec for UPPERWRPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upperwrprot::R](R) reader structure"]
impl crate::Readable for UPPERWRPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upperwrprot::W](W) writer structure"]
impl crate::Writable for UPPERWRPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPPERWRPROT to value 0"]
impl crate::Resettable for UPPERWRPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
