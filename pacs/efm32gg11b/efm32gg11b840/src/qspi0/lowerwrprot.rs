#[doc = "Register `LOWERWRPROT` reader"]
pub struct R(crate::R<LOWERWRPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOWERWRPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOWERWRPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOWERWRPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOWERWRPROT` writer"]
pub struct W(crate::W<LOWERWRPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOWERWRPROT_SPEC>;
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
impl From<crate::W<LOWERWRPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOWERWRPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSECTOR` reader - Lower Block Number"]
pub type SUBSECTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUBSECTOR` writer - Lower Block Number"]
pub type SUBSECTOR_W<'a> = crate::FieldWriter<'a, u32, LOWERWRPROT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Lower Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SUBSECTOR_R {
        SUBSECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower Block Number"]
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
#[doc = "Lower Write Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowerwrprot](index.html) module"]
pub struct LOWERWRPROT_SPEC;
impl crate::RegisterSpec for LOWERWRPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lowerwrprot::R](R) reader structure"]
impl crate::Readable for LOWERWRPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lowerwrprot::W](W) writer structure"]
impl crate::Writable for LOWERWRPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOWERWRPROT to value 0"]
impl crate::Resettable for LOWERWRPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
