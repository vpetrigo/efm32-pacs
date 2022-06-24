#[doc = "Register `BOOTTOCTRL` reader"]
pub struct R(crate::R<BOOTTOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTTOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTTOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTTOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTTOCTRL` writer"]
pub struct W(crate::W<BOOTTOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTTOCTRL_SPEC>;
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
impl From<crate::W<BOOTTOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTTOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTDATTOCNT` reader - Boot Data Timeout Counter Value"]
pub type BOOTDATTOCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BOOTDATTOCNT` writer - Boot Data Timeout Counter Value"]
pub type BOOTDATTOCNT_W<'a> = crate::FieldWriter<'a, u32, BOOTTOCTRL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    pub fn bootdattocnt(&self) -> BOOTDATTOCNT_R {
        BOOTDATTOCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    pub fn bootdattocnt(&mut self) -> BOOTDATTOCNT_W {
        BOOTDATTOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boottoctrl](index.html) module"]
pub struct BOOTTOCTRL_SPEC;
impl crate::RegisterSpec for BOOTTOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boottoctrl::R](R) reader structure"]
impl crate::Readable for BOOTTOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boottoctrl::W](W) writer structure"]
impl crate::Writable for BOOTTOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOTTOCTRL to value 0"]
impl crate::Resettable for BOOTTOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
