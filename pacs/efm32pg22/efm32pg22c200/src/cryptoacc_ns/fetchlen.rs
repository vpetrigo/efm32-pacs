#[doc = "Register `FETCHLEN` reader"]
pub struct R(crate::R<FETCHLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHLEN` writer"]
pub struct W(crate::W<FETCHLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHLEN_SPEC>;
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
impl From<crate::W<FETCHLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTH` reader - Length of data block"]
pub type LENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LENGTH` writer - Length of data block"]
pub type LENGTH_W<'a> = crate::FieldWriter<'a, u32, FETCHLEN_SPEC, u32, u32, 28, 0>;
#[doc = "Field `CONSTADDR` reader - Constant address"]
pub type CONSTADDR_R = crate::BitReader<bool>;
#[doc = "Field `CONSTADDR` writer - Constant address"]
pub type CONSTADDR_W<'a> = crate::BitWriter<'a, u32, FETCHLEN_SPEC, bool, 28>;
#[doc = "Field `REALIGN` reader - Realign length"]
pub type REALIGN_R = crate::BitReader<bool>;
#[doc = "Field `REALIGN` writer - Realign length"]
pub type REALIGN_W<'a> = crate::BitWriter<'a, u32, FETCHLEN_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:27 - Length of data block"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 28 - Constant address"]
    #[inline(always)]
    pub fn constaddr(&self) -> CONSTADDR_R {
        CONSTADDR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Realign length"]
    #[inline(always)]
    pub fn realign(&self) -> REALIGN_R {
        REALIGN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Length of data block"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W::new(self)
    }
    #[doc = "Bit 28 - Constant address"]
    #[inline(always)]
    pub fn constaddr(&mut self) -> CONSTADDR_W {
        CONSTADDR_W::new(self)
    }
    #[doc = "Bit 29 - Realign length"]
    #[inline(always)]
    pub fn realign(&mut self) -> REALIGN_W {
        REALIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fetcher: Length of data block. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchlen](index.html) module"]
pub struct FETCHLEN_SPEC;
impl crate::RegisterSpec for FETCHLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchlen::R](R) reader structure"]
impl crate::Readable for FETCHLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchlen::W](W) writer structure"]
impl crate::Writable for FETCHLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FETCHLEN to value 0"]
impl crate::Resettable for FETCHLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
