#[doc = "Register `WRITECOMPLETIONCTRL` reader"]
pub struct R(crate::R<WRITECOMPLETIONCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECOMPLETIONCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECOMPLETIONCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECOMPLETIONCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECOMPLETIONCTRL` writer"]
pub struct W(crate::W<WRITECOMPLETIONCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECOMPLETIONCTRL_SPEC>;
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
impl From<crate::W<WRITECOMPLETIONCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECOMPLETIONCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - Opcode"]
pub type OPCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPCODE` writer - Opcode"]
pub type OPCODE_W<'a> = crate::FieldWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, u8, u8, 8, 0>;
#[doc = "Field `POLLINGBITINDEX` reader - Polling Bit Index"]
pub type POLLINGBITINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POLLINGBITINDEX` writer - Polling Bit Index"]
pub type POLLINGBITINDEX_W<'a> =
    crate::FieldWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, u8, u8, 3, 8>;
#[doc = "Field `POLLINGPOLARITY` reader - Polling Polarity"]
pub type POLLINGPOLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLLINGPOLARITY` writer - Polling Polarity"]
pub type POLLINGPOLARITY_W<'a> = crate::BitWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, bool, 13>;
#[doc = "Field `DISABLEPOLLING` reader - Disable Polling"]
pub type DISABLEPOLLING_R = crate::BitReader<bool>;
#[doc = "Field `DISABLEPOLLING` writer - Disable Polling"]
pub type DISABLEPOLLING_W<'a> = crate::BitWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, bool, 14>;
#[doc = "Field `ENABLEPOLLINGEXP` reader - Enable Polling Expiration"]
pub type ENABLEPOLLINGEXP_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEPOLLINGEXP` writer - Enable Polling Expiration"]
pub type ENABLEPOLLINGEXP_W<'a> = crate::BitWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, bool, 15>;
#[doc = "Field `POLLCOUNT` reader - Poll Count"]
pub type POLLCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POLLCOUNT` writer - Poll Count"]
pub type POLLCOUNT_W<'a> = crate::FieldWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, u8, u8, 8, 16>;
#[doc = "Field `POLLREPDELAY` reader - Poll Repetition Delay"]
pub type POLLREPDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POLLREPDELAY` writer - Poll Repetition Delay"]
pub type POLLREPDELAY_W<'a> = crate::FieldWriter<'a, u32, WRITECOMPLETIONCTRL_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&self) -> POLLINGBITINDEX_R {
        POLLINGBITINDEX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&self) -> POLLINGPOLARITY_R {
        POLLINGPOLARITY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&self) -> DISABLEPOLLING_R {
        DISABLEPOLLING_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&self) -> ENABLEPOLLINGEXP_R {
        ENABLEPOLLINGEXP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&self) -> POLLCOUNT_R {
        POLLCOUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&self) -> POLLREPDELAY_R {
        POLLREPDELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&mut self) -> POLLINGBITINDEX_W {
        POLLINGBITINDEX_W::new(self)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&mut self) -> POLLINGPOLARITY_W {
        POLLINGPOLARITY_W::new(self)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&mut self) -> DISABLEPOLLING_W {
        DISABLEPOLLING_W::new(self)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&mut self) -> ENABLEPOLLINGEXP_W {
        ENABLEPOLLINGEXP_W::new(self)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&mut self) -> POLLCOUNT_W {
        POLLCOUNT_W::new(self)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&mut self) -> POLLREPDELAY_W {
        POLLREPDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Completion Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecompletionctrl](index.html) module"]
pub struct WRITECOMPLETIONCTRL_SPEC;
impl crate::RegisterSpec for WRITECOMPLETIONCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writecompletionctrl::R](R) reader structure"]
impl crate::Readable for WRITECOMPLETIONCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writecompletionctrl::W](W) writer structure"]
impl crate::Writable for WRITECOMPLETIONCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITECOMPLETIONCTRL to value 0x0001_0005"]
impl crate::Resettable for WRITECOMPLETIONCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0005
    }
}
