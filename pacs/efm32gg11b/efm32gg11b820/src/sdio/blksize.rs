#[doc = "Register `BLKSIZE` reader"]
pub struct R(crate::R<BLKSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKSIZE` writer"]
pub struct W(crate::W<BLKSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKSIZE_SPEC>;
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
impl From<crate::W<BLKSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TFRBLKSIZE_A {
    #[doc = "0: `0`"]
    NOXFER = 0,
}
impl From<TFRBLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: TFRBLKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TFRBLKSIZE` reader - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TFRBLKSIZE_R = crate::FieldReader<u16, TFRBLKSIZE_A>;
impl TFRBLKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TFRBLKSIZE_A> {
        match self.bits {
            0 => Some(TFRBLKSIZE_A::NOXFER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOXFER`"]
    #[inline(always)]
    pub fn is_noxfer(&self) -> bool {
        *self == TFRBLKSIZE_A::NOXFER
    }
}
#[doc = "Field `TFRBLKSIZE` writer - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
pub type TFRBLKSIZE_W<'a> = crate::FieldWriter<'a, u32, BLKSIZE_SPEC, u16, TFRBLKSIZE_A, 12, 0>;
impl<'a> TFRBLKSIZE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn noxfer(self) -> &'a mut W {
        self.variant(TFRBLKSIZE_A::NOXFER)
    }
}
#[doc = "Host SDMA Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSTSDMABUFSIZE_A {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    SIZE4 = 0,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    SIZE8 = 1,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    SIZE16 = 2,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    SIZE32 = 3,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    SIZE64 = 4,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    SIZE128 = 5,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
    SIZE256 = 6,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    SIZE512 = 7,
}
impl From<HSTSDMABUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: HSTSDMABUFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HSTSDMABUFSIZE` reader - Host SDMA Buffer Size"]
pub type HSTSDMABUFSIZE_R = crate::FieldReader<u8, HSTSDMABUFSIZE_A>;
impl HSTSDMABUFSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSTSDMABUFSIZE_A {
        match self.bits {
            0 => HSTSDMABUFSIZE_A::SIZE4,
            1 => HSTSDMABUFSIZE_A::SIZE8,
            2 => HSTSDMABUFSIZE_A::SIZE16,
            3 => HSTSDMABUFSIZE_A::SIZE32,
            4 => HSTSDMABUFSIZE_A::SIZE64,
            5 => HSTSDMABUFSIZE_A::SIZE128,
            6 => HSTSDMABUFSIZE_A::SIZE256,
            7 => HSTSDMABUFSIZE_A::SIZE512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE4`"]
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE4
    }
    #[doc = "Checks if the value of the field is `SIZE8`"]
    #[inline(always)]
    pub fn is_size8(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE8
    }
    #[doc = "Checks if the value of the field is `SIZE16`"]
    #[inline(always)]
    pub fn is_size16(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE16
    }
    #[doc = "Checks if the value of the field is `SIZE32`"]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE32
    }
    #[doc = "Checks if the value of the field is `SIZE64`"]
    #[inline(always)]
    pub fn is_size64(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE64
    }
    #[doc = "Checks if the value of the field is `SIZE128`"]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE128
    }
    #[doc = "Checks if the value of the field is `SIZE256`"]
    #[inline(always)]
    pub fn is_size256(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE256
    }
    #[doc = "Checks if the value of the field is `SIZE512`"]
    #[inline(always)]
    pub fn is_size512(&self) -> bool {
        *self == HSTSDMABUFSIZE_A::SIZE512
    }
}
#[doc = "Field `HSTSDMABUFSIZE` writer - Host SDMA Buffer Size"]
pub type HSTSDMABUFSIZE_W<'a> =
    crate::FieldWriterSafe<'a, u32, BLKSIZE_SPEC, u8, HSTSDMABUFSIZE_A, 3, 12>;
impl<'a> HSTSDMABUFSIZE_W<'a> {
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn size4(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE4)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn size8(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE8)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn size16(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE16)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn size32(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE32)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn size64(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE64)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn size128(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE128)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn size256(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE256)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn size512(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZE_A::SIZE512)
    }
}
#[doc = "Blocks Count for Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BLKSCNTFORCURRTFR_A {
    #[doc = "0: `0`"]
    STOPCNT = 0,
}
impl From<BLKSCNTFORCURRTFR_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSCNTFORCURRTFR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BLKSCNTFORCURRTFR` reader - Blocks Count for Current Transfer"]
pub type BLKSCNTFORCURRTFR_R = crate::FieldReader<u16, BLKSCNTFORCURRTFR_A>;
impl BLKSCNTFORCURRTFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLKSCNTFORCURRTFR_A> {
        match self.bits {
            0 => Some(BLKSCNTFORCURRTFR_A::STOPCNT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOPCNT`"]
    #[inline(always)]
    pub fn is_stopcnt(&self) -> bool {
        *self == BLKSCNTFORCURRTFR_A::STOPCNT
    }
}
#[doc = "Field `BLKSCNTFORCURRTFR` writer - Blocks Count for Current Transfer"]
pub type BLKSCNTFORCURRTFR_W<'a> =
    crate::FieldWriter<'a, u32, BLKSIZE_SPEC, u16, BLKSCNTFORCURRTFR_A, 16, 16>;
impl<'a> BLKSCNTFORCURRTFR_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stopcnt(self) -> &'a mut W {
        self.variant(BLKSCNTFORCURRTFR_A::STOPCNT)
    }
}
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&self) -> TFRBLKSIZE_R {
        TFRBLKSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&self) -> HSTSDMABUFSIZE_R {
        HSTSDMABUFSIZE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&self) -> BLKSCNTFORCURRTFR_R {
        BLKSCNTFORCURRTFR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline(always)]
    pub fn tfrblksize(&mut self) -> TFRBLKSIZE_W {
        TFRBLKSIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline(always)]
    pub fn hstsdmabufsize(&mut self) -> HSTSDMABUFSIZE_W {
        HSTSDMABUFSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline(always)]
    pub fn blkscntforcurrtfr(&mut self) -> BLKSCNTFORCURRTFR_W {
        BLKSCNTFORCURRTFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Size and Block Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blksize](index.html) module"]
pub struct BLKSIZE_SPEC;
impl crate::RegisterSpec for BLKSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blksize::R](R) reader structure"]
impl crate::Readable for BLKSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blksize::W](W) writer structure"]
impl crate::Writable for BLKSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLKSIZE to value 0"]
impl crate::Resettable for BLKSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
