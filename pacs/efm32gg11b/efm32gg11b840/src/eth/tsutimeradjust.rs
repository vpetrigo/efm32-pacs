#[doc = "Register `TSUTIMERADJUST` reader"]
pub struct R(crate::R<TSUTIMERADJUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERADJUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERADJUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERADJUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERADJUST` writer"]
pub struct W(crate::W<TSUTIMERADJUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERADJUST_SPEC>;
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
impl From<crate::W<TSUTIMERADJUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERADJUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCREMENTVAL` reader - Timer increment value"]
pub type INCREMENTVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INCREMENTVAL` writer - Timer increment value"]
pub type INCREMENTVAL_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERADJUST_SPEC, u32, u32, 30, 0>;
#[doc = "Field `ADDSUBTRACT` reader - Write as one to subtract from the 1588 timer"]
pub type ADDSUBTRACT_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUBTRACT` writer - Write as one to subtract from the 1588 timer"]
pub type ADDSUBTRACT_W<'a> = crate::BitWriter<'a, u32, TSUTIMERADJUST_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&self) -> INCREMENTVAL_R {
        INCREMENTVAL_R::new((self.bits & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&self) -> ADDSUBTRACT_R {
        ADDSUBTRACT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&mut self) -> INCREMENTVAL_W {
        INCREMENTVAL_W::new(self)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&mut self) -> ADDSUBTRACT_W {
        ADDSUBTRACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register returns all zeroes when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimeradjust](index.html) module"]
pub struct TSUTIMERADJUST_SPEC;
impl crate::RegisterSpec for TSUTIMERADJUST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimeradjust::R](R) reader structure"]
impl crate::Readable for TSUTIMERADJUST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimeradjust::W](W) writer structure"]
impl crate::Writable for TSUTIMERADJUST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERADJUST to value 0"]
impl crate::Resettable for TSUTIMERADJUST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
