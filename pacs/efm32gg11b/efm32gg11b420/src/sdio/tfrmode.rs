#[doc = "Register `TFRMODE` reader"]
pub struct R(crate::R<TFRMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFRMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFRMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFRMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFRMODE` writer"]
pub struct W(crate::W<TFRMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFRMODE_SPEC>;
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
impl From<crate::W<TFRMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFRMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 0>;
#[doc = "Field `BLKCNTEN` reader - Block Count Enable"]
pub type BLKCNTEN_R = crate::BitReader<bool>;
#[doc = "Field `BLKCNTEN` writer - Block Count Enable"]
pub type BLKCNTEN_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 1>;
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUTOCMDEN_A {
    #[doc = "0: Auto CMD Disabled"]
    ACMDDISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    ACMD12EN = 1,
    #[doc = "2: Auto CMD23 Enable"]
    ACMD23EN = 2,
}
impl From<AUTOCMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTOCMDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUTOCMDEN` reader - Auto Command Enable"]
pub type AUTOCMDEN_R = crate::FieldReader<u8, AUTOCMDEN_A>;
impl AUTOCMDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AUTOCMDEN_A> {
        match self.bits {
            0 => Some(AUTOCMDEN_A::ACMDDISABLED),
            1 => Some(AUTOCMDEN_A::ACMD12EN),
            2 => Some(AUTOCMDEN_A::ACMD23EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACMDDISABLED`"]
    #[inline(always)]
    pub fn is_acmddisabled(&self) -> bool {
        *self == AUTOCMDEN_A::ACMDDISABLED
    }
    #[doc = "Checks if the value of the field is `ACMD12EN`"]
    #[inline(always)]
    pub fn is_acmd12en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD12EN
    }
    #[doc = "Checks if the value of the field is `ACMD23EN`"]
    #[inline(always)]
    pub fn is_acmd23en(&self) -> bool {
        *self == AUTOCMDEN_A::ACMD23EN
    }
}
#[doc = "Field `AUTOCMDEN` writer - Auto Command Enable"]
pub type AUTOCMDEN_W<'a> = crate::FieldWriter<'a, u32, TFRMODE_SPEC, u8, AUTOCMDEN_A, 2, 2>;
impl<'a> AUTOCMDEN_W<'a> {
    #[doc = "Auto CMD Disabled"]
    #[inline(always)]
    pub fn acmddisabled(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMDDISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn acmd12en(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMD12EN)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn acmd23en(self) -> &'a mut W {
        self.variant(AUTOCMDEN_A::ACMD23EN)
    }
}
#[doc = "Field `DATDIRSEL` reader - Data Transfer Direction Select"]
pub type DATDIRSEL_R = crate::BitReader<bool>;
#[doc = "Field `DATDIRSEL` writer - Data Transfer Direction Select"]
pub type DATDIRSEL_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 4>;
#[doc = "Field `MULTSINGBLKSEL` reader - Multiple or Single Block Data Transfer Selection"]
pub type MULTSINGBLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `MULTSINGBLKSEL` writer - Multiple or Single Block Data Transfer Selection"]
pub type MULTSINGBLKSEL_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 5>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESPTYPESEL_A {
    #[doc = "0: No RESP"]
    NORESP = 0,
    #[doc = "1: RESP Length 136"]
    RESP136 = 1,
    #[doc = "2: RESP Length 48"]
    RESP48 = 2,
    #[doc = "3: RESP Length 48 check busy after RESP"]
    BUSYAFTRESP = 3,
}
impl From<RESPTYPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RESPTYPESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RESPTYPESEL` reader - Response Type Select"]
pub type RESPTYPESEL_R = crate::FieldReader<u8, RESPTYPESEL_A>;
impl RESPTYPESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPTYPESEL_A {
        match self.bits {
            0 => RESPTYPESEL_A::NORESP,
            1 => RESPTYPESEL_A::RESP136,
            2 => RESPTYPESEL_A::RESP48,
            3 => RESPTYPESEL_A::BUSYAFTRESP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORESP`"]
    #[inline(always)]
    pub fn is_noresp(&self) -> bool {
        *self == RESPTYPESEL_A::NORESP
    }
    #[doc = "Checks if the value of the field is `RESP136`"]
    #[inline(always)]
    pub fn is_resp136(&self) -> bool {
        *self == RESPTYPESEL_A::RESP136
    }
    #[doc = "Checks if the value of the field is `RESP48`"]
    #[inline(always)]
    pub fn is_resp48(&self) -> bool {
        *self == RESPTYPESEL_A::RESP48
    }
    #[doc = "Checks if the value of the field is `BUSYAFTRESP`"]
    #[inline(always)]
    pub fn is_busyaftresp(&self) -> bool {
        *self == RESPTYPESEL_A::BUSYAFTRESP
    }
}
#[doc = "Field `RESPTYPESEL` writer - Response Type Select"]
pub type RESPTYPESEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, TFRMODE_SPEC, u8, RESPTYPESEL_A, 2, 16>;
impl<'a> RESPTYPESEL_W<'a> {
    #[doc = "No RESP"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::NORESP)
    }
    #[doc = "RESP Length 136"]
    #[inline(always)]
    pub fn resp136(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::RESP136)
    }
    #[doc = "RESP Length 48"]
    #[inline(always)]
    pub fn resp48(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::RESP48)
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline(always)]
    pub fn busyaftresp(self) -> &'a mut W {
        self.variant(RESPTYPESEL_A::BUSYAFTRESP)
    }
}
#[doc = "Field `CMDCRCCHKEN` reader - Command CRC Check Enable"]
pub type CMDCRCCHKEN_R = crate::BitReader<bool>;
#[doc = "Field `CMDCRCCHKEN` writer - Command CRC Check Enable"]
pub type CMDCRCCHKEN_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 19>;
#[doc = "Field `CMDINDXCHKEN` reader - Command Index Check Enable"]
pub type CMDINDXCHKEN_R = crate::BitReader<bool>;
#[doc = "Field `CMDINDXCHKEN` writer - Command Index Check Enable"]
pub type CMDINDXCHKEN_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 20>;
#[doc = "Field `DATPRESSEL` reader - Data Present Select"]
pub type DATPRESSEL_R = crate::BitReader<bool>;
#[doc = "Field `DATPRESSEL` writer - Data Present Select"]
pub type DATPRESSEL_W<'a> = crate::BitWriter<'a, u32, TFRMODE_SPEC, bool, 21>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDTYPE_A {
    #[doc = "0: Normal Command"]
    NORMAL = 0,
    #[doc = "1: Suspend command"]
    SUSPEND = 1,
    #[doc = "2: Resume command"]
    RESUME = 2,
    #[doc = "3: Abort command"]
    ABORT = 3,
}
impl From<CMDTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMDTYPE` reader - Command Type"]
pub type CMDTYPE_R = crate::FieldReader<u8, CMDTYPE_A>;
impl CMDTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYPE_A {
        match self.bits {
            0 => CMDTYPE_A::NORMAL,
            1 => CMDTYPE_A::SUSPEND,
            2 => CMDTYPE_A::RESUME,
            3 => CMDTYPE_A::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPE_A::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPE_A::ABORT
    }
}
#[doc = "Field `CMDTYPE` writer - Command Type"]
pub type CMDTYPE_W<'a> = crate::FieldWriterSafe<'a, u32, TFRMODE_SPEC, u8, CMDTYPE_A, 2, 22>;
impl<'a> CMDTYPE_W<'a> {
    #[doc = "Normal Command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPE_A::NORMAL)
    }
    #[doc = "Suspend command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDTYPE_A::SUSPEND)
    }
    #[doc = "Resume command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDTYPE_A::RESUME)
    }
    #[doc = "Abort command"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDTYPE_A::ABORT)
    }
}
#[doc = "Field `CMDINDEX` reader - Command Index"]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - Command Index"]
pub type CMDINDEX_W<'a> = crate::FieldWriter<'a, u32, TFRMODE_SPEC, u8, u8, 6, 24>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&self) -> BLKCNTEN_R {
        BLKCNTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&self) -> AUTOCMDEN_R {
        AUTOCMDEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&self) -> DATDIRSEL_R {
        DATDIRSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&self) -> MULTSINGBLKSEL_R {
        MULTSINGBLKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> RESPTYPESEL_R {
        RESPTYPESEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&self) -> CMDCRCCHKEN_R {
        CMDCRCCHKEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&self) -> CMDINDXCHKEN_R {
        CMDINDXCHKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&self) -> DATPRESSEL_R {
        DATPRESSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CMDTYPE_R {
        CMDTYPE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blkcnten(&mut self) -> BLKCNTEN_W {
        BLKCNTEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn autocmden(&mut self) -> AUTOCMDEN_W {
        AUTOCMDEN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn datdirsel(&mut self) -> DATDIRSEL_W {
        DATDIRSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline(always)]
    pub fn multsingblksel(&mut self) -> MULTSINGBLKSEL_W {
        MULTSINGBLKSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&mut self) -> RESPTYPESEL_W {
        RESPTYPESEL_W::new(self)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cmdcrcchken(&mut self) -> CMDCRCCHKEN_W {
        CMDCRCCHKEN_W::new(self)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cmdindxchken(&mut self) -> CMDINDXCHKEN_W {
        CMDINDXCHKEN_W::new(self)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn datpressel(&mut self) -> DATPRESSEL_W {
        DATPRESSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CMDTYPE_W {
        CMDTYPE_W::new(self)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W {
        CMDINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Mode and Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfrmode](index.html) module"]
pub struct TFRMODE_SPEC;
impl crate::RegisterSpec for TFRMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfrmode::R](R) reader structure"]
impl crate::Readable for TFRMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfrmode::W](W) writer structure"]
impl crate::Writable for TFRMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TFRMODE to value 0"]
impl crate::Resettable for TFRMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
