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
#[doc = "Field `HITOF` reader - Hit Overflow Interrupt Flag"]
pub type HITOF_R = crate::BitReader<bool>;
#[doc = "Field `HITOF` writer - Hit Overflow Interrupt Flag"]
pub type HITOF_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `MISSOF` reader - Miss Overflow Interrupt Flag"]
pub type MISSOF_R = crate::BitReader<bool>;
#[doc = "Field `MISSOF` writer - Miss Overflow Interrupt Flag"]
pub type MISSOF_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `AHITOF` reader - Advanced Hit Overflow Interrupt Flag"]
pub type AHITOF_R = crate::BitReader<bool>;
#[doc = "Field `AHITOF` writer - Advanced Hit Overflow Interrupt Flag"]
pub type AHITOF_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `RAMERROR` reader - RAM error Interrupt Flag"]
pub type RAMERROR_R = crate::BitReader<bool>;
#[doc = "Field `RAMERROR` writer - RAM error Interrupt Flag"]
pub type RAMERROR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn hitof(&self) -> HITOF_R {
        HITOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn missof(&self) -> MISSOF_R {
        MISSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ahitof(&self) -> AHITOF_R {
        AHITOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerror(&self) -> RAMERROR_R {
        RAMERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn hitof(&mut self) -> HITOF_W {
        HITOF_W::new(self)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn missof(&mut self) -> MISSOF_W {
        MISSOF_W::new(self)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ahitof(&mut self) -> AHITOF_W {
        AHITOF_W::new(self)
    }
    #[doc = "Bit 8 - RAM error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerror(&mut self) -> RAMERROR_W {
        RAMERROR_W::new(self)
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
