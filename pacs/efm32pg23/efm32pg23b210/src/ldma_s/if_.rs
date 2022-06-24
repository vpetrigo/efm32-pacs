#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE0` reader - DMA Structure Operation Done"]
pub type DONE0_R = crate::BitReader<bool>;
#[doc = "Field `DONE0` writer - DMA Structure Operation Done"]
pub type DONE0_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `DONE1` reader - DMA Structure Operation Done"]
pub type DONE1_R = crate::BitReader<bool>;
#[doc = "Field `DONE1` writer - DMA Structure Operation Done"]
pub type DONE1_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `DONE2` reader - DMA Structure Operation Done"]
pub type DONE2_R = crate::BitReader<bool>;
#[doc = "Field `DONE2` writer - DMA Structure Operation Done"]
pub type DONE2_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `DONE3` reader - DMA Structure Operation Done"]
pub type DONE3_R = crate::BitReader<bool>;
#[doc = "Field `DONE3` writer - DMA Structure Operation Done"]
pub type DONE3_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 3>;
#[doc = "Field `DONE4` reader - DMA Structure Operation Done"]
pub type DONE4_R = crate::BitReader<bool>;
#[doc = "Field `DONE4` writer - DMA Structure Operation Done"]
pub type DONE4_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 4>;
#[doc = "Field `DONE5` reader - DMA Structure Operation Done"]
pub type DONE5_R = crate::BitReader<bool>;
#[doc = "Field `DONE5` writer - DMA Structure Operation Done"]
pub type DONE5_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 5>;
#[doc = "Field `DONE6` reader - DMA Structure Operation Done"]
pub type DONE6_R = crate::BitReader<bool>;
#[doc = "Field `DONE6` writer - DMA Structure Operation Done"]
pub type DONE6_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 6>;
#[doc = "Field `DONE7` reader - DMA Structure Operation Done"]
pub type DONE7_R = crate::BitReader<bool>;
#[doc = "Field `DONE7` writer - DMA Structure Operation Done"]
pub type DONE7_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 7>;
#[doc = "Field `ERROR` reader - Error Flag"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Error Flag"]
pub type ERROR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done2(&self) -> DONE2_R {
        DONE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done3(&self) -> DONE3_R {
        DONE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done4(&self) -> DONE4_R {
        DONE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done5(&self) -> DONE5_R {
        DONE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done6(&self) -> DONE6_R {
        DONE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done7(&self) -> DONE7_R {
        DONE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done0(&mut self) -> DONE0_W {
        DONE0_W::new(self)
    }
    #[doc = "Bit 1 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done1(&mut self) -> DONE1_W {
        DONE1_W::new(self)
    }
    #[doc = "Bit 2 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done2(&mut self) -> DONE2_W {
        DONE2_W::new(self)
    }
    #[doc = "Bit 3 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done3(&mut self) -> DONE3_W {
        DONE3_W::new(self)
    }
    #[doc = "Bit 4 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done4(&mut self) -> DONE4_W {
        DONE4_W::new(self)
    }
    #[doc = "Bit 5 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done5(&mut self) -> DONE5_W {
        DONE5_W::new(self)
    }
    #[doc = "Bit 6 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done6(&mut self) -> DONE6_W {
        DONE6_W::new(self)
    }
    #[doc = "Bit 7 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done7(&mut self) -> DONE7_W {
        DONE7_W::new(self)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
