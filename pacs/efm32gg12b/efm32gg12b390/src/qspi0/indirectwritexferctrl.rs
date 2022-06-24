#[doc = "Register `INDIRECTWRITEXFERCTRL` reader"]
pub struct R(crate::R<INDIRECTWRITEXFERCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTWRITEXFERCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTWRITEXFERCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTWRITEXFERCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTWRITEXFERCTRL` writer"]
pub struct W(crate::W<INDIRECTWRITEXFERCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTWRITEXFERCTRL_SPEC>;
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
impl From<crate::W<INDIRECTWRITEXFERCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTWRITEXFERCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Start Indirect Write"]
pub type START_W<'a> = crate::BitWriter<'a, u32, INDIRECTWRITEXFERCTRL_SPEC, bool, 0>;
#[doc = "Field `CANCEL` writer - Cancel Indirect Write"]
pub type CANCEL_W<'a> = crate::BitWriter<'a, u32, INDIRECTWRITEXFERCTRL_SPEC, bool, 1>;
#[doc = "Field `WRSTATUS` reader - Indirect Write Status"]
pub type WRSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `WRQUEUED` reader - Two Indirect Write Operations Have Been Queued"]
pub type WRQUEUED_R = crate::BitReader<bool>;
#[doc = "Field `INDOPSDONESTATUS` reader - Indirect Completion Status"]
pub type INDOPSDONESTATUS_R = crate::BitReader<bool>;
#[doc = "Field `INDOPSDONESTATUS` writer - Indirect Completion Status"]
pub type INDOPSDONESTATUS_W<'a> = crate::BitWriter<'a, u32, INDIRECTWRITEXFERCTRL_SPEC, bool, 5>;
#[doc = "Field `NUMINDOPSDONE` reader - Indirect Operations Done"]
pub type NUMINDOPSDONE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 2 - Indirect Write Status"]
    #[inline(always)]
    pub fn wrstatus(&self) -> WRSTATUS_R {
        WRSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Two Indirect Write Operations Have Been Queued"]
    #[inline(always)]
    pub fn wrqueued(&self) -> WRQUEUED_R {
        WRQUEUED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> INDOPSDONESTATUS_R {
        INDOPSDONESTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NUMINDOPSDONE_R {
        NUMINDOPSDONE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Write"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Cancel Indirect Write"]
    #[inline(always)]
    pub fn cancel(&mut self) -> CANCEL_W {
        CANCEL_W::new(self)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&mut self) -> INDOPSDONESTATUS_W {
        INDOPSDONESTATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Write Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferctrl](index.html) module"]
pub struct INDIRECTWRITEXFERCTRL_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectwritexferctrl::R](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectwritexferctrl::W](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERCTRL to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
