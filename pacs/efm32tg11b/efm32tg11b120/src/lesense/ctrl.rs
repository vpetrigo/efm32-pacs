#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configure Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCANMODE_A {
    #[doc = "0: A new scan is started each time the period counter overflows"]
    PERIODIC = 0,
    #[doc = "1: A single scan is performed when START in CMD is set"]
    ONESHOT = 1,
    #[doc = "2: Pulse on PRS channel"]
    PRS = 2,
}
impl From<SCANMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCANMODE` reader - Configure Scan Mode"]
pub type SCANMODE_R = crate::FieldReader<u8, SCANMODE_A>;
impl SCANMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCANMODE_A> {
        match self.bits {
            0 => Some(SCANMODE_A::PERIODIC),
            1 => Some(SCANMODE_A::ONESHOT),
            2 => Some(SCANMODE_A::PRS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == SCANMODE_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == SCANMODE_A::ONESHOT
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SCANMODE_A::PRS
    }
}
#[doc = "Field `SCANMODE` writer - Configure Scan Mode"]
pub type SCANMODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, SCANMODE_A, 2, 0>;
impl<'a> SCANMODE_W<'a> {
    #[doc = "A new scan is started each time the period counter overflows"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(SCANMODE_A::PERIODIC)
    }
    #[doc = "A single scan is performed when START in CMD is set"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut W {
        self.variant(SCANMODE_A::ONESHOT)
    }
    #[doc = "Pulse on PRS channel"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(SCANMODE_A::PRS)
    }
}
#[doc = "Scan Start PRS Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRSSEL` reader - Scan Start PRS Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL_A {
        match self.bits {
            0 => PRSSEL_A::PRSCH0,
            1 => PRSSEL_A::PRSCH1,
            2 => PRSSEL_A::PRSCH2,
            3 => PRSSEL_A::PRSCH3,
            4 => PRSSEL_A::PRSCH4,
            5 => PRSSEL_A::PRSCH5,
            6 => PRSSEL_A::PRSCH6,
            7 => PRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
}
#[doc = "Field `PRSSEL` writer - Scan Start PRS Select"]
pub type PRSSEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PRSSEL_A, 3, 2>;
impl<'a> PRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
}
#[doc = "Select Scan Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCANCONF_A {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP = 0,
    #[doc = "1: The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    INVMAP = 1,
    #[doc = "2: The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    TOGGLE = 2,
    #[doc = "3: The decoder state defines the CONF registers to be used."]
    DECDEF = 3,
}
impl From<SCANCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SCANCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCANCONF` reader - Select Scan Configuration"]
pub type SCANCONF_R = crate::FieldReader<u8, SCANCONF_A>;
impl SCANCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCANCONF_A {
        match self.bits {
            0 => SCANCONF_A::DIRMAP,
            1 => SCANCONF_A::INVMAP,
            2 => SCANCONF_A::TOGGLE,
            3 => SCANCONF_A::DECDEF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIRMAP`"]
    #[inline(always)]
    pub fn is_dirmap(&self) -> bool {
        *self == SCANCONF_A::DIRMAP
    }
    #[doc = "Checks if the value of the field is `INVMAP`"]
    #[inline(always)]
    pub fn is_invmap(&self) -> bool {
        *self == SCANCONF_A::INVMAP
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == SCANCONF_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `DECDEF`"]
    #[inline(always)]
    pub fn is_decdef(&self) -> bool {
        *self == SCANCONF_A::DECDEF
    }
}
#[doc = "Field `SCANCONF` writer - Select Scan Configuration"]
pub type SCANCONF_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, SCANCONF_A, 2, 7>;
impl<'a> SCANCONF_W<'a> {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::DIRMAP)
    }
    #[doc = "The channel configuration register registers used are CHX+8_CONF for channels 0-7 and CHX-8_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::INVMAP)
    }
    #[doc = "The channel configuration register registers used toggles between CHX_CONF and CHX+8_CONF when channel x triggers"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(SCANCONF_A::TOGGLE)
    }
    #[doc = "The decoder state defines the CONF registers to be used."]
    #[inline(always)]
    pub fn decdef(self) -> &'a mut W {
        self.variant(SCANCONF_A::DECDEF)
    }
}
#[doc = "Field `ALTEXMAP` reader - Alternative Excitation Map"]
pub type ALTEXMAP_R = crate::BitReader<bool>;
#[doc = "Field `ALTEXMAP` writer - Alternative Excitation Map"]
pub type ALTEXMAP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 11>;
#[doc = "Field `DUALSAMPLE` reader - Enable Dual Sample Mode"]
pub type DUALSAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `DUALSAMPLE` writer - Enable Dual Sample Mode"]
pub type DUALSAMPLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "Field `BUFOW` reader - Result Buffer Overwrite"]
pub type BUFOW_R = crate::BitReader<bool>;
#[doc = "Field `BUFOW` writer - Result Buffer Overwrite"]
pub type BUFOW_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `STRSCANRES` reader - Enable Storing of SCANRES"]
pub type STRSCANRES_R = crate::BitReader<bool>;
#[doc = "Field `STRSCANRES` writer - Enable Storing of SCANRES"]
pub type STRSCANRES_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 17>;
#[doc = "Field `BUFIDL` reader - Result Buffer Interrupt and DMA Trigger Level"]
pub type BUFIDL_R = crate::BitReader<bool>;
#[doc = "Field `BUFIDL` writer - Result Buffer Interrupt and DMA Trigger Level"]
pub type BUFIDL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 19>;
#[doc = "DMA Wake-up From EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAWU_A {
    #[doc = "0: No DMA wake-up from EM2"]
    DISABLE = 0,
    #[doc = "1: DMA wake-up from EM2 when data is valid in the result buffer"]
    BUFDATAV = 1,
    #[doc = "2: DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    BUFLEVEL = 2,
}
impl From<DMAWU_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAWU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMAWU` reader - DMA Wake-up From EM2"]
pub type DMAWU_R = crate::FieldReader<u8, DMAWU_A>;
impl DMAWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAWU_A> {
        match self.bits {
            0 => Some(DMAWU_A::DISABLE),
            1 => Some(DMAWU_A::BUFDATAV),
            2 => Some(DMAWU_A::BUFLEVEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAWU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `BUFDATAV`"]
    #[inline(always)]
    pub fn is_bufdatav(&self) -> bool {
        *self == DMAWU_A::BUFDATAV
    }
    #[doc = "Checks if the value of the field is `BUFLEVEL`"]
    #[inline(always)]
    pub fn is_buflevel(&self) -> bool {
        *self == DMAWU_A::BUFLEVEL
    }
}
#[doc = "Field `DMAWU` writer - DMA Wake-up From EM2"]
pub type DMAWU_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, DMAWU_A, 2, 20>;
impl<'a> DMAWU_W<'a> {
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAWU_A::DISABLE)
    }
    #[doc = "DMA wake-up from EM2 when data is valid in the result buffer"]
    #[inline(always)]
    pub fn bufdatav(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFDATAV)
    }
    #[doc = "DMA wake-up from EM2 when the result buffer is full/half-full depending on BUFIDL configuration"]
    #[inline(always)]
    pub fn buflevel(self) -> &'a mut W {
        self.variant(DMAWU_A::BUFLEVEL)
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 22>;
impl R {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> SCANMODE_R {
        SCANMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> SCANCONF_R {
        SCANCONF_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&self) -> ALTEXMAP_R {
        ALTEXMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DUALSAMPLE_R {
        DUALSAMPLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&self) -> BUFOW_R {
        BUFOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> STRSCANRES_R {
        STRSCANRES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&self) -> BUFIDL_R {
        BUFIDL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DMAWU_R {
        DMAWU_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure Scan Mode"]
    #[inline(always)]
    pub fn scanmode(&mut self) -> SCANMODE_W {
        SCANMODE_W::new(self)
    }
    #[doc = "Bits 2:4 - Scan Start PRS Select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PRSSEL_W {
        PRSSEL_W::new(self)
    }
    #[doc = "Bits 7:8 - Select Scan Configuration"]
    #[inline(always)]
    pub fn scanconf(&mut self) -> SCANCONF_W {
        SCANCONF_W::new(self)
    }
    #[doc = "Bit 11 - Alternative Excitation Map"]
    #[inline(always)]
    pub fn altexmap(&mut self) -> ALTEXMAP_W {
        ALTEXMAP_W::new(self)
    }
    #[doc = "Bit 13 - Enable Dual Sample Mode"]
    #[inline(always)]
    pub fn dualsample(&mut self) -> DUALSAMPLE_W {
        DUALSAMPLE_W::new(self)
    }
    #[doc = "Bit 16 - Result Buffer Overwrite"]
    #[inline(always)]
    pub fn bufow(&mut self) -> BUFOW_W {
        BUFOW_W::new(self)
    }
    #[doc = "Bit 17 - Enable Storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&mut self) -> STRSCANRES_W {
        STRSCANRES_W::new(self)
    }
    #[doc = "Bit 19 - Result Buffer Interrupt and DMA Trigger Level"]
    #[inline(always)]
    pub fn bufidl(&mut self) -> BUFIDL_W {
        BUFIDL_W::new(self)
    }
    #[doc = "Bits 20:21 - DMA Wake-up From EM2"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DMAWU_W {
        DMAWU_W::new(self)
    }
    #[doc = "Bit 22 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
