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
#[doc = "Field `FETCHERENDOFBLOCK` reader - End of block interrupt enable"]
pub type FETCHERENDOFBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERENDOFBLOCK` writer - End of block interrupt enable"]
pub type FETCHERENDOFBLOCK_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `FETCHERSTOPPED` reader - Stopped interrupt enable"]
pub type FETCHERSTOPPED_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERSTOPPED` writer - Stopped interrupt enable"]
pub type FETCHERSTOPPED_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `FETCHERERROR` reader - Error interrupt enable"]
pub type FETCHERERROR_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERERROR` writer - Error interrupt enable"]
pub type FETCHERERROR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `PUSHERENDOFBLOCK` reader - End of block interrupt enable"]
pub type PUSHERENDOFBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERENDOFBLOCK` writer - End of block interrupt enable"]
pub type PUSHERENDOFBLOCK_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `PUSHERSTOPPED` reader - Stopped interrupt enable"]
pub type PUSHERSTOPPED_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERSTOPPED` writer - Stopped interrupt enable"]
pub type PUSHERSTOPPED_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `PUSHERERROR` reader - Error interrupt enable"]
pub type PUSHERERROR_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERERROR` writer - Error interrupt enable"]
pub type PUSHERERROR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - End of block interrupt enable"]
    #[inline(always)]
    pub fn fetcherendofblock(&self) -> FETCHERENDOFBLOCK_R {
        FETCHERENDOFBLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stopped interrupt enable"]
    #[inline(always)]
    pub fn fetcherstopped(&self) -> FETCHERSTOPPED_R {
        FETCHERSTOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn fetchererror(&self) -> FETCHERERROR_R {
        FETCHERERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of block interrupt enable"]
    #[inline(always)]
    pub fn pusherendofblock(&self) -> PUSHERENDOFBLOCK_R {
        PUSHERENDOFBLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stopped interrupt enable"]
    #[inline(always)]
    pub fn pusherstopped(&self) -> PUSHERSTOPPED_R {
        PUSHERSTOPPED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn pushererror(&self) -> PUSHERERROR_R {
        PUSHERERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of block interrupt enable"]
    #[inline(always)]
    pub fn fetcherendofblock(&mut self) -> FETCHERENDOFBLOCK_W {
        FETCHERENDOFBLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Stopped interrupt enable"]
    #[inline(always)]
    pub fn fetcherstopped(&mut self) -> FETCHERSTOPPED_W {
        FETCHERSTOPPED_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt enable"]
    #[inline(always)]
    pub fn fetchererror(&mut self) -> FETCHERERROR_W {
        FETCHERERROR_W::new(self)
    }
    #[doc = "Bit 3 - End of block interrupt enable"]
    #[inline(always)]
    pub fn pusherendofblock(&mut self) -> PUSHERENDOFBLOCK_W {
        PUSHERENDOFBLOCK_W::new(self)
    }
    #[doc = "Bit 4 - Stopped interrupt enable"]
    #[inline(always)]
    pub fn pusherstopped(&mut self) -> PUSHERSTOPPED_W {
        PUSHERSTOPPED_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn pushererror(&mut self) -> PUSHERERROR_W {
        PUSHERERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
