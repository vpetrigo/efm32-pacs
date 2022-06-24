#[doc = "Register `CTRLX` reader"]
pub struct R(crate::R<CTRLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLX` writer"]
pub struct W(crate::W<CTRLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLX_SPEC>;
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
impl From<crate::W<CTRLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGHALT_A {
    #[doc = "0: Continue to transmit until TX buffer is empty"]
    DISABLE = 0,
    #[doc = "1: Negate RTS to stop link partner's transmission during debug HALT. NOTE** The core clock should be equal to or faster than the peripheral clock; otherwise, each single step could transmit multiple frames instead of just transmitting one frame."]
    ENABLE = 1,
}
impl From<DBGHALT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGHALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DBGHALT_R = crate::BitReader<DBGHALT_A>;
impl DBGHALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGHALT_A {
        match self.bits {
            false => DBGHALT_A::DISABLE,
            true => DBGHALT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGHALT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGHALT_A::ENABLE
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DBGHALT_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, DBGHALT_A, 0>;
impl<'a> DBGHALT_W<'a> {
    #[doc = "Continue to transmit until TX buffer is empty"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGHALT_A::DISABLE)
    }
    #[doc = "Negate RTS to stop link partner's transmission during debug HALT. NOTE** The core clock should be equal to or faster than the peripheral clock; otherwise, each single step could transmit multiple frames instead of just transmitting one frame."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGHALT_A::ENABLE)
    }
}
#[doc = "CTS Pin Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSINV_A {
    #[doc = "0: The USn_CTS pin is low true"]
    DISABLE = 0,
    #[doc = "1: The USn_CTS pin is high true"]
    ENABLE = 1,
}
impl From<CTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: CTSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSINV` reader - CTS Pin Inversion"]
pub type CTSINV_R = crate::BitReader<CTSINV_A>;
impl CTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSINV_A {
        match self.bits {
            false => CTSINV_A::DISABLE,
            true => CTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSINV_A::ENABLE
    }
}
#[doc = "Field `CTSINV` writer - CTS Pin Inversion"]
pub type CTSINV_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, CTSINV_A, 1>;
impl<'a> CTSINV_W<'a> {
    #[doc = "The USn_CTS pin is low true"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSINV_A::DISABLE)
    }
    #[doc = "The USn_CTS pin is high true"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSINV_A::ENABLE)
    }
}
#[doc = "CTS Function enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: Ingore CTS"]
    DISABLE = 0,
    #[doc = "1: Stop transmitting when CTS is negated"]
    ENABLE = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS Function enabled"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE,
            true => CTSEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSEN_A::ENABLE
    }
}
#[doc = "Field `CTSEN` writer - CTS Function enabled"]
pub type CTSEN_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, CTSEN_A, 2>;
impl<'a> CTSEN_W<'a> {
    #[doc = "Ingore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE)
    }
    #[doc = "Stop transmitting when CTS is negated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE)
    }
}
#[doc = "RTS Pin Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSINV_A {
    #[doc = "0: The USn_RTS pin is low true"]
    DISABLE = 0,
    #[doc = "1: The USn_RTS pin is high true"]
    ENABLE = 1,
}
impl From<RTSINV_A> for bool {
    #[inline(always)]
    fn from(variant: RTSINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSINV` reader - RTS Pin Inversion"]
pub type RTSINV_R = crate::BitReader<RTSINV_A>;
impl RTSINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSINV_A {
        match self.bits {
            false => RTSINV_A::DISABLE,
            true => RTSINV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTSINV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTSINV_A::ENABLE
    }
}
#[doc = "Field `RTSINV` writer - RTS Pin Inversion"]
pub type RTSINV_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, RTSINV_A, 3>;
impl<'a> RTSINV_W<'a> {
    #[doc = "The USn_RTS pin is low true"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTSINV_A::DISABLE)
    }
    #[doc = "The USn_RTS pin is high true"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTSINV_A::ENABLE)
    }
}
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RXPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RXPRSEN_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, bool, 7>;
#[doc = "Field `CLKPRSEN` reader - PRS CLK Enable"]
pub type CLKPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPRSEN` writer - PRS CLK Enable"]
pub type CLKPRSEN_W<'a> = crate::BitWriter<'a, u32, CTRLX_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CTSINV_R {
        CTSINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTS Function enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RTSINV_R {
        RTSINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RXPRSEN_R {
        RXPRSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&self) -> CLKPRSEN_R {
        CLKPRSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W::new(self)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CTSINV_W {
        CTSINV_W::new(self)
    }
    #[doc = "Bit 2 - CTS Function enabled"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RTSINV_W {
        RTSINV_W::new(self)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&mut self) -> RXPRSEN_W {
        RXPRSEN_W::new(self)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&mut self) -> CLKPRSEN_W {
        CLKPRSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlx](index.html) module"]
pub struct CTRLX_SPEC;
impl crate::RegisterSpec for CTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlx::R](R) reader structure"]
impl crate::Readable for CTRLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlx::W](W) writer structure"]
impl crate::Writable for CTRLX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLX to value 0"]
impl crate::Resettable for CTRLX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
