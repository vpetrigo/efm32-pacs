#[doc = "Register `HC6_SPLT` reader"]
pub struct R(crate::R<HC6_SPLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC6_SPLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC6_SPLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC6_SPLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HC6_SPLT` writer"]
pub struct W(crate::W<HC6_SPLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC6_SPLT_SPEC>;
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
impl From<crate::W<HC6_SPLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC6_SPLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTADDR` reader - Port Address"]
pub type PRTADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTADDR` writer - Port Address"]
pub type PRTADDR_W<'a> = crate::FieldWriter<'a, u32, HC6_SPLT_SPEC, u8, u8, 7, 0>;
#[doc = "Field `HUBADDR` reader - Hub Address"]
pub type HUBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HUBADDR` writer - Hub Address"]
pub type HUBADDR_W<'a> = crate::FieldWriter<'a, u32, HC6_SPLT_SPEC, u8, u8, 7, 7>;
#[doc = "Field `XACTPOS` reader - Transaction Position"]
pub type XACTPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XACTPOS` writer - Transaction Position"]
pub type XACTPOS_W<'a> = crate::FieldWriter<'a, u32, HC6_SPLT_SPEC, u8, u8, 2, 14>;
#[doc = "Field `COMPSPLT` reader - Do Complete Split"]
pub type COMPSPLT_R = crate::BitReader<bool>;
#[doc = "Field `COMPSPLT` writer - Do Complete Split"]
pub type COMPSPLT_W<'a> = crate::BitWriter<'a, u32, HC6_SPLT_SPEC, bool, 16>;
#[doc = "Field `SPLTENA` reader - Split Enable"]
pub type SPLTENA_R = crate::BitReader<bool>;
#[doc = "Field `SPLTENA` writer - Split Enable"]
pub type SPLTENA_W<'a> = crate::BitWriter<'a, u32, HC6_SPLT_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&self) -> COMPSPLT_R {
        COMPSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&self) -> SPLTENA_R {
        SPLTENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W {
        PRTADDR_W::new(self)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W {
        HUBADDR_W::new(self)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W {
        XACTPOS_W::new(self)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&mut self) -> COMPSPLT_W {
        COMPSPLT_W::new(self)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&mut self) -> SPLTENA_W {
        SPLTENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel x Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_splt](index.html) module"]
pub struct HC6_SPLT_SPEC;
impl crate::RegisterSpec for HC6_SPLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc6_splt::R](R) reader structure"]
impl crate::Readable for HC6_SPLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc6_splt::W](W) writer structure"]
impl crate::Writable for HC6_SPLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HC6_SPLT to value 0"]
impl crate::Resettable for HC6_SPLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
