#[doc = "Register `RDDATACAPTURE` reader"]
pub struct R(crate::R<RDDATACAPTURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDDATACAPTURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDDATACAPTURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDDATACAPTURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDDATACAPTURE` writer"]
pub struct W(crate::W<RDDATACAPTURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDDATACAPTURE_SPEC>;
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
impl From<crate::W<RDDATACAPTURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDDATACAPTURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass the Adapted Loopback Clock Circuit"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypass the Adapted Loopback Clock Circuit"]
pub type BYPASS_W<'a> = crate::BitWriter<'a, u32, RDDATACAPTURE_SPEC, bool, 0>;
#[doc = "Field `DELAY` reader - Read Delay"]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - Read Delay"]
pub type DELAY_W<'a> = crate::FieldWriter<'a, u32, RDDATACAPTURE_SPEC, u8, u8, 4, 1>;
#[doc = "Field `DQSENABLE` reader - DQS Enable Bit"]
pub type DQSENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DQSENABLE` writer - DQS Enable Bit"]
pub type DQSENABLE_W<'a> = crate::BitWriter<'a, u32, RDDATACAPTURE_SPEC, bool, 8>;
#[doc = "Field `DDRREADDELAY` reader - DDR Read Delay"]
pub type DDRREADDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDRREADDELAY` writer - DDR Read Delay"]
pub type DDRREADDELAY_W<'a> = crate::FieldWriter<'a, u32, RDDATACAPTURE_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&self) -> DQSENABLE_R {
        DQSENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&self) -> DDRREADDELAY_R {
        DDRREADDELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W::new(self)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&mut self) -> DQSENABLE_W {
        DQSENABLE_W::new(self)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&mut self) -> DDRREADDELAY_W {
        DDRREADDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Data Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rddatacapture](index.html) module"]
pub struct RDDATACAPTURE_SPEC;
impl crate::RegisterSpec for RDDATACAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rddatacapture::R](R) reader structure"]
impl crate::Readable for RDDATACAPTURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rddatacapture::W](W) writer structure"]
impl crate::Writable for RDDATACAPTURE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDDATACAPTURE to value 0x01"]
impl crate::Resettable for RDDATACAPTURE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
