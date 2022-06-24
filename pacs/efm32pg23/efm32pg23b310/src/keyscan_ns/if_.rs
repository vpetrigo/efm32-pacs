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
#[doc = "Field `NOKEY` reader - No key was pressed"]
pub type NOKEY_R = crate::BitReader<bool>;
#[doc = "Field `NOKEY` writer - No key was pressed"]
pub type NOKEY_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `KEY` reader - A key was pressed"]
pub type KEY_R = crate::BitReader<bool>;
#[doc = "Field `KEY` writer - A key was pressed"]
pub type KEY_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `SCANNED` reader - Completed scan"]
pub type SCANNED_R = crate::BitReader<bool>;
#[doc = "Field `SCANNED` writer - Completed scan"]
pub type SCANNED_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `WAKEUP` reader - Wake up"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` writer - Wake up"]
pub type WAKEUP_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - No key was pressed"]
    #[inline(always)]
    pub fn nokey(&self) -> NOKEY_R {
        NOKEY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A key was pressed"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Completed scan"]
    #[inline(always)]
    pub fn scanned(&self) -> SCANNED_R {
        SCANNED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No key was pressed"]
    #[inline(always)]
    pub fn nokey(&mut self) -> NOKEY_W {
        NOKEY_W::new(self)
    }
    #[doc = "Bit 1 - A key was pressed"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W::new(self)
    }
    #[doc = "Bit 2 - Completed scan"]
    #[inline(always)]
    pub fn scanned(&mut self) -> SCANNED_W {
        SCANNED_W::new(self)
    }
    #[doc = "Bit 3 - Wake up"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
