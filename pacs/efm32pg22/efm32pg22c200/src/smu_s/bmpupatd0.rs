#[doc = "Register `BMPUPATD0` reader"]
pub struct R(crate::R<BMPUPATD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPUPATD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPUPATD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPUPATD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMPUPATD0` writer"]
pub struct W(crate::W<BMPUPATD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMPUPATD0_SPEC>;
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
impl From<crate::W<BMPUPATD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMPUPATD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTOACC` reader - CRYPTOACC DMA privileged mode"]
pub type CRYPTOACC_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTOACC` writer - CRYPTOACC DMA privileged mode"]
pub type CRYPTOACC_W<'a> = crate::BitWriter<'a, u32, BMPUPATD0_SPEC, bool, 1>;
#[doc = "Field `LDMA` reader - MCU LDMA privileged mode"]
pub type LDMA_R = crate::BitReader<bool>;
#[doc = "Field `LDMA` writer - MCU LDMA privileged mode"]
pub type LDMA_W<'a> = crate::BitWriter<'a, u32, BMPUPATD0_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 1 - CRYPTOACC DMA privileged mode"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CRYPTOACC_R {
        CRYPTOACC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CRYPTOACC DMA privileged mode"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CRYPTOACC_W {
        CRYPTOACC_W::new(self)
    }
    #[doc = "Bit 4 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LDMA_W {
        LDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set master bits to 1 to mark as a privileged master.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmpupatd0](index.html) module"]
pub struct BMPUPATD0_SPEC;
impl crate::RegisterSpec for BMPUPATD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmpupatd0::R](R) reader structure"]
impl crate::Readable for BMPUPATD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmpupatd0::W](W) writer structure"]
impl crate::Writable for BMPUPATD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMPUPATD0 to value 0x1f"]
impl crate::Resettable for BMPUPATD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
