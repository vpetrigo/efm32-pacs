#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configure scan mode\n\nValue on reset: 0"]
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
#[doc = "Field `SCANMODE` reader - Configure scan mode"]
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
#[doc = "Field `SCANMODE` writer - Configure scan mode"]
pub type SCANMODE_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, SCANMODE_A, 2, 0>;
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
#[doc = "Select scan configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCANCONF_A {
    #[doc = "0: The channel configuration register registers used are directly mapped to the channel number."]
    DIRMAP = 0,
    #[doc = "1: The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
    INVMAP = 1,
    #[doc = "2: The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers"]
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
#[doc = "Field `SCANCONF` reader - Select scan configuration"]
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
#[doc = "Field `SCANCONF` writer - Select scan configuration"]
pub type SCANCONF_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, SCANCONF_A, 2, 2>;
impl<'a> SCANCONF_W<'a> {
    #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
    #[inline(always)]
    pub fn dirmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::DIRMAP)
    }
    #[doc = "The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
    #[inline(always)]
    pub fn invmap(self) -> &'a mut W {
        self.variant(SCANCONF_A::INVMAP)
    }
    #[doc = "The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers"]
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
#[doc = "Field `DUALSAMPLE` reader - Enable dual sample mode"]
pub type DUALSAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `DUALSAMPLE` writer - Enable dual sample mode"]
pub type DUALSAMPLE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 5>;
#[doc = "Field `STRSCANRES` reader - Enable storing of SCANRES"]
pub type STRSCANRES_R = crate::BitReader<bool>;
#[doc = "Field `STRSCANRES` writer - Enable storing of SCANRES"]
pub type STRSCANRES_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 6>;
#[doc = "DMA wake-up from EM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAWU_A {
    #[doc = "0: No DMA wake-up from EM2"]
    DISABLE = 0,
    #[doc = "1: DMA wake-up from EM2 when FIFO count is greater or equal to RESFIDL"]
    ENABLE = 1,
}
impl From<DMAWU_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWU` reader - DMA wake-up from EM2"]
pub type DMAWU_R = crate::BitReader<DMAWU_A>;
impl DMAWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAWU_A {
        match self.bits {
            false => DMAWU_A::DISABLE,
            true => DMAWU_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAWU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAWU_A::ENABLE
    }
}
#[doc = "Field `DMAWU` writer - DMA wake-up from EM2"]
pub type DMAWU_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DMAWU_A, 7>;
impl<'a> DMAWU_W<'a> {
    #[doc = "No DMA wake-up from EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAWU_A::DISABLE)
    }
    #[doc = "DMA wake-up from EM2 when FIFO count is greater or equal to RESFIDL"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAWU_A::ENABLE)
    }
}
#[doc = "Field `RESFIDL` reader - Result FIFO level"]
pub type RESFIDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESFIDL` writer - Result FIFO level"]
pub type RESFIDL_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, 8>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGRUN_A {
    #[doc = "0: LESENSE can not start new scans in debug mode"]
    X0 = 0,
    #[doc = "1: LESENSE can start new scans in debug mode"]
    X1 = 1,
}
impl From<DEBUGRUN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<DEBUGRUN_A>;
impl DEBUGRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGRUN_A {
        match self.bits {
            false => DEBUGRUN_A::X0,
            true => DEBUGRUN_A::X1,
        }
    }
    #[doc = "Checks if the value of the field is `X0`"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == DEBUGRUN_A::X0
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DEBUGRUN_A::X1
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DEBUGRUN_A, 17>;
impl<'a> DEBUGRUN_W<'a> {
    #[doc = "LESENSE can not start new scans in debug mode"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::X0)
    }
    #[doc = "LESENSE can start new scans in debug mode"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DEBUGRUN_A::X1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    pub fn scanmode(&self) -> SCANMODE_R {
        SCANMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select scan configuration"]
    #[inline(always)]
    pub fn scanconf(&self) -> SCANCONF_R {
        SCANCONF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Enable dual sample mode"]
    #[inline(always)]
    pub fn dualsample(&self) -> DUALSAMPLE_R {
        DUALSAMPLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&self) -> STRSCANRES_R {
        STRSCANRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA wake-up from EM2"]
    #[inline(always)]
    pub fn dmawu(&self) -> DMAWU_R {
        DMAWU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Result FIFO level"]
    #[inline(always)]
    pub fn resfidl(&self) -> RESFIDL_R {
        RESFIDL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure scan mode"]
    #[inline(always)]
    pub fn scanmode(&mut self) -> SCANMODE_W {
        SCANMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Select scan configuration"]
    #[inline(always)]
    pub fn scanconf(&mut self) -> SCANCONF_W {
        SCANCONF_W::new(self)
    }
    #[doc = "Bit 5 - Enable dual sample mode"]
    #[inline(always)]
    pub fn dualsample(&mut self) -> DUALSAMPLE_W {
        DUALSAMPLE_W::new(self)
    }
    #[doc = "Bit 6 - Enable storing of SCANRES"]
    #[inline(always)]
    pub fn strscanres(&mut self) -> STRSCANRES_W {
        STRSCANRES_W::new(self)
    }
    #[doc = "Bit 7 - DMA wake-up from EM2"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DMAWU_W {
        DMAWU_W::new(self)
    }
    #[doc = "Bits 8:11 - Result FIFO level"]
    #[inline(always)]
    pub fn resfidl(&mut self) -> RESFIDL_W {
        RESFIDL_W::new(self)
    }
    #[doc = "Bit 17 - Debug Mode Run Enable"]
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
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
