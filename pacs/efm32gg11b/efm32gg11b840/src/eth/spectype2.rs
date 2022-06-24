#[doc = "Register `SPECTYPE2` reader"]
pub struct R(crate::R<SPECTYPE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPECTYPE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPECTYPE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPECTYPE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPECTYPE2` writer"]
pub struct W(crate::W<SPECTYPE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPECTYPE2_SPEC>;
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
impl From<crate::W<SPECTYPE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPECTYPE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - Type ID match 2"]
pub type MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH` writer - Type ID match 2"]
pub type MATCH_W<'a> = crate::FieldWriter<'a, u32, SPECTYPE2_SPEC, u16, u16, 16, 0>;
#[doc = "Field `ENBCOPY` reader - Enable copying of type ID match 2 matched frames."]
pub type ENBCOPY_R = crate::BitReader<bool>;
#[doc = "Field `ENBCOPY` writer - Enable copying of type ID match 2 matched frames."]
pub type ENBCOPY_W<'a> = crate::BitWriter<'a, u32, SPECTYPE2_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 2"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 2 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> ENBCOPY_R {
        ENBCOPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 2"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W::new(self)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 2 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&mut self) -> ENBCOPY_W {
        ENBCOPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Type ID Match 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spectype2](index.html) module"]
pub struct SPECTYPE2_SPEC;
impl crate::RegisterSpec for SPECTYPE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spectype2::R](R) reader structure"]
impl crate::Readable for SPECTYPE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spectype2::W](W) writer structure"]
impl crate::Writable for SPECTYPE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPECTYPE2 to value 0"]
impl crate::Resettable for SPECTYPE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
