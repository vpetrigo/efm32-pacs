#[doc = "Register `SPECADDR1TOP` reader"]
pub struct R(crate::R<SPECADDR1TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPECADDR1TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPECADDR1TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPECADDR1TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPECADDR1TOP` writer"]
pub struct W(crate::W<SPECADDR1TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPECADDR1TOP_SPEC>;
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
impl From<crate::W<SPECADDR1TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPECADDR1TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Specific address 1 MSB"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Specific address 1 MSB"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, SPECADDR1TOP_SPEC, u16, u16, 16, 0>;
#[doc = "Field `FILTERTYPE` reader - MAC SA or DA selection"]
pub type FILTERTYPE_R = crate::BitReader<bool>;
#[doc = "Field `FILTERTYPE` writer - MAC SA or DA selection"]
pub type FILTERTYPE_W<'a> = crate::BitWriter<'a, u32, SPECADDR1TOP_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FILTERTYPE_R {
        FILTERTYPE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&mut self) -> FILTERTYPE_W {
        FILTERTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 1 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [specaddr1top](index.html) module"]
pub struct SPECADDR1TOP_SPEC;
impl crate::RegisterSpec for SPECADDR1TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [specaddr1top::R](R) reader structure"]
impl crate::Readable for SPECADDR1TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [specaddr1top::W](W) writer structure"]
impl crate::Writable for SPECADDR1TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPECADDR1TOP to value 0"]
impl crate::Resettable for SPECADDR1TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
