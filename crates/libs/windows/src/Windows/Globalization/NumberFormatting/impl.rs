pub trait INumberFormatter_Impl: Sized {
    fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for INumberFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter";
}
impl INumberFormatter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: isize>() -> INumberFormatter_Vtbl {
        unsafe extern "system" fn FormatInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter_Impl::FormatInt(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter_Impl::FormatUInt(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter_Impl::FormatDouble(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatter, OFFSET>(),
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatter as windows_core::Interface>::IID
    }
}
pub trait INumberFormatter2_Impl: Sized {
    fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
impl INumberFormatter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: isize>() -> INumberFormatter2_Vtbl {
        unsafe extern "system" fn FormatInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter2_Impl::FormatInt(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter2_Impl::FormatUInt(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatter2_Impl::FormatDouble(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatter2, OFFSET>(),
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait INumberFormatterOptions_Impl: Sized {
    fn Languages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>;
    fn GeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn IntegerDigits(&self) -> windows_core::Result<i32>;
    fn SetIntegerDigits(&self, value: i32) -> windows_core::Result<()>;
    fn FractionDigits(&self) -> windows_core::Result<i32>;
    fn SetFractionDigits(&self, value: i32) -> windows_core::Result<()>;
    fn IsGrouped(&self) -> windows_core::Result<bool>;
    fn SetIsGrouped(&self, value: bool) -> windows_core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(&self) -> windows_core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> windows_core::Result<()>;
    fn NumeralSystem(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNumeralSystem(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ResolvedLanguage(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ResolvedGeographicRegion(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for INumberFormatterOptions {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatterOptions";
}
#[cfg(feature = "Foundation_Collections")]
impl INumberFormatterOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>() -> INumberFormatterOptions_Vtbl {
        unsafe extern "system" fn Languages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::Languages(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeographicRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::GeographicRegion(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntegerDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::IntegerDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntegerDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberFormatterOptions_Impl::SetIntegerDigits(this, value).into()
        }
        unsafe extern "system" fn FractionDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::FractionDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFractionDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberFormatterOptions_Impl::SetFractionDigits(this, value).into()
        }
        unsafe extern "system" fn IsGrouped<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::IsGrouped(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGrouped<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberFormatterOptions_Impl::SetIsGrouped(this, value).into()
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::IsDecimalPointAlwaysDisplayed(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberFormatterOptions_Impl::SetIsDecimalPointAlwaysDisplayed(this, value).into()
        }
        unsafe extern "system" fn NumeralSystem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::NumeralSystem(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumeralSystem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberFormatterOptions_Impl::SetNumeralSystem(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ResolvedLanguage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::ResolvedLanguage(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberFormatterOptions_Impl::ResolvedGeographicRegion(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatterOptions, OFFSET>(),
            Languages: Languages::<Identity, Impl, OFFSET>,
            GeographicRegion: GeographicRegion::<Identity, Impl, OFFSET>,
            IntegerDigits: IntegerDigits::<Identity, Impl, OFFSET>,
            SetIntegerDigits: SetIntegerDigits::<Identity, Impl, OFFSET>,
            FractionDigits: FractionDigits::<Identity, Impl, OFFSET>,
            SetFractionDigits: SetFractionDigits::<Identity, Impl, OFFSET>,
            IsGrouped: IsGrouped::<Identity, Impl, OFFSET>,
            SetIsGrouped: SetIsGrouped::<Identity, Impl, OFFSET>,
            IsDecimalPointAlwaysDisplayed: IsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            SetIsDecimalPointAlwaysDisplayed: SetIsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            NumeralSystem: NumeralSystem::<Identity, Impl, OFFSET>,
            SetNumeralSystem: SetNumeralSystem::<Identity, Impl, OFFSET>,
            ResolvedLanguage: ResolvedLanguage::<Identity, Impl, OFFSET>,
            ResolvedGeographicRegion: ResolvedGeographicRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatterOptions as windows_core::Interface>::IID
    }
}
pub trait INumberParser_Impl: Sized {
    fn ParseInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IReference<f64>>;
}
impl windows_core::RuntimeName for INumberParser {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberParser";
}
impl INumberParser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: isize>() -> INumberParser_Vtbl {
        unsafe extern "system" fn ParseInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: std::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberParser_Impl::ParseInt(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseUInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: std::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberParser_Impl::ParseUInt(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: std::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberParser_Impl::ParseDouble(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberParser, OFFSET>(),
            ParseInt: ParseInt::<Identity, Impl, OFFSET>,
            ParseUInt: ParseUInt::<Identity, Impl, OFFSET>,
            ParseDouble: ParseDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberParser as windows_core::Interface>::IID
    }
}
pub trait INumberRounder_Impl: Sized {
    fn RoundInt32(&self, value: i32) -> windows_core::Result<i32>;
    fn RoundUInt32(&self, value: u32) -> windows_core::Result<u32>;
    fn RoundInt64(&self, value: i64) -> windows_core::Result<i64>;
    fn RoundUInt64(&self, value: u64) -> windows_core::Result<u64>;
    fn RoundSingle(&self, value: f32) -> windows_core::Result<f32>;
    fn RoundDouble(&self, value: f64) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for INumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounder";
}
impl INumberRounder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>() -> INumberRounder_Vtbl {
        unsafe extern "system" fn RoundInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundInt32(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundUInt32(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundInt64(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundUInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundUInt64(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundSingle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32, result__: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundSingle(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounder_Impl::RoundDouble(this, value) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberRounder, OFFSET>(),
            RoundInt32: RoundInt32::<Identity, Impl, OFFSET>,
            RoundUInt32: RoundUInt32::<Identity, Impl, OFFSET>,
            RoundInt64: RoundInt64::<Identity, Impl, OFFSET>,
            RoundUInt64: RoundUInt64::<Identity, Impl, OFFSET>,
            RoundSingle: RoundSingle::<Identity, Impl, OFFSET>,
            RoundDouble: RoundDouble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberRounder as windows_core::Interface>::IID
    }
}
pub trait INumberRounderOption_Impl: Sized {
    fn NumberRounder(&self) -> windows_core::Result<INumberRounder>;
    fn SetNumberRounder(&self, value: Option<&INumberRounder>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INumberRounderOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberRounderOption";
}
impl INumberRounderOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: isize>() -> INumberRounderOption_Vtbl {
        unsafe extern "system" fn NumberRounder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INumberRounderOption_Impl::NumberRounder(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberRounder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INumberRounderOption_Impl::SetNumberRounder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberRounderOption, OFFSET>(),
            NumberRounder: NumberRounder::<Identity, Impl, OFFSET>,
            SetNumberRounder: SetNumberRounder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberRounderOption as windows_core::Interface>::IID
    }
}
pub trait ISignedZeroOption_Impl: Sized {
    fn IsZeroSigned(&self) -> windows_core::Result<bool>;
    fn SetIsZeroSigned(&self, value: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISignedZeroOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignedZeroOption";
}
impl ISignedZeroOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: isize>() -> ISignedZeroOption_Vtbl {
        unsafe extern "system" fn IsZeroSigned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISignedZeroOption_Impl::IsZeroSigned(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZeroSigned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISignedZeroOption_Impl::SetIsZeroSigned(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISignedZeroOption, OFFSET>(),
            IsZeroSigned: IsZeroSigned::<Identity, Impl, OFFSET>,
            SetIsZeroSigned: SetIsZeroSigned::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignedZeroOption as windows_core::Interface>::IID
    }
}
pub trait ISignificantDigitsOption_Impl: Sized {
    fn SignificantDigits(&self) -> windows_core::Result<i32>;
    fn SetSignificantDigits(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISignificantDigitsOption {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.ISignificantDigitsOption";
}
impl ISignificantDigitsOption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>() -> ISignificantDigitsOption_Vtbl {
        unsafe extern "system" fn SignificantDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISignificantDigitsOption_Impl::SignificantDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignificantDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISignificantDigitsOption_Impl::SetSignificantDigits(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISignificantDigitsOption, OFFSET>(),
            SignificantDigits: SignificantDigits::<Identity, Impl, OFFSET>,
            SetSignificantDigits: SetSignificantDigits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignificantDigitsOption as windows_core::Interface>::IID
    }
}
