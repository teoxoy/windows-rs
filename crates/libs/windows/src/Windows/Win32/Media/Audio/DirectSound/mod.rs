#[inline]
pub unsafe fn DirectSoundCaptureCreate<P0>(pcguiddevice: Option<*const windows_core::GUID>, ppdsc: *mut Option<IDirectSoundCapture>, punkouter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCaptureCreate(pcguiddevice : *const windows_core::GUID, ppdsc : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCaptureCreate(core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null())), core::mem::transmute(ppdsc), punkouter.param().abi()).ok()
}
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<P0>(pcguiddevice: Option<*const windows_core::GUID>, ppdsc8: *mut Option<IDirectSoundCapture>, punkouter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCaptureCreate8(pcguiddevice : *const windows_core::GUID, ppdsc8 : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCaptureCreate8(core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null())), core::mem::transmute(ppdsc8), punkouter.param().abi()).ok()
}
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCaptureEnumerateA(pdsenumcallback, core::mem::transmute(pcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCaptureEnumerateW(pdsenumcallback, core::mem::transmute(pcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn DirectSoundCreate<P0>(pcguiddevice: Option<*const windows_core::GUID>, ppds: *mut Option<IDirectSound>, punkouter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCreate(pcguiddevice : *const windows_core::GUID, ppds : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCreate(core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null())), core::mem::transmute(ppds), punkouter.param().abi()).ok()
}
#[inline]
pub unsafe fn DirectSoundCreate8<P0>(pcguiddevice: Option<*const windows_core::GUID>, ppds8: *mut Option<IDirectSound8>, punkouter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dsound.dll" "system" fn DirectSoundCreate8(pcguiddevice : *const windows_core::GUID, ppds8 : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundCreate8(core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null())), core::mem::transmute(ppds8), punkouter.param().abi()).ok()
}
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("dsound.dll" "system" fn DirectSoundEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundEnumerateA(pdsenumcallback, core::mem::transmute(pcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("dsound.dll" "system" fn DirectSoundEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundEnumerateW(pdsenumcallback, core::mem::transmute(pcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<P0, P1>(pcguidcapturedevice: Option<*const windows_core::GUID>, pcguidrenderdevice: Option<*const windows_core::GUID>, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: P0, dwlevel: u32, ppdsfd: *mut Option<IDirectSoundFullDuplex>, ppdscbuffer8: *mut Option<IDirectSoundCaptureBuffer8>, ppdsbuffer8: *mut Option<IDirectSoundBuffer8>, punkouter: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dsound.dll" "system" fn DirectSoundFullDuplexCreate(pcguidcapturedevice : *const windows_core::GUID, pcguidrenderdevice : *const windows_core::GUID, pcdscbufferdesc : *const DSCBUFFERDESC, pcdsbufferdesc : *const DSBUFFERDESC, hwnd : super::super::super::Foundation:: HWND, dwlevel : u32, ppdsfd : *mut * mut core::ffi::c_void, ppdscbuffer8 : *mut * mut core::ffi::c_void, ppdsbuffer8 : *mut * mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DirectSoundFullDuplexCreate(core::mem::transmute(pcguidcapturedevice.unwrap_or(std::ptr::null())), core::mem::transmute(pcguidrenderdevice.unwrap_or(std::ptr::null())), pcdscbufferdesc, pcdsbufferdesc, hwnd.param().abi(), dwlevel, core::mem::transmute(ppdsfd), core::mem::transmute(ppdscbuffer8), core::mem::transmute(ppdsbuffer8), punkouter.param().abi()).ok()
}
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: Option<*const windows_core::GUID>) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("dsound.dll" "system" fn GetDeviceID(pguidsrc : *const windows_core::GUID, pguiddest : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    GetDeviceID(core::mem::transmute(pguidsrc.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
windows_core::imp::define_interface!(IDirectSound, IDirectSound_Vtbl, 0x279afa83_4981_11ce_a521_0020af0be560);
impl std::ops::Deref for IDirectSound {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound, windows_core::IUnknown);
impl IDirectSound {
    pub unsafe fn CreateSoundBuffer<P0>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut Option<IDirectSoundBuffer>, punkouter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateSoundBuffer)(windows_core::Interface::as_raw(self), pcdsbufferdesc, core::mem::transmute(ppdsbuffer), punkouter.param().abi()).ok()
    }
    pub unsafe fn GetCaps(&self, pdscaps: *mut DSCAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdscaps).ok()
    }
    pub unsafe fn DuplicateSoundBuffer<P0>(&self, pdsbufferoriginal: P0) -> windows_core::Result<IDirectSoundBuffer>
    where
        P0: windows_core::Param<IDirectSoundBuffer>,
    {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).DuplicateSoundBuffer)(windows_core::Interface::as_raw(self), pdsbufferoriginal.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCooperativeLevel<P0>(&self, hwnd: P0, dwlevel: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), hwnd.param().abi(), dwlevel).ok()
    }
    pub unsafe fn Compact(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSpeakerConfig(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpeakerConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSpeakerConfig)(windows_core::Interface::as_raw(self), dwspeakerconfig).ok()
    }
    pub unsafe fn Initialize(&self, pcguiddevice: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IDirectSound_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCAPS) -> windows_core::HRESULT,
    pub DuplicateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSound3DBuffer, IDirectSound3DBuffer_Vtbl, 0x279afa86_4981_11ce_a521_0020af0be560);
impl std::ops::Deref for IDirectSound3DBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound3DBuffer, windows_core::IUnknown);
impl IDirectSound3DBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pds3dbuffer).ok()
    }
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConeAngles)(windows_core::Interface::as_raw(self), pdwinsideconeangle, pdwoutsideconeangle).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetConeOrientation(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConeOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConeOutsideVolume(&self) -> windows_core::Result<i32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConeOutsideVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxDistance(&self) -> windows_core::Result<f32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinDistance(&self) -> windows_core::Result<f32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMode(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcds3dbuffer, dwapply).ok()
    }
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConeAngles)(windows_core::Interface::as_raw(self), dwinsideconeangle, dwoutsideconeangle, dwapply).ok()
    }
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConeOrientation)(windows_core::Interface::as_raw(self), x, y, z, dwapply).ok()
    }
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConeOutsideVolume)(windows_core::Interface::as_raw(self), lconeoutsidevolume, dwapply).ok()
    }
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxDistance)(windows_core::Interface::as_raw(self), flmaxdistance, dwapply).ok()
    }
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinDistance)(windows_core::Interface::as_raw(self), flmindistance, dwapply).ok()
    }
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), dwmode, dwapply).ok()
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply).ok()
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVelocity)(windows_core::Interface::as_raw(self), x, y, z, dwapply).ok()
    }
}
#[repr(C)]
pub struct IDirectSound3DBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DS3DBUFFER) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetConeOrientation: usize,
    pub GetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DS3DBUFFER, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub SetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub SetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSound3DListener, IDirectSound3DListener_Vtbl, 0x279afa84_4981_11ce_a521_0020af0be560);
impl std::ops::Deref for IDirectSound3DListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound3DListener, windows_core::IUnknown);
impl IDirectSound3DListener {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self, plistener: *mut DS3DLISTENER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), plistener).ok()
    }
    pub unsafe fn GetDistanceFactor(&self) -> windows_core::Result<f32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDistanceFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDopplerFactor(&self) -> windows_core::Result<f32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDopplerFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOrientation)(windows_core::Interface::as_raw(self), pvorientfront, pvorienttop).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRolloffFactor(&self) -> windows_core::Result<f32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRolloffFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pclistener, dwapply).ok()
    }
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDistanceFactor)(windows_core::Interface::as_raw(self), fldistancefactor, dwapply).ok()
    }
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDopplerFactor)(windows_core::Interface::as_raw(self), fldopplerfactor, dwapply).ok()
    }
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), xfront, yfront, zfront, xtop, ytop, ztop, dwapply).ok()
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply).ok()
    }
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRolloffFactor)(windows_core::Interface::as_raw(self), flrollofffactor, dwapply).ok()
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVelocity)(windows_core::Interface::as_raw(self), x, y, z, dwapply).ok()
    }
    pub unsafe fn CommitDeferredSettings(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitDeferredSettings)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectSound3DListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DS3DLISTENER) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOrientation: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    pub GetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DS3DLISTENER, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub CommitDeferredSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSound8, IDirectSound8_Vtbl, 0xc50a7e93_f395_4834_9ef6_7fa99de50966);
impl std::ops::Deref for IDirectSound8 {
    type Target = IDirectSound;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound8, windows_core::IUnknown, IDirectSound);
impl IDirectSound8 {
    pub unsafe fn VerifyCertification(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).VerifyCertification)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectSound8_Vtbl {
    pub base__: IDirectSound_Vtbl,
    pub VerifyCertification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundBuffer, IDirectSoundBuffer_Vtbl, 0x279afa85_4981_11ce_a521_0020af0be560);
impl std::ops::Deref for IDirectSoundBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer, windows_core::IUnknown);
impl IDirectSoundBuffer {
    pub unsafe fn GetCaps(&self, pdsbuffercaps: *mut DSBCAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdsbuffercaps).ok()
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: Option<*mut u32>, pdwcurrentwritecursor: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwcurrentplaycursor.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwcurrentwritecursor.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: Option<*mut super::WAVEFORMATEX>, dwsizeallocated: u32, pdwsizewritten: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), core::mem::transmute(pwfxformat.unwrap_or(std::ptr::null_mut())), dwsizeallocated, core::mem::transmute(pdwsizewritten.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVolume(&self) -> windows_core::Result<i32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPan(&self) -> windows_core::Result<i32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize<P0>(&self, pdirectsound: P0, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectSound>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pcdsbufferdesc).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: Option<*mut *mut core::ffi::c_void>, pdwaudiobytes2: Option<*mut u32>, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1, pdwaudiobytes1, core::mem::transmute(ppvaudioptr2.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwaudiobytes2.unwrap_or(std::ptr::null_mut())), dwflags).ok()
    }
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self), dwreserved1, dwpriority, dwflags).ok()
    }
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), dwnewposition).ok()
    }
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), pcfxformat).ok()
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume).ok()
    }
    pub unsafe fn SetPan(&self, lpan: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPan)(windows_core::Interface::as_raw(self), lpan).ok()
    }
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrequency)(windows_core::Interface::as_raw(self), dwfrequency).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: Option<*const core::ffi::c_void>, dwaudiobytes2: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), pvaudioptr1, dwaudiobytes1, core::mem::transmute(pvaudioptr2.unwrap_or(std::ptr::null())), dwaudiobytes2).ok()
    }
    pub unsafe fn Restore(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSBCAPS) -> windows_core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, u32, *mut u32) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetPan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DSBUFFERDESC) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetPan: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundBuffer8, IDirectSoundBuffer8_Vtbl, 0x6825a449_7524_4d82_920f_50e36ab3ab1e);
impl std::ops::Deref for IDirectSoundBuffer8 {
    type Target = IDirectSoundBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer8, windows_core::IUnknown, IDirectSoundBuffer);
impl IDirectSoundBuffer8 {
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: Option<*const DSEFFECTDESC>, pdwresultcodes: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFX)(windows_core::Interface::as_raw(self), dweffectscount, core::mem::transmute(pdsfxdesc.unwrap_or(std::ptr::null())), core::mem::transmute(pdwresultcodes.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AcquireResources(&self, dwflags: u32, pdwresultcodes: &mut [u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcquireResources)(windows_core::Interface::as_raw(self), dwflags, pdwresultcodes.len().try_into().unwrap(), core::mem::transmute(pdwresultcodes.as_ptr())).ok()
    }
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundBuffer8_Vtbl {
    pub base__: IDirectSoundBuffer_Vtbl,
    pub SetFX: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DSEFFECTDESC, *mut u32) -> windows_core::HRESULT,
    pub AcquireResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundCapture, IDirectSoundCapture_Vtbl, 0xb0210781_89cd_11d0_af08_00a0c925cd16);
impl std::ops::Deref for IDirectSoundCapture {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCapture, windows_core::IUnknown);
impl IDirectSoundCapture {
    pub unsafe fn CreateCaptureBuffer<P0>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut Option<IDirectSoundCaptureBuffer>, punkouter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateCaptureBuffer)(windows_core::Interface::as_raw(self), pcdscbufferdesc, core::mem::transmute(ppdscbuffer), punkouter.param().abi()).ok()
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCCAPS> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize(&self, pcguiddevice: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(pcguiddevice.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundCapture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCaptureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCCAPS) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer, IDirectSoundCaptureBuffer_Vtbl, 0xb0210782_89cd_11d0_af08_00a0c925cd16);
impl std::ops::Deref for IDirectSoundCaptureBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer, windows_core::IUnknown);
impl IDirectSoundCaptureBuffer {
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCBCAPS> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: Option<*mut u32>, pdwreadposition: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwcaptureposition.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwreadposition.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: Option<*mut super::WAVEFORMATEX>, dwsizeallocated: u32, pdwsizewritten: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), core::mem::transmute(pwfxformat.unwrap_or(std::ptr::null_mut())), dwsizeallocated, core::mem::transmute(pdwsizewritten.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize<P0>(&self, pdirectsoundcapture: P0, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectSoundCapture>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsoundcapture.param().abi(), pcdscbufferdesc).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: Option<*mut *mut core::ffi::c_void>, pdwaudiobytes2: Option<*mut u32>, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1, pdwaudiobytes1, core::mem::transmute(ppvaudioptr2.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwaudiobytes2.unwrap_or(std::ptr::null_mut())), dwflags).ok()
    }
    pub unsafe fn Start(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: Option<*const core::ffi::c_void>, dwaudiobytes2: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), pvaudioptr1, dwaudiobytes1, core::mem::transmute(pvaudioptr2.unwrap_or(std::ptr::null())), dwaudiobytes2).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundCaptureBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCBCAPS) -> windows_core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WAVEFORMATEX, u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DSCBUFFERDESC) -> windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer8, IDirectSoundCaptureBuffer8_Vtbl, 0x00990df4_0dbb_4872_833e_6d303e80aeb6);
impl std::ops::Deref for IDirectSoundCaptureBuffer8 {
    type Target = IDirectSoundCaptureBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer8, windows_core::IUnknown, IDirectSoundCaptureBuffer);
impl IDirectSoundCaptureBuffer8 {
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject).ok()
    }
    pub unsafe fn GetFXStatus(&self, pdwfxstatus: &mut [u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFXStatus)(windows_core::Interface::as_raw(self), pdwfxstatus.len().try_into().unwrap(), core::mem::transmute(pdwfxstatus.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundCaptureBuffer8_Vtbl {
    pub base__: IDirectSoundCaptureBuffer_Vtbl,
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFXStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundCaptureFXAec, IDirectSoundCaptureFXAec_Vtbl, 0xad74143d_903d_4ab7_8066_28d363036d65);
impl std::ops::Deref for IDirectSoundCaptureFXAec {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXAec, windows_core::IUnknown);
impl IDirectSoundCaptureFXAec {
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pdscfxaec).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundCaptureFXAec_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCFXAec) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCFXAec) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundCaptureFXNoiseSuppress, IDirectSoundCaptureFXNoiseSuppress_Vtbl, 0xed311e41_fbae_4175_9625_cd0854f693ca);
impl std::ops::Deref for IDirectSoundCaptureFXNoiseSuppress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXNoiseSuppress, windows_core::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdscfxnoisesuppress).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSCFXNoiseSuppress> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXChorus, IDirectSoundFXChorus_Vtbl, 0x880842e3_145f_43e6_a934_a71806e50547);
impl std::ops::Deref for IDirectSoundFXChorus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXChorus, windows_core::IUnknown);
impl IDirectSoundFXChorus {
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxchorus).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxchorus: *mut DSFXChorus) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxchorus).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFXChorus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXChorus) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXChorus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXCompressor, IDirectSoundFXCompressor_Vtbl, 0x4bbd1154_62f6_4e2c_a15c_d3b6c417f7a0);
impl std::ops::Deref for IDirectSoundFXCompressor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXCompressor, windows_core::IUnknown);
impl IDirectSoundFXCompressor {
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxcompressor).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxcompressor).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFXCompressor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXCompressor) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXCompressor) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXDistortion, IDirectSoundFXDistortion_Vtbl, 0x8ecf4326_455f_4d8b_bda9_8d5d3e9e3e0b);
impl std::ops::Deref for IDirectSoundFXDistortion {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXDistortion, windows_core::IUnknown);
impl IDirectSoundFXDistortion {
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxdistortion).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxdistortion).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFXDistortion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXDistortion) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXDistortion) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXEcho, IDirectSoundFXEcho_Vtbl, 0x8bd28edf_50db_4e92_a2bd_445488d1ed42);
impl std::ops::Deref for IDirectSoundFXEcho {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXEcho, windows_core::IUnknown);
impl IDirectSoundFXEcho {
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxecho).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxecho: *mut DSFXEcho) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxecho).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFXEcho_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXEcho) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXEcho) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXFlanger, IDirectSoundFXFlanger_Vtbl, 0x903e9878_2c92_4072_9b2c_ea68f5396783);
impl std::ops::Deref for IDirectSoundFXFlanger {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXFlanger, windows_core::IUnknown);
impl IDirectSoundFXFlanger {
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxflanger).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxflanger: *mut DSFXFlanger) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxflanger).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFXFlanger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXFlanger) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXFlanger) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXGargle, IDirectSoundFXGargle_Vtbl, 0xd616f352_d622_11ce_aac5_0020af0b99a3);
impl std::ops::Deref for IDirectSoundFXGargle {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXGargle, windows_core::IUnknown);
impl IDirectSoundFXGargle {
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxgargle).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXGargle> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectSoundFXGargle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXGargle) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXGargle) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXI3DL2Reverb, IDirectSoundFXI3DL2Reverb_Vtbl, 0x4b166a6a_0d66_43f3_80e3_ee6280dee1a4);
impl std::ops::Deref for IDirectSoundFXI3DL2Reverb {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXI3DL2Reverb, windows_core::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxi3dl2reverb).ok()
    }
    pub unsafe fn GetAllParameters(&self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxi3dl2reverb).ok()
    }
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPreset)(windows_core::Interface::as_raw(self), dwpreset).ok()
    }
    pub unsafe fn GetPreset(&self) -> windows_core::Result<u32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuality(&self, lquality: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuality)(windows_core::Interface::as_raw(self), lquality).ok()
    }
    pub unsafe fn GetQuality(&self) -> windows_core::Result<i32> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectSoundFXI3DL2Reverb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXI3DL2Reverb) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXI3DL2Reverb) -> windows_core::HRESULT,
    pub SetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXParamEq, IDirectSoundFXParamEq_Vtbl, 0xc03ca9fe_fe90_4204_8078_82334cd177da);
impl std::ops::Deref for IDirectSoundFXParamEq {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXParamEq, windows_core::IUnknown);
impl IDirectSoundFXParamEq {
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxparameq).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXParamEq> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectSoundFXParamEq_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXParamEq) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXParamEq) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFXWavesReverb, IDirectSoundFXWavesReverb_Vtbl, 0x46858c3a_0dc6_45e3_b760_d4eef16cb325);
impl std::ops::Deref for IDirectSoundFXWavesReverb {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFXWavesReverb, windows_core::IUnknown);
impl IDirectSoundFXWavesReverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxwavesreverb).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXWavesReverb> {
        let mut result__ = std::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectSoundFXWavesReverb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXWavesReverb) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXWavesReverb) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundFullDuplex, IDirectSoundFullDuplex_Vtbl, 0xedcb4c7a_daab_4216_a42e_6c50596ddc1d);
impl std::ops::Deref for IDirectSoundFullDuplex {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundFullDuplex, windows_core::IUnknown);
impl IDirectSoundFullDuplex {
    pub unsafe fn Initialize<P0>(&self, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: P0, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut Option<IDirectSoundBuffer8>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcaptureguid, prenderguid, lpdscbufferdesc, lpdsbufferdesc, hwnd.param().abi(), dwlevel, core::mem::transmute(lplpdirectsoundcapturebuffer8), core::mem::transmute(lplpdirectsoundbuffer8)).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundFullDuplex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const DSCBUFFERDESC, *const DSBUFFERDESC, super::super::super::Foundation::HWND, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectSoundNotify, IDirectSoundNotify_Vtbl, 0xb0210783_89cd_11d0_af08_00a0c925cd16);
impl std::ops::Deref for IDirectSoundNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundNotify, windows_core::IUnknown);
impl IDirectSoundNotify {
    pub unsafe fn SetNotificationPositions(&self, pcpositionnotifies: &[DSBPOSITIONNOTIFY]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotificationPositions)(windows_core::Interface::as_raw(self), pcpositionnotifies.len().try_into().unwrap(), core::mem::transmute(pcpositionnotifies.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IDirectSoundNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNotificationPositions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DSBPOSITIONNOTIFY) -> windows_core::HRESULT,
}
pub const CLSID_DirectSound: windows_core::GUID = windows_core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: windows_core::GUID = windows_core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: windows_core::GUID = windows_core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: windows_core::GUID = windows_core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: windows_core::GUID = windows_core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
pub const DS3DALG_HRTF_FULL: windows_core::GUID = windows_core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: windows_core::GUID = windows_core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DMODE_DISABLE: u32 = 2u32;
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
pub const DS3DMODE_NORMAL: u32 = 0u32;
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
pub const DS3D_DEFERRED: u32 = 1u32;
pub const DS3D_IMMEDIATE: u32 = 0u32;
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
pub const DS3D_MINCONEANGLE: u32 = 0u32;
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
pub const DSBCAPS_CTRL3D: u32 = 16u32;
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
pub const DSBCAPS_CTRLFX: u32 = 512u32;
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
pub const DSBCAPS_STATIC: u32 = 2u32;
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
pub const DSBFREQUENCY_MIN: u32 = 100u32;
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
pub const DSBPAN_CENTER: u32 = 0u32;
pub const DSBPAN_LEFT: i32 = -10000i32;
pub const DSBPAN_RIGHT: u32 = 10000u32;
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
pub const DSBPLAY_LOOPING: u32 = 1u32;
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
pub const DSBSIZE_FX_MIN: u32 = 150u32;
pub const DSBSIZE_MAX: u32 = 268435455u32;
pub const DSBSIZE_MIN: u32 = 4u32;
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
pub const DSBSTATUS_LOOPING: u32 = 4u32;
pub const DSBSTATUS_PLAYING: u32 = 1u32;
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
pub const DSBVOLUME_MAX: u32 = 0u32;
pub const DSBVOLUME_MIN: i32 = -10000i32;
pub const DSCAPS_CERTIFIED: u32 = 64u32;
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
pub const DSCBSTART_LOOPING: u32 = 1u32;
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
pub const DSFXR_FAILED: i32 = 4i32;
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
pub const DSFXR_PRESENT: i32 = 0i32;
pub const DSFXR_SENDLOOP: i32 = 6i32;
pub const DSFXR_UNALLOCATED: i32 = 3i32;
pub const DSFXR_UNKNOWN: i32 = 5i32;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
pub const DSFX_LOCHARDWARE: u32 = 1u32;
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
pub const DSSCL_NORMAL: u32 = 1u32;
pub const DSSCL_PRIORITY: u32 = 2u32;
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
pub const DSSPEAKER_5POINT1: u32 = 6u32;
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
pub const DSSPEAKER_7POINT1: u32 = 7u32;
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
pub const DSSPEAKER_MONO: u32 = 2u32;
pub const DSSPEAKER_QUAD: u32 = 3u32;
pub const DSSPEAKER_STEREO: u32 = 4u32;
pub const DSSPEAKER_SURROUND: u32 = 5u32;
pub const DS_CERTIFIED: u32 = 0u32;
pub const DS_NO_VIRTUALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x878000A_u32 as _);
pub const DS_UNCERTIFIED: u32 = 1u32;
pub const GUID_All_Objects: windows_core::GUID = windows_core::GUID::from_u128(0xaa114de5_c262_4169_a1c8_23d698cc73b5);
pub const GUID_DSCFX_CLASS_AEC: windows_core::GUID = windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const GUID_DSCFX_CLASS_NS: windows_core::GUID = windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const GUID_DSCFX_MS_AEC: windows_core::GUID = windows_core::GUID::from_u128(0xcdebb919_379a_488a_8765_f53cfd36de40);
pub const GUID_DSCFX_MS_NS: windows_core::GUID = windows_core::GUID::from_u128(0x11c5c73b_66e9_4ba1_a0ba_e814c6eed92d);
pub const GUID_DSCFX_SYSTEM_AEC: windows_core::GUID = windows_core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const GUID_DSCFX_SYSTEM_NS: windows_core::GUID = windows_core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
pub const GUID_DSFX_STANDARD_CHORUS: windows_core::GUID = windows_core::GUID::from_u128(0xefe6629c_81f7_4281_bd91_c9d604a95af6);
pub const GUID_DSFX_STANDARD_COMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0xef011f79_4000_406d_87af_bffb3fc39d57);
pub const GUID_DSFX_STANDARD_DISTORTION: windows_core::GUID = windows_core::GUID::from_u128(0xef114c90_cd1d_484e_96e5_09cfaf912a21);
pub const GUID_DSFX_STANDARD_ECHO: windows_core::GUID = windows_core::GUID::from_u128(0xef3e932c_d40b_4f51_8ccf_3f98f1b29d5d);
pub const GUID_DSFX_STANDARD_FLANGER: windows_core::GUID = windows_core::GUID::from_u128(0xefca3d92_dfd8_4672_a603_7420894bad98);
pub const GUID_DSFX_STANDARD_GARGLE: windows_core::GUID = windows_core::GUID::from_u128(0xdafd8210_5711_4b91_9fe3_f75b7ae279bf);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: windows_core::GUID = windows_core::GUID::from_u128(0xef985e71_d5c7_42d4_ba4d_2d073e2e96f4);
pub const GUID_DSFX_STANDARD_PARAMEQ: windows_core::GUID = windows_core::GUID::from_u128(0x120ced89_3bf4_4173_a132_3cb406cf3231);
pub const GUID_DSFX_WAVES_REVERB: windows_core::GUID = windows_core::GUID::from_u128(0x87fc0268_9a55_4360_95aa_004a1d9de26c);
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
pub const _FACDS: u32 = 2168u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS3DBUFFER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub dwInsideConeAngle: u32,
    pub dwOutsideConeAngle: u32,
    pub vConeOrientation: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub lConeOutsideVolume: i32,
    pub flMinDistance: f32,
    pub flMaxDistance: f32,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for DS3DBUFFER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for DS3DBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DS3DLISTENER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientFront: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientTop: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub flDistanceFactor: f32,
    pub flRolloffFactor: f32,
    pub flDopplerFactor: f32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::TypeKind for DS3DLISTENER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for DS3DLISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
impl windows_core::TypeKind for DSBCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for DSBPOSITIONNOTIFY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub guid3DAlgorithm: windows_core::GUID,
}
impl windows_core::TypeKind for DSBUFFERDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl windows_core::TypeKind for DSBUFFERDESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMinSecondarySampleRate: u32,
    pub dwMaxSecondarySampleRate: u32,
    pub dwPrimaryBuffers: u32,
    pub dwMaxHwMixingAllBuffers: u32,
    pub dwMaxHwMixingStaticBuffers: u32,
    pub dwMaxHwMixingStreamingBuffers: u32,
    pub dwFreeHwMixingAllBuffers: u32,
    pub dwFreeHwMixingStaticBuffers: u32,
    pub dwFreeHwMixingStreamingBuffers: u32,
    pub dwMaxHw3DAllBuffers: u32,
    pub dwMaxHw3DStaticBuffers: u32,
    pub dwMaxHw3DStreamingBuffers: u32,
    pub dwFreeHw3DAllBuffers: u32,
    pub dwFreeHw3DStaticBuffers: u32,
    pub dwFreeHw3DStreamingBuffers: u32,
    pub dwTotalHwMemBytes: u32,
    pub dwFreeHwMemBytes: u32,
    pub dwMaxContigFreeHwMemBytes: u32,
    pub dwUnlockTransferRateHwBuffers: u32,
    pub dwPlayCpuOverheadSwBuffers: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for DSCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for DSCBCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: *mut DSCEFFECTDESC,
}
impl windows_core::TypeKind for DSCBUFFERDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl windows_core::TypeKind for DSCBUFFERDESC1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
impl windows_core::TypeKind for DSCCAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: windows_core::GUID,
    pub guidDSCFXInstance: windows_core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for DSCEFFECTDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCFXAec {
    pub fEnable: super::super::super::Foundation::BOOL,
    pub fNoiseFill: super::super::super::Foundation::BOOL,
    pub dwMode: u32,
}
impl windows_core::TypeKind for DSCFXAec {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCFXAec {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCFXNoiseSuppress {
    pub fEnable: super::super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DSCFXNoiseSuppress {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: windows_core::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl windows_core::TypeKind for DSEFFECTDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl windows_core::TypeKind for DSFXChorus {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXChorus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
impl windows_core::TypeKind for DSFXCompressor {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXCompressor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
impl windows_core::TypeKind for DSFXDistortion {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXDistortion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
impl windows_core::TypeKind for DSFXEcho {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXEcho {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl windows_core::TypeKind for DSFXFlanger {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXFlanger {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
impl windows_core::TypeKind for DSFXGargle {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXGargle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXI3DL2Reverb {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub lReflections: i32,
    pub flReflectionsDelay: f32,
    pub lReverb: i32,
    pub flReverbDelay: f32,
    pub flDiffusion: f32,
    pub flDensity: f32,
    pub flHFReference: f32,
}
impl windows_core::TypeKind for DSFXI3DL2Reverb {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
impl windows_core::TypeKind for DSFXParamEq {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXParamEq {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl windows_core::TypeKind for DSFXWavesReverb {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSFXWavesReverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
