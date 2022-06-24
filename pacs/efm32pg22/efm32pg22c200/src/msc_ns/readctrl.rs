#[doc = "Register `READCTRL` reader"]
pub struct R(crate::R<READCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READCTRL` writer"]
pub struct W(crate::W<READCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCTRL_SPEC>;
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
impl From<crate::W<READCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTBUFEN` reader - Flash dout pipeline buffer enable"]
pub type DOUTBUFEN_R = crate::BitReader<bool>;
#[doc = "Field `DOUTBUFEN` writer - Flash dout pipeline buffer enable"]
pub type DOUTBUFEN_W<'a> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, 12>;
#[doc = "Read Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    WS0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer"]
    WS1 = 1,
    #[doc = "2: Two wait-states inserted for eatch fetch or read transfer"]
    WS2 = 2,
    #[doc = "3: Three wait-states inserted for eatch fetch or read transfer"]
    WS3 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Read Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::WS0,
            1 => MODE_A::WS1,
            2 => MODE_A::WS2,
            3 => MODE_A::WS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == MODE_A::WS2
    }
    #[doc = "Checks if the value of the field is `WS3`"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == MODE_A::WS3
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type MODE_W<'a> = crate::FieldWriterSafe<'a, u32, READCTRL_SPEC, u8, MODE_A, 2, 20>;
impl<'a> MODE_W<'a> {
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(MODE_A::WS1)
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(MODE_A::WS2)
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(MODE_A::WS3)
    }
}
impl R {
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    pub fn doutbufen(&self) -> DOUTBUFEN_R {
        DOUTBUFEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    pub fn doutbufen(&mut self) -> DOUTBUFEN_W {
        DOUTBUFEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Read Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readctrl](index.html) module"]
pub struct READCTRL_SPEC;
impl crate::RegisterSpec for READCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readctrl::R](R) reader structure"]
impl crate::Readable for READCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readctrl::W](W) writer structure"]
impl crate::Writable for READCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READCTRL to value 0x0020_0000"]
impl crate::Resettable for READCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}
