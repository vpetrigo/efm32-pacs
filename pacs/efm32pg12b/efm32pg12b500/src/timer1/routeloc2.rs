#[doc = "Register `ROUTELOC2` reader"]
pub struct R(crate::R<ROUTELOC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC2` writer"]
pub struct W(crate::W<ROUTELOC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC2_SPEC>;
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
impl From<crate::W<ROUTELOC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDTI0LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
    #[doc = "17: Location 17"]
    LOC17 = 17,
    #[doc = "18: Location 18"]
    LOC18 = 18,
    #[doc = "19: Location 19"]
    LOC19 = 19,
    #[doc = "20: Location 20"]
    LOC20 = 20,
    #[doc = "21: Location 21"]
    LOC21 = 21,
    #[doc = "22: Location 22"]
    LOC22 = 22,
    #[doc = "23: Location 23"]
    LOC23 = 23,
    #[doc = "24: Location 24"]
    LOC24 = 24,
    #[doc = "25: Location 25"]
    LOC25 = 25,
    #[doc = "26: Location 26"]
    LOC26 = 26,
    #[doc = "27: Location 27"]
    LOC27 = 27,
    #[doc = "28: Location 28"]
    LOC28 = 28,
    #[doc = "29: Location 29"]
    LOC29 = 29,
    #[doc = "30: Location 30"]
    LOC30 = 30,
    #[doc = "31: Location 31"]
    LOC31 = 31,
}
impl From<CDTI0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI0LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDTI0LOC` reader - I/O Location"]
pub type CDTI0LOC_R = crate::FieldReader<u8, CDTI0LOC_A>;
impl CDTI0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI0LOC_A> {
        match self.bits {
            0 => Some(CDTI0LOC_A::LOC0),
            1 => Some(CDTI0LOC_A::LOC1),
            2 => Some(CDTI0LOC_A::LOC2),
            3 => Some(CDTI0LOC_A::LOC3),
            4 => Some(CDTI0LOC_A::LOC4),
            5 => Some(CDTI0LOC_A::LOC5),
            6 => Some(CDTI0LOC_A::LOC6),
            7 => Some(CDTI0LOC_A::LOC7),
            8 => Some(CDTI0LOC_A::LOC8),
            9 => Some(CDTI0LOC_A::LOC9),
            10 => Some(CDTI0LOC_A::LOC10),
            11 => Some(CDTI0LOC_A::LOC11),
            12 => Some(CDTI0LOC_A::LOC12),
            13 => Some(CDTI0LOC_A::LOC13),
            14 => Some(CDTI0LOC_A::LOC14),
            15 => Some(CDTI0LOC_A::LOC15),
            16 => Some(CDTI0LOC_A::LOC16),
            17 => Some(CDTI0LOC_A::LOC17),
            18 => Some(CDTI0LOC_A::LOC18),
            19 => Some(CDTI0LOC_A::LOC19),
            20 => Some(CDTI0LOC_A::LOC20),
            21 => Some(CDTI0LOC_A::LOC21),
            22 => Some(CDTI0LOC_A::LOC22),
            23 => Some(CDTI0LOC_A::LOC23),
            24 => Some(CDTI0LOC_A::LOC24),
            25 => Some(CDTI0LOC_A::LOC25),
            26 => Some(CDTI0LOC_A::LOC26),
            27 => Some(CDTI0LOC_A::LOC27),
            28 => Some(CDTI0LOC_A::LOC28),
            29 => Some(CDTI0LOC_A::LOC29),
            30 => Some(CDTI0LOC_A::LOC30),
            31 => Some(CDTI0LOC_A::LOC31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI0LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI0LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI0LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI0LOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI0LOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI0LOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI0LOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI0LOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI0LOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI0LOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI0LOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI0LOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI0LOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI0LOC_A::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI0LOC_A::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI0LOC_A::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI0LOC_A::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI0LOC_A::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI0LOC_A::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI0LOC_A::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI0LOC_A::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI0LOC_A::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI0LOC_A::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI0LOC_A::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI0LOC_A::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI0LOC_A::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI0LOC_A::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI0LOC_A::LOC31
    }
}
#[doc = "Field `CDTI0LOC` writer - I/O Location"]
pub type CDTI0LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI0LOC_A, 6, 0>;
impl<'a> CDTI0LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDTI1LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
    #[doc = "17: Location 17"]
    LOC17 = 17,
    #[doc = "18: Location 18"]
    LOC18 = 18,
    #[doc = "19: Location 19"]
    LOC19 = 19,
    #[doc = "20: Location 20"]
    LOC20 = 20,
    #[doc = "21: Location 21"]
    LOC21 = 21,
    #[doc = "22: Location 22"]
    LOC22 = 22,
    #[doc = "23: Location 23"]
    LOC23 = 23,
    #[doc = "24: Location 24"]
    LOC24 = 24,
    #[doc = "25: Location 25"]
    LOC25 = 25,
    #[doc = "26: Location 26"]
    LOC26 = 26,
    #[doc = "27: Location 27"]
    LOC27 = 27,
    #[doc = "28: Location 28"]
    LOC28 = 28,
    #[doc = "29: Location 29"]
    LOC29 = 29,
    #[doc = "30: Location 30"]
    LOC30 = 30,
    #[doc = "31: Location 31"]
    LOC31 = 31,
}
impl From<CDTI1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI1LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDTI1LOC` reader - I/O Location"]
pub type CDTI1LOC_R = crate::FieldReader<u8, CDTI1LOC_A>;
impl CDTI1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI1LOC_A> {
        match self.bits {
            0 => Some(CDTI1LOC_A::LOC0),
            1 => Some(CDTI1LOC_A::LOC1),
            2 => Some(CDTI1LOC_A::LOC2),
            3 => Some(CDTI1LOC_A::LOC3),
            4 => Some(CDTI1LOC_A::LOC4),
            5 => Some(CDTI1LOC_A::LOC5),
            6 => Some(CDTI1LOC_A::LOC6),
            7 => Some(CDTI1LOC_A::LOC7),
            8 => Some(CDTI1LOC_A::LOC8),
            9 => Some(CDTI1LOC_A::LOC9),
            10 => Some(CDTI1LOC_A::LOC10),
            11 => Some(CDTI1LOC_A::LOC11),
            12 => Some(CDTI1LOC_A::LOC12),
            13 => Some(CDTI1LOC_A::LOC13),
            14 => Some(CDTI1LOC_A::LOC14),
            15 => Some(CDTI1LOC_A::LOC15),
            16 => Some(CDTI1LOC_A::LOC16),
            17 => Some(CDTI1LOC_A::LOC17),
            18 => Some(CDTI1LOC_A::LOC18),
            19 => Some(CDTI1LOC_A::LOC19),
            20 => Some(CDTI1LOC_A::LOC20),
            21 => Some(CDTI1LOC_A::LOC21),
            22 => Some(CDTI1LOC_A::LOC22),
            23 => Some(CDTI1LOC_A::LOC23),
            24 => Some(CDTI1LOC_A::LOC24),
            25 => Some(CDTI1LOC_A::LOC25),
            26 => Some(CDTI1LOC_A::LOC26),
            27 => Some(CDTI1LOC_A::LOC27),
            28 => Some(CDTI1LOC_A::LOC28),
            29 => Some(CDTI1LOC_A::LOC29),
            30 => Some(CDTI1LOC_A::LOC30),
            31 => Some(CDTI1LOC_A::LOC31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI1LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI1LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI1LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI1LOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI1LOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI1LOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI1LOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI1LOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI1LOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI1LOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI1LOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI1LOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI1LOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI1LOC_A::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI1LOC_A::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI1LOC_A::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI1LOC_A::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI1LOC_A::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI1LOC_A::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI1LOC_A::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI1LOC_A::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI1LOC_A::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI1LOC_A::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI1LOC_A::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI1LOC_A::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI1LOC_A::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI1LOC_A::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI1LOC_A::LOC31
    }
}
#[doc = "Field `CDTI1LOC` writer - I/O Location"]
pub type CDTI1LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI1LOC_A, 6, 8>;
impl<'a> CDTI1LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDTI2LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
    #[doc = "8: Location 8"]
    LOC8 = 8,
    #[doc = "9: Location 9"]
    LOC9 = 9,
    #[doc = "10: Location 10"]
    LOC10 = 10,
    #[doc = "11: Location 11"]
    LOC11 = 11,
    #[doc = "12: Location 12"]
    LOC12 = 12,
    #[doc = "13: Location 13"]
    LOC13 = 13,
    #[doc = "14: Location 14"]
    LOC14 = 14,
    #[doc = "15: Location 15"]
    LOC15 = 15,
    #[doc = "16: Location 16"]
    LOC16 = 16,
    #[doc = "17: Location 17"]
    LOC17 = 17,
    #[doc = "18: Location 18"]
    LOC18 = 18,
    #[doc = "19: Location 19"]
    LOC19 = 19,
    #[doc = "20: Location 20"]
    LOC20 = 20,
    #[doc = "21: Location 21"]
    LOC21 = 21,
    #[doc = "22: Location 22"]
    LOC22 = 22,
    #[doc = "23: Location 23"]
    LOC23 = 23,
    #[doc = "24: Location 24"]
    LOC24 = 24,
    #[doc = "25: Location 25"]
    LOC25 = 25,
    #[doc = "26: Location 26"]
    LOC26 = 26,
    #[doc = "27: Location 27"]
    LOC27 = 27,
    #[doc = "28: Location 28"]
    LOC28 = 28,
    #[doc = "29: Location 29"]
    LOC29 = 29,
    #[doc = "30: Location 30"]
    LOC30 = 30,
    #[doc = "31: Location 31"]
    LOC31 = 31,
}
impl From<CDTI2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI2LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDTI2LOC` reader - I/O Location"]
pub type CDTI2LOC_R = crate::FieldReader<u8, CDTI2LOC_A>;
impl CDTI2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI2LOC_A> {
        match self.bits {
            0 => Some(CDTI2LOC_A::LOC0),
            1 => Some(CDTI2LOC_A::LOC1),
            2 => Some(CDTI2LOC_A::LOC2),
            3 => Some(CDTI2LOC_A::LOC3),
            4 => Some(CDTI2LOC_A::LOC4),
            5 => Some(CDTI2LOC_A::LOC5),
            6 => Some(CDTI2LOC_A::LOC6),
            7 => Some(CDTI2LOC_A::LOC7),
            8 => Some(CDTI2LOC_A::LOC8),
            9 => Some(CDTI2LOC_A::LOC9),
            10 => Some(CDTI2LOC_A::LOC10),
            11 => Some(CDTI2LOC_A::LOC11),
            12 => Some(CDTI2LOC_A::LOC12),
            13 => Some(CDTI2LOC_A::LOC13),
            14 => Some(CDTI2LOC_A::LOC14),
            15 => Some(CDTI2LOC_A::LOC15),
            16 => Some(CDTI2LOC_A::LOC16),
            17 => Some(CDTI2LOC_A::LOC17),
            18 => Some(CDTI2LOC_A::LOC18),
            19 => Some(CDTI2LOC_A::LOC19),
            20 => Some(CDTI2LOC_A::LOC20),
            21 => Some(CDTI2LOC_A::LOC21),
            22 => Some(CDTI2LOC_A::LOC22),
            23 => Some(CDTI2LOC_A::LOC23),
            24 => Some(CDTI2LOC_A::LOC24),
            25 => Some(CDTI2LOC_A::LOC25),
            26 => Some(CDTI2LOC_A::LOC26),
            27 => Some(CDTI2LOC_A::LOC27),
            28 => Some(CDTI2LOC_A::LOC28),
            29 => Some(CDTI2LOC_A::LOC29),
            30 => Some(CDTI2LOC_A::LOC30),
            31 => Some(CDTI2LOC_A::LOC31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI2LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CDTI2LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CDTI2LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CDTI2LOC_A::LOC7
    }
    #[doc = "Checks if the value of the field is `LOC8`"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CDTI2LOC_A::LOC8
    }
    #[doc = "Checks if the value of the field is `LOC9`"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CDTI2LOC_A::LOC9
    }
    #[doc = "Checks if the value of the field is `LOC10`"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CDTI2LOC_A::LOC10
    }
    #[doc = "Checks if the value of the field is `LOC11`"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CDTI2LOC_A::LOC11
    }
    #[doc = "Checks if the value of the field is `LOC12`"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CDTI2LOC_A::LOC12
    }
    #[doc = "Checks if the value of the field is `LOC13`"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CDTI2LOC_A::LOC13
    }
    #[doc = "Checks if the value of the field is `LOC14`"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CDTI2LOC_A::LOC14
    }
    #[doc = "Checks if the value of the field is `LOC15`"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CDTI2LOC_A::LOC15
    }
    #[doc = "Checks if the value of the field is `LOC16`"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CDTI2LOC_A::LOC16
    }
    #[doc = "Checks if the value of the field is `LOC17`"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CDTI2LOC_A::LOC17
    }
    #[doc = "Checks if the value of the field is `LOC18`"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CDTI2LOC_A::LOC18
    }
    #[doc = "Checks if the value of the field is `LOC19`"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CDTI2LOC_A::LOC19
    }
    #[doc = "Checks if the value of the field is `LOC20`"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CDTI2LOC_A::LOC20
    }
    #[doc = "Checks if the value of the field is `LOC21`"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CDTI2LOC_A::LOC21
    }
    #[doc = "Checks if the value of the field is `LOC22`"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CDTI2LOC_A::LOC22
    }
    #[doc = "Checks if the value of the field is `LOC23`"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CDTI2LOC_A::LOC23
    }
    #[doc = "Checks if the value of the field is `LOC24`"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CDTI2LOC_A::LOC24
    }
    #[doc = "Checks if the value of the field is `LOC25`"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CDTI2LOC_A::LOC25
    }
    #[doc = "Checks if the value of the field is `LOC26`"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CDTI2LOC_A::LOC26
    }
    #[doc = "Checks if the value of the field is `LOC27`"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CDTI2LOC_A::LOC27
    }
    #[doc = "Checks if the value of the field is `LOC28`"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CDTI2LOC_A::LOC28
    }
    #[doc = "Checks if the value of the field is `LOC29`"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CDTI2LOC_A::LOC29
    }
    #[doc = "Checks if the value of the field is `LOC30`"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CDTI2LOC_A::LOC30
    }
    #[doc = "Checks if the value of the field is `LOC31`"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CDTI2LOC_A::LOC31
    }
}
#[doc = "Field `CDTI2LOC` writer - I/O Location"]
pub type CDTI2LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI2LOC_A, 6, 16>;
impl<'a> CDTI2LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&self) -> CDTI0LOC_R {
        CDTI0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&self) -> CDTI1LOC_R {
        CDTI1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&self) -> CDTI2LOC_R {
        CDTI2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&mut self) -> CDTI0LOC_W {
        CDTI0LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&mut self) -> CDTI1LOC_W {
        CDTI1LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&mut self) -> CDTI2LOC_W {
        CDTI2LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc2](index.html) module"]
pub struct ROUTELOC2_SPEC;
impl crate::RegisterSpec for ROUTELOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc2::R](R) reader structure"]
impl crate::Readable for ROUTELOC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc2::W](W) writer structure"]
impl crate::Writable for ROUTELOC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for ROUTELOC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
