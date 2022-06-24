#[doc = "Register `ST18_ARC` reader"]
pub struct R(crate::R<ST18_ARC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST18_ARC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST18_ARC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST18_ARC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST18_ARC` writer"]
pub struct W(crate::W<ST18_ARC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST18_ARC_SPEC>;
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
impl From<crate::W<ST18_ARC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST18_ARC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCOMP` reader - Sensor compare value"]
pub type SCOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCOMP` writer - Sensor compare value"]
pub type SCOMP_W<'a> = crate::FieldWriter<'a, u32, ST18_ARC_SPEC, u8, u8, 4, 0>;
#[doc = "Field `SMASK` reader - Sensor mask"]
pub type SMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMASK` writer - Sensor mask"]
pub type SMASK_W<'a> = crate::FieldWriter<'a, u32, ST18_ARC_SPEC, u8, u8, 4, 4>;
#[doc = "Field `CURSTATE` reader - Current State"]
pub type CURSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURSTATE` writer - Current State"]
pub type CURSTATE_W<'a> = crate::FieldWriter<'a, u32, ST18_ARC_SPEC, u8, u8, 5, 8>;
#[doc = "Configure transition action in normal mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSACT_A {
    #[doc = "0: No PRS output generated (if PRSCOUNT == 0), or do not count (if PRSCOUNT == 1)."]
    NONE = 0,
    #[doc = "1: Pulse generated on LESENSE PRS output 0 (if PRSCOUNT == 0)."]
    PRS0 = 1,
    #[doc = "1: Count Up (if PRSCOUNT == 1)."]
    UP = 1,
    #[doc = "2: Pulse generated on LESENSE PRS output 1 (if PRSCOUNT == 0)."]
    PRS1 = 2,
    #[doc = "2: Count Down (if PRSCOUNT == 1)."]
    DOWN = 2,
    #[doc = "3: Pulse generated on LESENSE PRS output 0 and 1 (if PRSCOUNT == 0)."]
    PRS01 = 3,
    #[doc = "4: Pulse generated on LESENSE PRS output 2. (PRSCOUNT == 0 OR 1)."]
    PRS2 = 4,
    #[doc = "5: Pulse generated on LESENSE PRS output 0 and 2 (if PRSCOUNT == 0)."]
    PRS02 = 5,
    #[doc = "5: Count Up and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    UPANDPRS2 = 5,
    #[doc = "6: Pulse generated on LESENSE PRS output 1 and 2 (if PRSCOUNT == 0)."]
    PRS12 = 6,
    #[doc = "6: Count Down and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    DOWNANDPRS2 = 6,
    #[doc = "7: Pulse generated on LESENSE PRS output 0, 1 and 2 (if PRSCOUNT == 0)."]
    PRS012 = 7,
}
impl From<PRSACT_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSACT` reader - Configure transition action in normal mode"]
pub type PRSACT_R = crate::FieldReader<u8, PRSACT_A>;
impl PRSACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSACT_A> {
        match self.bits {
            0 => Some(PRSACT_A::NONE),
            1 => Some(PRSACT_A::PRS0),
            1 => Some(PRSACT_A::UP),
            2 => Some(PRSACT_A::PRS1),
            2 => Some(PRSACT_A::DOWN),
            3 => Some(PRSACT_A::PRS01),
            4 => Some(PRSACT_A::PRS2),
            5 => Some(PRSACT_A::PRS02),
            5 => Some(PRSACT_A::UPANDPRS2),
            6 => Some(PRSACT_A::PRS12),
            6 => Some(PRSACT_A::DOWNANDPRS2),
            7 => Some(PRSACT_A::PRS012),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PRSACT_A::NONE
    }
    #[doc = "Checks if the value of the field is `PRS0`"]
    #[inline(always)]
    pub fn is_prs0(&self) -> bool {
        *self == PRSACT_A::PRS0
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == PRSACT_A::UP
    }
    #[doc = "Checks if the value of the field is `PRS1`"]
    #[inline(always)]
    pub fn is_prs1(&self) -> bool {
        *self == PRSACT_A::PRS1
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == PRSACT_A::DOWN
    }
    #[doc = "Checks if the value of the field is `PRS01`"]
    #[inline(always)]
    pub fn is_prs01(&self) -> bool {
        *self == PRSACT_A::PRS01
    }
    #[doc = "Checks if the value of the field is `PRS2`"]
    #[inline(always)]
    pub fn is_prs2(&self) -> bool {
        *self == PRSACT_A::PRS2
    }
    #[doc = "Checks if the value of the field is `PRS02`"]
    #[inline(always)]
    pub fn is_prs02(&self) -> bool {
        *self == PRSACT_A::PRS02
    }
    #[doc = "Checks if the value of the field is `UPANDPRS2`"]
    #[inline(always)]
    pub fn is_upandprs2(&self) -> bool {
        *self == PRSACT_A::UPANDPRS2
    }
    #[doc = "Checks if the value of the field is `PRS12`"]
    #[inline(always)]
    pub fn is_prs12(&self) -> bool {
        *self == PRSACT_A::PRS12
    }
    #[doc = "Checks if the value of the field is `DOWNANDPRS2`"]
    #[inline(always)]
    pub fn is_downandprs2(&self) -> bool {
        *self == PRSACT_A::DOWNANDPRS2
    }
    #[doc = "Checks if the value of the field is `PRS012`"]
    #[inline(always)]
    pub fn is_prs012(&self) -> bool {
        *self == PRSACT_A::PRS012
    }
}
#[doc = "Field `PRSACT` writer - Configure transition action in normal mode"]
pub type PRSACT_W<'a> = crate::FieldWriter<'a, u32, ST18_ARC_SPEC, u8, PRSACT_A, 3, 13>;
impl<'a> PRSACT_W<'a> {
    #[doc = "No PRS output generated (if PRSCOUNT == 0), or do not count (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PRSACT_A::NONE)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs0(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS0)
    }
    #[doc = "Count Up (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(PRSACT_A::UP)
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs1(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS1)
    }
    #[doc = "Count Down (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(PRSACT_A::DOWN)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 1 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs01(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS01)
    }
    #[doc = "Pulse generated on LESENSE PRS output 2. (PRSCOUNT == 0 OR 1)."]
    #[inline(always)]
    pub fn prs2(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs02(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS02)
    }
    #[doc = "Count Up and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn upandprs2(self) -> &'a mut W {
        self.variant(PRSACT_A::UPANDPRS2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs12(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS12)
    }
    #[doc = "Count Down and Pulse generated on LESENSE PRS output 2 (if PRSCOUNT == 1)."]
    #[inline(always)]
    pub fn downandprs2(self) -> &'a mut W {
        self.variant(PRSACT_A::DOWNANDPRS2)
    }
    #[doc = "Pulse generated on LESENSE PRS output 0, 1 and 2 (if PRSCOUNT == 0)."]
    #[inline(always)]
    pub fn prs012(self) -> &'a mut W {
        self.variant(PRSACT_A::PRS012)
    }
}
#[doc = "Field `NEXTSTATE` reader - Next state index"]
pub type NEXTSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXTSTATE` writer - Next state index"]
pub type NEXTSTATE_W<'a> = crate::FieldWriter<'a, u32, ST18_ARC_SPEC, u8, u8, 5, 16>;
#[doc = "Field `SETIF` reader - Set interrupt flag"]
pub type SETIF_R = crate::BitReader<bool>;
#[doc = "Field `SETIF` writer - Set interrupt flag"]
pub type SETIF_W<'a> = crate::BitWriter<'a, u32, ST18_ARC_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn scomp(&self) -> SCOMP_R {
        SCOMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn smask(&self) -> SMASK_R {
        SMASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Current State"]
    #[inline(always)]
    pub fn curstate(&self) -> CURSTATE_R {
        CURSTATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Configure transition action in normal mode"]
    #[inline(always)]
    pub fn prsact(&self) -> PRSACT_R {
        PRSACT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NEXTSTATE_R {
        NEXTSTATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor compare value"]
    #[inline(always)]
    pub fn scomp(&mut self) -> SCOMP_W {
        SCOMP_W::new(self)
    }
    #[doc = "Bits 4:7 - Sensor mask"]
    #[inline(always)]
    pub fn smask(&mut self) -> SMASK_W {
        SMASK_W::new(self)
    }
    #[doc = "Bits 8:12 - Current State"]
    #[inline(always)]
    pub fn curstate(&mut self) -> CURSTATE_W {
        CURSTATE_W::new(self)
    }
    #[doc = "Bits 13:15 - Configure transition action in normal mode"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PRSACT_W {
        PRSACT_W::new(self)
    }
    #[doc = "Bits 16:20 - Next state index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NEXTSTATE_W {
        NEXTSTATE_W::new(self)
    }
    #[doc = "Bit 21 - Set interrupt flag"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st18_arc](index.html) module"]
pub struct ST18_ARC_SPEC;
impl crate::RegisterSpec for ST18_ARC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st18_arc::R](R) reader structure"]
impl crate::Readable for ST18_ARC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st18_arc::W](W) writer structure"]
impl crate::Writable for ST18_ARC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST18_ARC to value 0"]
impl crate::Resettable for ST18_ARC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
