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
#[doc = "Field `FC` reader - Frame Counter"]
pub type FC_R = crate::BitReader<bool>;
#[doc = "Field `FC` writer - Frame Counter"]
pub type FC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `DISPLAY` reader - Display Update Event"]
pub type DISPLAY_R = crate::BitReader<bool>;
#[doc = "Field `DISPLAY` writer - Display Update Event"]
pub type DISPLAY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `SYNCBUSYDONE` reader - Sync Busy Done"]
pub type SYNCBUSYDONE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCBUSYDONE` writer - Sync Busy Done"]
pub type SYNCBUSYDONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Frame Counter"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Display Update Event"]
    #[inline(always)]
    pub fn display(&self) -> DISPLAY_R {
        DISPLAY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync Busy Done"]
    #[inline(always)]
    pub fn syncbusydone(&self) -> SYNCBUSYDONE_R {
        SYNCBUSYDONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Counter"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W {
        FC_W::new(self)
    }
    #[doc = "Bit 1 - Display Update Event"]
    #[inline(always)]
    pub fn display(&mut self) -> DISPLAY_W {
        DISPLAY_W::new(self)
    }
    #[doc = "Bit 2 - Sync Busy Done"]
    #[inline(always)]
    pub fn syncbusydone(&mut self) -> SYNCBUSYDONE_W {
        SYNCBUSYDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
