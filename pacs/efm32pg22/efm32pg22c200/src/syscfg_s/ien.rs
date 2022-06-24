#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW0` reader - Software interrupt 0"]
pub type SW0_R = crate::BitReader<bool>;
#[doc = "Field `SW0` writer - Software interrupt 0"]
pub type SW0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `SW1` reader - Software interrupt 1"]
pub type SW1_R = crate::BitReader<bool>;
#[doc = "Field `SW1` writer - Software interrupt 1"]
pub type SW1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `SW2` reader - Software interrupt 2"]
pub type SW2_R = crate::BitReader<bool>;
#[doc = "Field `SW2` writer - Software interrupt 2"]
pub type SW2_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `SW3` reader - Software interrupt 3"]
pub type SW3_R = crate::BitReader<bool>;
#[doc = "Field `SW3` writer - Software interrupt 3"]
pub type SW3_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `RAMERR1B` reader - RAM 1-bit Error Interrupt Enable"]
pub type RAMERR1B_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR1B` writer - RAM 1-bit Error Interrupt Enable"]
pub type RAMERR1B_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `RAMERR2B` reader - RAM 2-bit Error Interrupt Enable"]
pub type RAMERR2B_R = crate::BitReader<bool>;
#[doc = "Field `RAMERR2B` writer - RAM 2-bit Error Interrupt Enable"]
pub type RAMERR2B_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - Software interrupt 0"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt 1"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt 2"]
    #[inline(always)]
    pub fn sw2(&self) -> SW2_R {
        SW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt 3"]
    #[inline(always)]
    pub fn sw3(&self) -> SW3_R {
        SW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM 1-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> RAMERR1B_R {
        RAMERR1B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM 2-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> RAMERR2B_R {
        RAMERR2B_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt 0"]
    #[inline(always)]
    pub fn sw0(&mut self) -> SW0_W {
        SW0_W::new(self)
    }
    #[doc = "Bit 1 - Software interrupt 1"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W {
        SW1_W::new(self)
    }
    #[doc = "Bit 2 - Software interrupt 2"]
    #[inline(always)]
    pub fn sw2(&mut self) -> SW2_W {
        SW2_W::new(self)
    }
    #[doc = "Bit 3 - Software interrupt 3"]
    #[inline(always)]
    pub fn sw3(&mut self) -> SW3_W {
        SW3_W::new(self)
    }
    #[doc = "Bit 16 - RAM 1-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> RAMERR1B_W {
        RAMERR1B_W::new(self)
    }
    #[doc = "Bit 17 - RAM 2-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> RAMERR2B_W {
        RAMERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to enable interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
