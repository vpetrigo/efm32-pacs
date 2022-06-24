#[doc = "Register `SCANFIFOCFG` reader"]
pub struct R(crate::R<SCANFIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANFIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANFIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANFIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANFIFOCFG` writer"]
pub struct W(crate::W<SCANFIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANFIFOCFG_SPEC>;
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
impl From<crate::W<SCANFIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANFIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALIGNMENT_A {
    #[doc = "0: ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    RIGHT12 = 0,
    #[doc = "1: ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    RIGHT16 = 1,
    #[doc = "2: ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    RIGHT20 = 2,
    #[doc = "3: DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    LEFT12 = 3,
    #[doc = "4: DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    LEFT16 = 4,
    #[doc = "5: DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    LEFT20 = 5,
}
impl From<ALIGNMENT_A> for u8 {
    #[inline(always)]
    fn from(variant: ALIGNMENT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALIGNMENT` reader - Alignment"]
pub type ALIGNMENT_R = crate::FieldReader<u8, ALIGNMENT_A>;
impl ALIGNMENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALIGNMENT_A> {
        match self.bits {
            0 => Some(ALIGNMENT_A::RIGHT12),
            1 => Some(ALIGNMENT_A::RIGHT16),
            2 => Some(ALIGNMENT_A::RIGHT20),
            3 => Some(ALIGNMENT_A::LEFT12),
            4 => Some(ALIGNMENT_A::LEFT16),
            5 => Some(ALIGNMENT_A::LEFT20),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT12`"]
    #[inline(always)]
    pub fn is_right12(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT12
    }
    #[doc = "Checks if the value of the field is `RIGHT16`"]
    #[inline(always)]
    pub fn is_right16(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT16
    }
    #[doc = "Checks if the value of the field is `RIGHT20`"]
    #[inline(always)]
    pub fn is_right20(&self) -> bool {
        *self == ALIGNMENT_A::RIGHT20
    }
    #[doc = "Checks if the value of the field is `LEFT12`"]
    #[inline(always)]
    pub fn is_left12(&self) -> bool {
        *self == ALIGNMENT_A::LEFT12
    }
    #[doc = "Checks if the value of the field is `LEFT16`"]
    #[inline(always)]
    pub fn is_left16(&self) -> bool {
        *self == ALIGNMENT_A::LEFT16
    }
    #[doc = "Checks if the value of the field is `LEFT20`"]
    #[inline(always)]
    pub fn is_left20(&self) -> bool {
        *self == ALIGNMENT_A::LEFT20
    }
}
#[doc = "Field `ALIGNMENT` writer - Alignment"]
pub type ALIGNMENT_W<'a> = crate::FieldWriter<'a, u32, SCANFIFOCFG_SPEC, u8, ALIGNMENT_A, 3, 0>;
impl<'a> ALIGNMENT_W<'a> {
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    #[inline(always)]
    pub fn right12(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT12)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    #[inline(always)]
    pub fn right16(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT16)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    #[inline(always)]
    pub fn right20(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::RIGHT20)
    }
    #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left12(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT12)
    }
    #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left16(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT16)
    }
    #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left20(self) -> &'a mut W {
        self.variant(ALIGNMENT_A::LEFT20)
    }
}
#[doc = "Field `SHOWID` reader - Show ID"]
pub type SHOWID_R = crate::BitReader<bool>;
#[doc = "Field `SHOWID` writer - Show ID"]
pub type SHOWID_W<'a> = crate::BitWriter<'a, u32, SCANFIFOCFG_SPEC, bool, 3>;
#[doc = "Data Valid Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DVL_A {
    #[doc = "0: When 1 entry in the scan FIFO is valid, set the SCANFIFODVL interrupt and request DMA."]
    VALID1 = 0,
    #[doc = "1: When 2 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    VALID2 = 1,
    #[doc = "2: When 3 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    VALID3 = 2,
    #[doc = "3: When 4 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    VALID4 = 3,
}
impl From<DVL_A> for u8 {
    #[inline(always)]
    fn from(variant: DVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DVL` reader - Data Valid Level"]
pub type DVL_R = crate::FieldReader<u8, DVL_A>;
impl DVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVL_A {
        match self.bits {
            0 => DVL_A::VALID1,
            1 => DVL_A::VALID2,
            2 => DVL_A::VALID3,
            3 => DVL_A::VALID4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALID1`"]
    #[inline(always)]
    pub fn is_valid1(&self) -> bool {
        *self == DVL_A::VALID1
    }
    #[doc = "Checks if the value of the field is `VALID2`"]
    #[inline(always)]
    pub fn is_valid2(&self) -> bool {
        *self == DVL_A::VALID2
    }
    #[doc = "Checks if the value of the field is `VALID3`"]
    #[inline(always)]
    pub fn is_valid3(&self) -> bool {
        *self == DVL_A::VALID3
    }
    #[doc = "Checks if the value of the field is `VALID4`"]
    #[inline(always)]
    pub fn is_valid4(&self) -> bool {
        *self == DVL_A::VALID4
    }
}
#[doc = "Field `DVL` writer - Data Valid Level"]
pub type DVL_W<'a> = crate::FieldWriterSafe<'a, u32, SCANFIFOCFG_SPEC, u8, DVL_A, 2, 4>;
impl<'a> DVL_W<'a> {
    #[doc = "When 1 entry in the scan FIFO is valid, set the SCANFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid1(self) -> &'a mut W {
        self.variant(DVL_A::VALID1)
    }
    #[doc = "When 2 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid2(self) -> &'a mut W {
        self.variant(DVL_A::VALID2)
    }
    #[doc = "When 3 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid3(self) -> &'a mut W {
        self.variant(DVL_A::VALID3)
    }
    #[doc = "When 4 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid4(self) -> &'a mut W {
        self.variant(DVL_A::VALID4)
    }
}
#[doc = "Scan FIFO DMA Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAWUFIFOSCAN_A {
    #[doc = "0: While in EM2 or EM3, the DMA controller will not be requested."]
    DISABLED = 0,
    #[doc = "1: While in EM2 or EM3, the DMA controller will be requested when the scan FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    ENABLED = 1,
}
impl From<DMAWUFIFOSCAN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWUFIFOSCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWUFIFOSCAN` reader - Scan FIFO DMA Wakeup"]
pub type DMAWUFIFOSCAN_R = crate::BitReader<DMAWUFIFOSCAN_A>;
impl DMAWUFIFOSCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAWUFIFOSCAN_A {
        match self.bits {
            false => DMAWUFIFOSCAN_A::DISABLED,
            true => DMAWUFIFOSCAN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAWUFIFOSCAN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAWUFIFOSCAN_A::ENABLED
    }
}
#[doc = "Field `DMAWUFIFOSCAN` writer - Scan FIFO DMA Wakeup"]
pub type DMAWUFIFOSCAN_W<'a> = crate::BitWriter<'a, u32, SCANFIFOCFG_SPEC, DMAWUFIFOSCAN_A, 8>;
impl<'a> DMAWUFIFOSCAN_W<'a> {
    #[doc = "While in EM2 or EM3, the DMA controller will not be requested."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAWUFIFOSCAN_A::DISABLED)
    }
    #[doc = "While in EM2 or EM3, the DMA controller will be requested when the scan FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAWUFIFOSCAN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    pub fn alignment(&self) -> ALIGNMENT_R {
        ALIGNMENT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    pub fn showid(&self) -> SHOWID_R {
        SHOWID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data Valid Level"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Scan FIFO DMA Wakeup"]
    #[inline(always)]
    pub fn dmawufifoscan(&self) -> DMAWUFIFOSCAN_R {
        DMAWUFIFOSCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    pub fn alignment(&mut self) -> ALIGNMENT_W {
        ALIGNMENT_W::new(self)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    pub fn showid(&mut self) -> SHOWID_W {
        SHOWID_W::new(self)
    }
    #[doc = "Bits 4:5 - Data Valid Level"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DVL_W {
        DVL_W::new(self)
    }
    #[doc = "Bit 8 - Scan FIFO DMA Wakeup"]
    #[inline(always)]
    pub fn dmawufifoscan(&mut self) -> DMAWUFIFOSCAN_W {
        DMAWUFIFOSCAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifocfg](index.html) module"]
pub struct SCANFIFOCFG_SPEC;
impl crate::RegisterSpec for SCANFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanfifocfg::R](R) reader structure"]
impl crate::Readable for SCANFIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanfifocfg::W](W) writer structure"]
impl crate::Writable for SCANFIFOCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCANFIFOCFG to value 0x30"]
impl crate::Resettable for SCANFIFOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
