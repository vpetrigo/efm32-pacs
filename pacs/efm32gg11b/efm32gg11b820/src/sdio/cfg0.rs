#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNINGCNT` reader - Tuning Counter Value"]
pub type TUNINGCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNINGCNT` writer - Tuning Counter Value"]
pub type TUNINGCNT_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 6, 0>;
#[doc = "Field `TOUTCLKFREQ` reader - Timeout Clock Frequency"]
pub type TOUTCLKFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUTCLKFREQ` writer - Timeout Clock Frequency"]
pub type TOUTCLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 6, 6>;
#[doc = "Field `TOUTCLKUNIT` reader - Timeout Clock Unit in kHz or MHz"]
pub type TOUTCLKUNIT_R = crate::BitReader<bool>;
#[doc = "Field `TOUTCLKUNIT` writer - Timeout Clock Unit in kHz or MHz"]
pub type TOUTCLKUNIT_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 12>;
#[doc = "Field `BASECLKFREQ` reader - Base Clock Frequency for SD_CLK"]
pub type BASECLKFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASECLKFREQ` writer - Base Clock Frequency for SD_CLK"]
pub type BASECLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 8, 13>;
#[doc = "MAX Block Length of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAXBLKLEN_A {
    #[doc = "0: 512 Bytes are Selected"]
    _512B = 0,
    #[doc = "1: 1024 Bytes are Selected"]
    _1024B = 1,
    #[doc = "2: 2048 Bytes are Selected"]
    _2048B = 2,
}
impl From<MAXBLKLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXBLKLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAXBLKLEN` reader - MAX Block Length of Transfer"]
pub type MAXBLKLEN_R = crate::FieldReader<u8, MAXBLKLEN_A>;
impl MAXBLKLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAXBLKLEN_A> {
        match self.bits {
            0 => Some(MAXBLKLEN_A::_512B),
            1 => Some(MAXBLKLEN_A::_1024B),
            2 => Some(MAXBLKLEN_A::_2048B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_512B`"]
    #[inline(always)]
    pub fn is_512b(&self) -> bool {
        *self == MAXBLKLEN_A::_512B
    }
    #[doc = "Checks if the value of the field is `_1024B`"]
    #[inline(always)]
    pub fn is_1024b(&self) -> bool {
        *self == MAXBLKLEN_A::_1024B
    }
    #[doc = "Checks if the value of the field is `_2048B`"]
    #[inline(always)]
    pub fn is_2048b(&self) -> bool {
        *self == MAXBLKLEN_A::_2048B
    }
}
#[doc = "Field `MAXBLKLEN` writer - MAX Block Length of Transfer"]
pub type MAXBLKLEN_W<'a> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, MAXBLKLEN_A, 2, 21>;
impl<'a> MAXBLKLEN_W<'a> {
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn _512b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_512B)
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn _1024b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_1024B)
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn _2048b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_2048B)
    }
}
#[doc = "Field `C8BITSUP` reader - 8-bit Interface Support"]
pub type C8BITSUP_R = crate::BitReader<bool>;
#[doc = "Field `C8BITSUP` writer - 8-bit Interface Support"]
pub type C8BITSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 23>;
#[doc = "Field `CADMA2SUP` reader - ADMA2 Mode Support"]
pub type CADMA2SUP_R = crate::BitReader<bool>;
#[doc = "Field `CADMA2SUP` writer - ADMA2 Mode Support"]
pub type CADMA2SUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 24>;
#[doc = "Field `CHSSUP` reader - High Speed Mode Support"]
pub type CHSSUP_R = crate::BitReader<bool>;
#[doc = "Field `CHSSUP` writer - High Speed Mode Support"]
pub type CHSSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 25>;
#[doc = "Field `CSDMASUP` reader - SDMA Mode Support"]
pub type CSDMASUP_R = crate::BitReader<bool>;
#[doc = "Field `CSDMASUP` writer - SDMA Mode Support"]
pub type CSDMASUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 26>;
#[doc = "Field `CSUSPRESSUP` reader - Suspend/Resume Support"]
pub type CSUSPRESSUP_R = crate::BitReader<bool>;
#[doc = "Field `CSUSPRESSUP` writer - Suspend/Resume Support"]
pub type CSUSPRESSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 27>;
#[doc = "Field `C3P3VSUP` reader - Core 3P3V Support"]
pub type C3P3VSUP_R = crate::BitReader<bool>;
#[doc = "Field `C3P3VSUP` writer - Core 3P3V Support"]
pub type C3P3VSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 28>;
#[doc = "Field `C3P0VSUP` reader - 3P0V Support"]
pub type C3P0VSUP_R = crate::BitReader<bool>;
#[doc = "Field `C3P0VSUP` writer - 3P0V Support"]
pub type C3P0VSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 29>;
#[doc = "Field `C1P8VSUP` reader - 1P8V Support"]
pub type C1P8VSUP_R = crate::BitReader<bool>;
#[doc = "Field `C1P8VSUP` writer - 1P8V Support"]
pub type C1P8VSUP_W<'a> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, 30>;
impl R {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&self) -> TUNINGCNT_R {
        TUNINGCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&self) -> TOUTCLKFREQ_R {
        TOUTCLKFREQ_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&self) -> TOUTCLKUNIT_R {
        TOUTCLKUNIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&self) -> BASECLKFREQ_R {
        BASECLKFREQ_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&self) -> MAXBLKLEN_R {
        MAXBLKLEN_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&self) -> C8BITSUP_R {
        C8BITSUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&self) -> CADMA2SUP_R {
        CADMA2SUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&self) -> CHSSUP_R {
        CHSSUP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&self) -> CSDMASUP_R {
        CSDMASUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&self) -> CSUSPRESSUP_R {
        CSUSPRESSUP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&self) -> C3P3VSUP_R {
        C3P3VSUP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&self) -> C3P0VSUP_R {
        C3P0VSUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&self) -> C1P8VSUP_R {
        C1P8VSUP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&mut self) -> TUNINGCNT_W {
        TUNINGCNT_W::new(self)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&mut self) -> TOUTCLKFREQ_W {
        TOUTCLKFREQ_W::new(self)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&mut self) -> TOUTCLKUNIT_W {
        TOUTCLKUNIT_W::new(self)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&mut self) -> BASECLKFREQ_W {
        BASECLKFREQ_W::new(self)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&mut self) -> MAXBLKLEN_W {
        MAXBLKLEN_W::new(self)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&mut self) -> C8BITSUP_W {
        C8BITSUP_W::new(self)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&mut self) -> CADMA2SUP_W {
        CADMA2SUP_W::new(self)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&mut self) -> CHSSUP_W {
        CHSSUP_W::new(self)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&mut self) -> CSDMASUP_W {
        CSDMASUP_W::new(self)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&mut self) -> CSUSPRESSUP_W {
        CSUSPRESSUP_W::new(self)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&mut self) -> C3P3VSUP_W {
        C3P3VSUP_W::new(self)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&mut self) -> C3P0VSUP_W {
        C3P0VSUP_W::new(self)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&mut self) -> C1P8VSUP_W {
        C1P8VSUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
