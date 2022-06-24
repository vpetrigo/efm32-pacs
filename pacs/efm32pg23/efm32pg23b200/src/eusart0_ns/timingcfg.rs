#[doc = "Register `TIMINGCFG` reader"]
pub struct R(crate::R<TIMINGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGCFG` writer"]
pub struct W(crate::W<TIMINGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGCFG_SPEC>;
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
impl From<crate::W<TIMINGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXDELAY_A {
    #[doc = "0: Frames are transmitted immediately."]
    NONE = 0,
    #[doc = "1: Transmission of new frames is delayed by a single bit period."]
    SINGLE = 1,
    #[doc = "2: Transmission of new frames is delayed by a two bit periods."]
    DOUBLE = 2,
    #[doc = "3: Transmission of new frames is delayed by a three bit periods."]
    TRIPPLE = 3,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TXDELAY_R = crate::FieldReader<u8, TXDELAY_A>;
impl TXDELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::NONE,
            1 => TXDELAY_A::SINGLE,
            2 => TXDELAY_A::DOUBLE,
            3 => TXDELAY_A::TRIPPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXDELAY_A::NONE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDELAY_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == TXDELAY_A::DOUBLE
    }
    #[doc = "Checks if the value of the field is `TRIPPLE`"]
    #[inline(always)]
    pub fn is_tripple(&self) -> bool {
        *self == TXDELAY_A::TRIPPLE
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TXDELAY_W<'a> = crate::FieldWriterSafe<'a, u32, TIMINGCFG_SPEC, u8, TXDELAY_A, 2, 0>;
impl<'a> TXDELAY_W<'a> {
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDELAY_A::NONE)
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDELAY_A::SINGLE)
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(TXDELAY_A::DOUBLE)
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn tripple(self) -> &'a mut W {
        self.variant(TXDELAY_A::TRIPPLE)
    }
}
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSSETUP_A {
    #[doc = "0: CS is asserted half or 1 baud-time before the start of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    ZERO = 0,
    #[doc = "1: CS is asserted 1 additional baud-time before start of transmission"]
    ONE = 1,
    #[doc = "2: CS is asserted 2 additional baud-times before start of transmission"]
    TWO = 2,
    #[doc = "3: CS is asserted 3 additional baud-times before start of transmission"]
    THREE = 3,
    #[doc = "4: CS is asserted 4 additional baud-times before start of transmission"]
    FOUR = 4,
    #[doc = "5: CS is asserted 5 additional baud-times before start of transmission"]
    FIVE = 5,
    #[doc = "6: CS is asserted 6 additional baud-times before start of transmission"]
    SIX = 6,
    #[doc = "7: CS is asserted 7 additional baud-times before start of transmission"]
    SEVEN = 7,
}
impl From<CSSETUP_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSETUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSSETUP` reader - Chip Select Setup"]
pub type CSSETUP_R = crate::FieldReader<u8, CSSETUP_A>;
impl CSSETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSETUP_A {
        match self.bits {
            0 => CSSETUP_A::ZERO,
            1 => CSSETUP_A::ONE,
            2 => CSSETUP_A::TWO,
            3 => CSSETUP_A::THREE,
            4 => CSSETUP_A::FOUR,
            5 => CSSETUP_A::FIVE,
            6 => CSSETUP_A::SIX,
            7 => CSSETUP_A::SEVEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CSSETUP_A::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == CSSETUP_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == CSSETUP_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP_A::SEVEN
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CSSETUP_W<'a> = crate::FieldWriterSafe<'a, u32, TIMINGCFG_SPEC, u8, CSSETUP_A, 3, 4>;
impl<'a> CSSETUP_W<'a> {
    #[doc = "CS is asserted half or 1 baud-time before the start of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSSETUP_A::ZERO)
    }
    #[doc = "CS is asserted 1 additional baud-time before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSSETUP_A::ONE)
    }
    #[doc = "CS is asserted 2 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSSETUP_A::TWO)
    }
    #[doc = "CS is asserted 3 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSSETUP_A::THREE)
    }
    #[doc = "CS is asserted 4 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(CSSETUP_A::FOUR)
    }
    #[doc = "CS is asserted 5 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(CSSETUP_A::FIVE)
    }
    #[doc = "CS is asserted 6 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(CSSETUP_A::SIX)
    }
    #[doc = "CS is asserted 7 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSSETUP_A::SEVEN)
    }
}
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSHOLD_A {
    #[doc = "0: CS is de-asserted half or 1 baud-time after the end of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    ZERO = 0,
    #[doc = "1: CS is de-asserted 1 additional baud-time after the end of transmission"]
    ONE = 1,
    #[doc = "2: CS is de-asserted 2 additional baud-times after the end of transmission"]
    TWO = 2,
    #[doc = "3: CS is de-asserted 3 additional baud-times after the end of transmission"]
    THREE = 3,
    #[doc = "4: CS is de-asserted 4 additional baud-times after the end of transmission"]
    FOUR = 4,
    #[doc = "5: CS is de-asserted 5 additional baud-times after the end of transmission"]
    FIVE = 5,
    #[doc = "6: CS is de-asserted 6 additional baud-times after the end of transmission"]
    SIX = 6,
    #[doc = "7: CS is de-asserted 7 additional baud-times after the end of transmission"]
    SEVEN = 7,
}
impl From<CSHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CSHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSHOLD` reader - Chip Select Hold"]
pub type CSHOLD_R = crate::FieldReader<u8, CSHOLD_A>;
impl CSHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSHOLD_A {
        match self.bits {
            0 => CSHOLD_A::ZERO,
            1 => CSHOLD_A::ONE,
            2 => CSHOLD_A::TWO,
            3 => CSHOLD_A::THREE,
            4 => CSHOLD_A::FOUR,
            5 => CSHOLD_A::FIVE,
            6 => CSHOLD_A::SIX,
            7 => CSHOLD_A::SEVEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CSHOLD_A::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == CSHOLD_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == CSHOLD_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD_A::SEVEN
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CSHOLD_W<'a> = crate::FieldWriterSafe<'a, u32, TIMINGCFG_SPEC, u8, CSHOLD_A, 3, 8>;
impl<'a> CSHOLD_W<'a> {
    #[doc = "CS is de-asserted half or 1 baud-time after the end of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSHOLD_A::ZERO)
    }
    #[doc = "CS is de-asserted 1 additional baud-time after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSHOLD_A::ONE)
    }
    #[doc = "CS is de-asserted 2 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSHOLD_A::TWO)
    }
    #[doc = "CS is de-asserted 3 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSHOLD_A::THREE)
    }
    #[doc = "CS is de-asserted 4 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(CSHOLD_A::FOUR)
    }
    #[doc = "CS is de-asserted 5 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(CSHOLD_A::FIVE)
    }
    #[doc = "CS is de-asserted 6 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(CSHOLD_A::SIX)
    }
    #[doc = "CS is de-asserted 7 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSHOLD_A::SEVEN)
    }
}
#[doc = "Inter-Character Spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICS_A {
    #[doc = "0: There is no space between charcters"]
    ZERO = 0,
    #[doc = "1: Create a space of 1 baud-times between frames"]
    ONE = 1,
    #[doc = "2: Create a space of 2 baud-times between frames"]
    TWO = 2,
    #[doc = "3: Create a space of 3 baud-times between frames"]
    THREE = 3,
    #[doc = "4: Create a space of 4 baud-times between frames"]
    FOUR = 4,
    #[doc = "5: Create a space of 5 baud-times between frames"]
    FIVE = 5,
    #[doc = "6: Create a space of 6 baud-times between frames"]
    SIX = 6,
    #[doc = "7: Create a space of 7 baud-times between frames"]
    SEVEN = 7,
}
impl From<ICS_A> for u8 {
    #[inline(always)]
    fn from(variant: ICS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ICS` reader - Inter-Character Spacing"]
pub type ICS_R = crate::FieldReader<u8, ICS_A>;
impl ICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICS_A {
        match self.bits {
            0 => ICS_A::ZERO,
            1 => ICS_A::ONE,
            2 => ICS_A::TWO,
            3 => ICS_A::THREE,
            4 => ICS_A::FOUR,
            5 => ICS_A::FIVE,
            6 => ICS_A::SIX,
            7 => ICS_A::SEVEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ICS_A::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == ICS_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == ICS_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS_A::SEVEN
    }
}
#[doc = "Field `ICS` writer - Inter-Character Spacing"]
pub type ICS_W<'a> = crate::FieldWriterSafe<'a, u32, TIMINGCFG_SPEC, u8, ICS_A, 3, 12>;
impl<'a> ICS_W<'a> {
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICS_A::ZERO)
    }
    #[doc = "Create a space of 1 baud-times between frames"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ICS_A::ONE)
    }
    #[doc = "Create a space of 2 baud-times between frames"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(ICS_A::TWO)
    }
    #[doc = "Create a space of 3 baud-times between frames"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(ICS_A::THREE)
    }
    #[doc = "Create a space of 4 baud-times between frames"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(ICS_A::FOUR)
    }
    #[doc = "Create a space of 5 baud-times between frames"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(ICS_A::FIVE)
    }
    #[doc = "Create a space of 6 baud-times between frames"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(ICS_A::SIX)
    }
    #[doc = "Create a space of 7 baud-times between frames"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(ICS_A::SEVEN)
    }
}
#[doc = "Field `SETUPWINDOW` reader - Setup Window"]
pub type SETUPWINDOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETUPWINDOW` writer - Setup Window"]
pub type SETUPWINDOW_W<'a> = crate::FieldWriter<'a, u32, TIMINGCFG_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CSSETUP_R {
        CSSETUP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CSHOLD_R {
        CSHOLD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn ics(&self) -> ICS_R {
        ICS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Setup Window"]
    #[inline(always)]
    pub fn setupwindow(&self) -> SETUPWINDOW_R {
        SETUPWINDOW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TXDELAY_W {
        TXDELAY_W::new(self)
    }
    #[doc = "Bits 4:6 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&mut self) -> CSSETUP_W {
        CSSETUP_W::new(self)
    }
    #[doc = "Bits 8:10 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&mut self) -> CSHOLD_W {
        CSHOLD_W::new(self)
    }
    #[doc = "Bits 12:14 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn ics(&mut self) -> ICS_W {
        ICS_W::new(self)
    }
    #[doc = "Bits 16:19 - Setup Window"]
    #[inline(always)]
    pub fn setupwindow(&mut self) -> SETUPWINDOW_W {
        SETUPWINDOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timingcfg](index.html) module"]
pub struct TIMINGCFG_SPEC;
impl crate::RegisterSpec for TIMINGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timingcfg::R](R) reader structure"]
impl crate::Readable for TIMINGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timingcfg::W](W) writer structure"]
impl crate::Writable for TIMINGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGCFG to value 0x0005_0000"]
impl crate::Resettable for TIMINGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0000
    }
}
