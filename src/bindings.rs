#[inline]
pub unsafe fn CoInitializeEx(
    pvreserved: Option<*const core::ffi::c_void>,
    dwcoinit: u32,
) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[inline]
pub unsafe fn CoInitializeSecurity(
    psecdesc: Option<PSECURITY_DESCRIPTOR>,
    cauthsvc: i32,
    asauthsvc: Option<*const SOLE_AUTHENTICATION_SERVICE>,
    preserved1: Option<*const core::ffi::c_void>,
    dwauthnlevel: u32,
    dwimplevel: u32,
    pauthlist: Option<*const core::ffi::c_void>,
    dwcapabilities: u32,
    preserved3: Option<*const core::ffi::c_void>,
) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : u32, dwimplevel : u32, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        CoInitializeSecurity(
            psecdesc.unwrap_or(core::mem::zeroed()) as _,
            cauthsvc,
            asauthsvc.unwrap_or(core::mem::zeroed()) as _,
            preserved1.unwrap_or(core::mem::zeroed()) as _,
            dwauthnlevel,
            dwimplevel,
            pauthlist.unwrap_or(core::mem::zeroed()) as _,
            dwcapabilities,
            preserved3.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
#[inline]
pub unsafe fn CreateVssBackupComponentsInternal() -> windows_core::Result<IVssBackupComponents> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppbackup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssBackupComponentsInternal(&mut result__)
            .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn VssFreeSnapshotPropertiesInternal(pprop: *const VSS_SNAPSHOT_PROP) {
    windows_core::link!("vssapi.dll" "system" fn VssFreeSnapshotPropertiesInternal(pprop : *const VSS_SNAPSHOT_PROP));
    unsafe { VssFreeSnapshotPropertiesInternal(pprop) }
}
pub type COINIT = i32;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64;
pub type EOLE_AUTHENTICATION_CAPABILITIES = i32;
pub const E_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80070005_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
windows_core::imp::define_interface!(
    IVssAsync,
    IVssAsync_Vtbl,
    0x507c37b4_cf5b_4e95_b0af_14eb9767467e
);
windows_core::imp::interface_hierarchy!(IVssAsync, windows_core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Wait)(
                windows_core::Interface::as_raw(self),
                dwmilliseconds,
            )
        }
    }
    pub unsafe fn QueryStatus(
        &self,
        phrresult: *mut windows_core::HRESULT,
        preserved: *mut i32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).QueryStatus)(
                windows_core::Interface::as_raw(self),
                phrresult as _,
                preserved as _,
            )
        }
    }
}
#[repr(C)]
pub struct IVssAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::HRESULT,
        *mut i32,
    ) -> windows_core::HRESULT,
}
pub trait IVssAsync_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn QueryStatus(
        &self,
        phrresult: *mut windows_core::HRESULT,
        preserved: *mut i32,
    ) -> windows_core::Result<()>;
}
impl IVssAsync_Vtbl {
    pub const fn new<Identity: IVssAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IVssAsync_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Wait<Identity: IVssAsync_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwmilliseconds: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn QueryStatus<Identity: IVssAsync_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            phrresult: *mut windows_core::HRESULT,
            preserved: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::QueryStatus(
                    this,
                    core::mem::transmute_copy(&phrresult),
                    core::mem::transmute_copy(&preserved),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            QueryStatus: QueryStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssAsync {}
windows_core::imp::define_interface!(
    IVssBackupComponents,
    IVssBackupComponents_Vtbl,
    0x665c1d5f_c218_414d_a05d_7fef5f9d5c86
);
windows_core::imp::interface_hierarchy!(IVssBackupComponents, windows_core::IUnknown);
impl IVssBackupComponents {
    pub unsafe fn GetWriterComponentsCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterComponentsCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetWriterComponents(
        &self,
        iwriter: u32,
    ) -> windows_core::Result<IVssWriterComponentsExt> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterComponents)(
                windows_core::Interface::as_raw(self),
                iwriter,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn InitializeForBackup(
        &self,
        bstrxml: &windows_core::BSTR,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeForBackup)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(bstrxml),
            )
        }
    }
    pub unsafe fn SetBackupState(
        &self,
        bselectcomponents: bool,
        bbackupbootablesystemstate: bool,
        backuptype: VSS_BACKUP_TYPE,
        bpartialfilesupport: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackupState)(
                windows_core::Interface::as_raw(self),
                bselectcomponents,
                bbackupbootablesystemstate,
                backuptype,
                bpartialfilesupport,
            )
        }
    }
    pub unsafe fn InitializeForRestore(
        &self,
        bstrxml: &windows_core::BSTR,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeForRestore)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(bstrxml),
            )
        }
    }
    pub unsafe fn SetRestoreState(&self, restoretype: VSS_RESTORE_TYPE) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetRestoreState)(
                windows_core::Interface::as_raw(self),
                restoretype,
            )
        }
    }
    pub unsafe fn GatherWriterMetadata(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GatherWriterMetadata)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriterMetadataCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterMetadataCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetWriterMetadata(
        &self,
        iwriter: u32,
        pidinstance: *mut VSS_ID,
        ppmetadata: *mut Option<IVssExamineWriterMetadata>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetWriterMetadata)(
                windows_core::Interface::as_raw(self),
                iwriter,
                pidinstance as _,
                core::mem::transmute(ppmetadata),
            )
        }
    }
    pub unsafe fn FreeWriterMetadata(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FreeWriterMetadata)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn AddComponent<P3, P4>(
        &self,
        instanceid: VSS_ID,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P3,
        wszcomponentname: P4,
    ) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddComponent)(
                windows_core::Interface::as_raw(self),
                instanceid,
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
            )
        }
    }
    pub unsafe fn PrepareForBackup(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrepareForBackup)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AbortBackup(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).AbortBackup)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GatherWriterStatus(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GatherWriterStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriterStatusCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterStatusCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn FreeWriterStatus(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FreeWriterStatus)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetWriterStatus(
        &self,
        iwriter: u32,
        pidinstance: *mut VSS_ID,
        pidwriter: *mut VSS_ID,
        pbstrwriter: *mut windows_core::BSTR,
        pnstatus: *mut VSS_WRITER_STATE,
        phresultfailure: *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetWriterStatus)(
                windows_core::Interface::as_raw(self),
                iwriter,
                pidinstance as _,
                pidwriter as _,
                core::mem::transmute(pbstrwriter),
                pnstatus as _,
                phresultfailure as _,
            )
        }
    }
    pub unsafe fn SetBackupSucceeded<P3, P4>(
        &self,
        instanceid: VSS_ID,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P3,
        wszcomponentname: P4,
        bsucceded: bool,
    ) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackupSucceeded)(
                windows_core::Interface::as_raw(self),
                instanceid,
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                bsucceded,
            )
        }
    }
    pub unsafe fn SetBackupOptions<P2, P3, P4>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszbackupoptions: P4,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackupOptions)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszbackupoptions.param().abi(),
            )
        }
    }
    pub unsafe fn SetSelectedForRestore<P2, P3>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        bselectedforrestore: bool,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSelectedForRestore)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                bselectedforrestore,
            )
        }
    }
    pub unsafe fn SetRestoreOptions<P2, P3, P4>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszrestoreoptions: P4,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetRestoreOptions)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszrestoreoptions.param().abi(),
            )
        }
    }
    pub unsafe fn SetAdditionalRestores<P2, P3>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        badditionalrestores: bool,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetAdditionalRestores)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                badditionalrestores,
            )
        }
    }
    pub unsafe fn SetPreviousBackupStamp<P2, P3, P4>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszpreviousbackupstamp: P4,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreviousBackupStamp)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszpreviousbackupstamp.param().abi(),
            )
        }
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SaveAsXML)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pbstrxml),
            )
        }
    }
    pub unsafe fn BackupComplete(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupComplete)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddAlternativeLocationMapping<P2, P3, P4, P5, P7>(
        &self,
        writerid: VSS_ID,
        componenttype: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszpath: P4,
        wszfilespec: P5,
        brecursive: bool,
        wszdestination: P7,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddAlternativeLocationMapping)(
                windows_core::Interface::as_raw(self),
                writerid,
                componenttype,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszpath.param().abi(),
                wszfilespec.param().abi(),
                brecursive,
                wszdestination.param().abi(),
            )
        }
    }
    pub unsafe fn AddRestoreSubcomponent<P2, P3, P4, P5>(
        &self,
        writerid: VSS_ID,
        componenttype: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszsubcomponentlogicalpath: P4,
        wszsubcomponentname: P5,
        brepair: bool,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddRestoreSubcomponent)(
                windows_core::Interface::as_raw(self),
                writerid,
                componenttype,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszsubcomponentlogicalpath.param().abi(),
                wszsubcomponentname.param().abi(),
                brepair,
            )
        }
    }
    pub unsafe fn SetFileRestoreStatus<P2, P3>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        status: VSS_FILE_RESTORE_STATUS,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileRestoreStatus)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                status,
            )
        }
    }
    pub unsafe fn AddNewTarget<P2, P3, P4, P5, P7>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        wszpath: P4,
        wszfilename: P5,
        brecursive: bool,
        wszalternatepath: P7,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddNewTarget)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                wszpath.param().abi(),
                wszfilename.param().abi(),
                brecursive,
                wszalternatepath.param().abi(),
            )
        }
    }
    pub unsafe fn SetRangesFilePath<P2, P3, P5>(
        &self,
        writerid: VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: P2,
        wszcomponentname: P3,
        ipartialfile: u32,
        wszrangesfile: P5,
    ) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetRangesFilePath)(
                windows_core::Interface::as_raw(self),
                writerid,
                ct,
                wszlogicalpath.param().abi(),
                wszcomponentname.param().abi(),
                ipartialfile,
                wszrangesfile.param().abi(),
            )
        }
    }
    pub unsafe fn PreRestore(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreRestore)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn PostRestore(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostRestore)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetContext)(
                windows_core::Interface::as_raw(self),
                lcontext,
            )
        }
    }
    pub unsafe fn StartSnapshotSet(&self) -> windows_core::Result<VSS_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartSnapshotSet)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn AddToSnapshotSet<P0>(
        &self,
        pwszvolumename: P0,
        providerid: VSS_ID,
    ) -> windows_core::Result<VSS_ID>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddToSnapshotSet)(
                windows_core::Interface::as_raw(self),
                pwszvolumename.param().abi(),
                providerid,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn DoSnapshotSet(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoSnapshotSet)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteSnapshots(
        &self,
        sourceobjectid: VSS_ID,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: bool,
        pldeletedsnapshots: *const i32,
        pnondeletedsnapshotid: *const VSS_ID,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteSnapshots)(
                windows_core::Interface::as_raw(self),
                sourceobjectid,
                esourceobjecttype,
                bforcedelete.into(),
                pldeletedsnapshots,
                pnondeletedsnapshotid,
            )
        }
    }
    pub unsafe fn ImportSnapshots(&self) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportSnapshots)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BreakSnapshotSet(&self, snapshotsetid: VSS_ID) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).BreakSnapshotSet)(
                windows_core::Interface::as_raw(self),
                snapshotsetid,
            )
        }
    }
    pub unsafe fn GetSnapshotProperties(
        &self,
        snapshotid: VSS_ID,
        pprop: *mut VSS_SNAPSHOT_PROP,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetSnapshotProperties)(
                windows_core::Interface::as_raw(self),
                snapshotid,
                pprop as _,
            )
        }
    }
    pub unsafe fn Query(
        &self,
        queriedobjectid: VSS_ID,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
        ppenum: *const Option<IVssEnumObject>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Query)(
                windows_core::Interface::as_raw(self),
                queriedobjectid,
                equeriedobjecttype,
                ereturnedobjectstype,
                core::mem::transmute(ppenum),
            )
        }
    }
    pub unsafe fn IsVolumeSupported<P1>(
        &self,
        providerid: VSS_ID,
        pwszvolumename: P1,
        pbsupportedbythisprovider: *const windows_core::BOOL,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IsVolumeSupported)(
                windows_core::Interface::as_raw(self),
                providerid,
                pwszvolumename.param().abi(),
                pbsupportedbythisprovider,
            )
        }
    }
    pub unsafe fn DisableWriterClasses(
        &self,
        rgwriterclassid: *const VSS_ID,
        cclassid: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DisableWriterClasses)(
                windows_core::Interface::as_raw(self),
                rgwriterclassid,
                cclassid,
            )
        }
    }
    pub unsafe fn EnableWriterClasses(
        &self,
        rgwriterclassid: *const VSS_ID,
        cclassid: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EnableWriterClasses)(
                windows_core::Interface::as_raw(self),
                rgwriterclassid,
                cclassid,
            )
        }
    }
    pub unsafe fn DisableWriterInstances(
        &self,
        rgwriterinstanceid: *const VSS_ID,
        cinstanceid: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).DisableWriterInstances)(
                windows_core::Interface::as_raw(self),
                rgwriterinstanceid,
                cinstanceid,
            )
        }
    }
    pub unsafe fn ExposeSnapshot<P1, P3>(
        &self,
        snapshotid: VSS_ID,
        wszpathfromroot: P1,
        lattributes: i32,
        wszexpose: P3,
    ) -> windows_core::Result<VSS_PWSZ>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExposeSnapshot)(
                windows_core::Interface::as_raw(self),
                snapshotid,
                wszpathfromroot.param().abi(),
                lattributes,
                wszexpose.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RevertToSnapshot(
        &self,
        snapshotid: VSS_ID,
        bforcedismount: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RevertToSnapshot)(
                windows_core::Interface::as_raw(self),
                snapshotid,
                bforcedismount.into(),
            )
        }
    }
    pub unsafe fn QueryRevertStatus<P0>(&self, pwszvolume: P0) -> windows_core::Result<IVssAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryRevertStatus)(
                windows_core::Interface::as_raw(self),
                pwszvolume.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IVssBackupComponents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWriterComponentsCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWriterComponents: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InitializeForBackup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBackupState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        bool,
        bool,
        VSS_BACKUP_TYPE,
        bool,
    ) -> windows_core::HRESULT,
    pub InitializeForRestore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetRestoreState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_RESTORE_TYPE,
    ) -> windows_core::HRESULT,
    pub GatherWriterMetadata: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWriterMetadataCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWriterMetadata: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut VSS_ID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FreeWriterMetadata:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub PrepareForBackup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AbortBackup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GatherWriterStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetWriterStatusCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreeWriterStatus:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWriterStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut VSS_ID,
        *mut VSS_ID,
        *mut *mut core::ffi::c_void,
        *mut VSS_WRITER_STATE,
        *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub SetBackupSucceeded: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
    ) -> windows_core::HRESULT,
    pub SetBackupOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetSelectedForRestore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
    ) -> windows_core::HRESULT,
    pub SetRestoreOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetAdditionalRestores: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
    ) -> windows_core::HRESULT,
    pub SetPreviousBackupStamp: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SaveAsXML: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BackupComplete: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddAlternativeLocationMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub AddRestoreSubcomponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
    ) -> windows_core::HRESULT,
    pub SetFileRestoreStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        VSS_FILE_RESTORE_STATUS,
    ) -> windows_core::HRESULT,
    pub AddNewTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        bool,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetRangesFilePath: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_COMPONENT_TYPE,
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        u32,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub PreRestore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PostRestore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartSnapshotSet:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_ID) -> windows_core::HRESULT,
    pub AddToSnapshotSet: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        VSS_ID,
        *mut VSS_ID,
    ) -> windows_core::HRESULT,
    pub DoSnapshotSet: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DeleteSnapshots: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_OBJECT_TYPE,
        windows_core::BOOL,
        *const i32,
        *const VSS_ID,
    ) -> windows_core::HRESULT,
    pub ImportSnapshots: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BreakSnapshotSet:
        unsafe extern "system" fn(*mut core::ffi::c_void, VSS_ID) -> windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        *mut VSS_SNAPSHOT_PROP,
    ) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        VSS_OBJECT_TYPE,
        VSS_OBJECT_TYPE,
        *const *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsVolumeSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        windows_core::PCWSTR,
        *const windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub DisableWriterClasses: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const VSS_ID,
        u32,
    ) -> windows_core::HRESULT,
    pub EnableWriterClasses: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const VSS_ID,
        u32,
    ) -> windows_core::HRESULT,
    pub DisableWriterInstances: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const VSS_ID,
        u32,
    ) -> windows_core::HRESULT,
    pub ExposeSnapshot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        windows_core::PCWSTR,
        i32,
        windows_core::PCWSTR,
        *mut VSS_PWSZ,
    ) -> windows_core::HRESULT,
    pub RevertToSnapshot: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VSS_ID,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IVssBackupComponents_Impl: windows_core::IUnknownImpl {
    fn GetWriterComponentsCount(&self) -> windows_core::Result<u32>;
    fn GetWriterComponents(&self, iwriter: u32) -> windows_core::Result<IVssWriterComponentsExt>;
    fn InitializeForBackup(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBackupState(
        &self,
        bselectcomponents: bool,
        bbackupbootablesystemstate: bool,
        backuptype: VSS_BACKUP_TYPE,
        bpartialfilesupport: bool,
    ) -> windows_core::Result<()>;
    fn InitializeForRestore(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreState(&self, restoretype: VSS_RESTORE_TYPE) -> windows_core::Result<()>;
    fn GatherWriterMetadata(&self) -> windows_core::Result<IVssAsync>;
    fn GetWriterMetadataCount(&self) -> windows_core::Result<u32>;
    fn GetWriterMetadata(
        &self,
        iwriter: u32,
        pidinstance: *mut VSS_ID,
        ppmetadata: windows_core::OutRef<IVssExamineWriterMetadata>,
    ) -> windows_core::Result<()>;
    fn FreeWriterMetadata(&self) -> windows_core::Result<()>;
    fn AddComponent(
        &self,
        instanceid: &VSS_ID,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn PrepareForBackup(&self) -> windows_core::Result<IVssAsync>;
    fn AbortBackup(&self) -> windows_core::Result<()>;
    fn GatherWriterStatus(&self) -> windows_core::Result<IVssAsync>;
    fn GetWriterStatusCount(&self) -> windows_core::Result<u32>;
    fn FreeWriterStatus(&self) -> windows_core::Result<()>;
    fn GetWriterStatus(
        &self,
        iwriter: u32,
        pidinstance: *mut VSS_ID,
        pidwriter: *mut VSS_ID,
        pbstrwriter: *mut windows_core::BSTR,
        pnstatus: *mut VSS_WRITER_STATE,
        phresultfailure: *mut windows_core::HRESULT,
    ) -> windows_core::Result<()>;
    fn SetBackupSucceeded(
        &self,
        instanceid: &VSS_ID,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        bsucceded: bool,
    ) -> windows_core::Result<()>;
    fn SetBackupOptions(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszbackupoptions: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetSelectedForRestore(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        bselectedforrestore: bool,
    ) -> windows_core::Result<()>;
    fn SetRestoreOptions(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszrestoreoptions: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetAdditionalRestores(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        badditionalrestores: bool,
    ) -> windows_core::Result<()>;
    fn SetPreviousBackupStamp(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszpreviousbackupstamp: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::Result<()>;
    fn BackupComplete(&self) -> windows_core::Result<IVssAsync>;
    fn AddAlternativeLocationMapping(
        &self,
        writerid: &VSS_ID,
        componenttype: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszpath: &windows_core::PCWSTR,
        wszfilespec: &windows_core::PCWSTR,
        brecursive: bool,
        wszdestination: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn AddRestoreSubcomponent(
        &self,
        writerid: &VSS_ID,
        componenttype: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszsubcomponentlogicalpath: &windows_core::PCWSTR,
        wszsubcomponentname: &windows_core::PCWSTR,
        brepair: bool,
    ) -> windows_core::Result<()>;
    fn SetFileRestoreStatus(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        status: VSS_FILE_RESTORE_STATUS,
    ) -> windows_core::Result<()>;
    fn AddNewTarget(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        wszpath: &windows_core::PCWSTR,
        wszfilename: &windows_core::PCWSTR,
        brecursive: bool,
        wszalternatepath: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetRangesFilePath(
        &self,
        writerid: &VSS_ID,
        ct: VSS_COMPONENT_TYPE,
        wszlogicalpath: &windows_core::PCWSTR,
        wszcomponentname: &windows_core::PCWSTR,
        ipartialfile: u32,
        wszrangesfile: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn PreRestore(&self) -> windows_core::Result<IVssAsync>;
    fn PostRestore(&self) -> windows_core::Result<IVssAsync>;
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn StartSnapshotSet(&self) -> windows_core::Result<VSS_ID>;
    fn AddToSnapshotSet(
        &self,
        pwszvolumename: &windows_core::PCWSTR,
        providerid: &VSS_ID,
    ) -> windows_core::Result<VSS_ID>;
    fn DoSnapshotSet(&self) -> windows_core::Result<IVssAsync>;
    fn DeleteSnapshots(
        &self,
        sourceobjectid: &VSS_ID,
        esourceobjecttype: VSS_OBJECT_TYPE,
        bforcedelete: windows_core::BOOL,
        pldeletedsnapshots: *const i32,
        pnondeletedsnapshotid: *const VSS_ID,
    ) -> windows_core::Result<()>;
    fn ImportSnapshots(&self) -> windows_core::Result<IVssAsync>;
    fn BreakSnapshotSet(&self, snapshotsetid: &VSS_ID) -> windows_core::Result<()>;
    fn GetSnapshotProperties(
        &self,
        snapshotid: &VSS_ID,
        pprop: *mut VSS_SNAPSHOT_PROP,
    ) -> windows_core::Result<()>;
    fn Query(
        &self,
        queriedobjectid: &VSS_ID,
        equeriedobjecttype: VSS_OBJECT_TYPE,
        ereturnedobjectstype: VSS_OBJECT_TYPE,
        ppenum: *const Option<IVssEnumObject>,
    ) -> windows_core::Result<()>;
    fn IsVolumeSupported(
        &self,
        providerid: &VSS_ID,
        pwszvolumename: &windows_core::PCWSTR,
        pbsupportedbythisprovider: *const windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn DisableWriterClasses(
        &self,
        rgwriterclassid: *const VSS_ID,
        cclassid: u32,
    ) -> windows_core::Result<()>;
    fn EnableWriterClasses(
        &self,
        rgwriterclassid: *const VSS_ID,
        cclassid: u32,
    ) -> windows_core::Result<()>;
    fn DisableWriterInstances(
        &self,
        rgwriterinstanceid: *const VSS_ID,
        cinstanceid: u32,
    ) -> windows_core::Result<()>;
    fn ExposeSnapshot(
        &self,
        snapshotid: &VSS_ID,
        wszpathfromroot: &windows_core::PCWSTR,
        lattributes: i32,
        wszexpose: &windows_core::PCWSTR,
    ) -> windows_core::Result<VSS_PWSZ>;
    fn RevertToSnapshot(
        &self,
        snapshotid: &VSS_ID,
        bforcedismount: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn QueryRevertStatus(
        &self,
        pwszvolume: &windows_core::PCWSTR,
    ) -> windows_core::Result<IVssAsync>;
}
impl IVssBackupComponents_Vtbl {
    pub const fn new<Identity: IVssBackupComponents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterComponentsCount<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pccomponents: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterComponentsCount(this) {
                    Ok(ok__) => {
                        pccomponents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterComponents<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            iwriter: u32,
            ppwriter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterComponents(
                    this,
                    core::mem::transmute_copy(&iwriter),
                ) {
                    Ok(ok__) => {
                        ppwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeForBackup<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bstrxml: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::InitializeForBackup(this, core::mem::transmute(&bstrxml))
                    .into()
            }
        }
        unsafe extern "system" fn SetBackupState<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bselectcomponents: bool,
            bbackupbootablesystemstate: bool,
            backuptype: VSS_BACKUP_TYPE,
            bpartialfilesupport: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupState(
                    this,
                    core::mem::transmute_copy(&bselectcomponents),
                    core::mem::transmute_copy(&bbackupbootablesystemstate),
                    core::mem::transmute_copy(&backuptype),
                    core::mem::transmute_copy(&bpartialfilesupport),
                )
                .into()
            }
        }
        unsafe extern "system" fn InitializeForRestore<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bstrxml: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::InitializeForRestore(
                    this,
                    core::mem::transmute(&bstrxml),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetRestoreState<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            restoretype: VSS_RESTORE_TYPE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreState(
                    this,
                    core::mem::transmute_copy(&restoretype),
                )
                .into()
            }
        }
        unsafe extern "system" fn GatherWriterMetadata<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GatherWriterMetadata(this) {
                    Ok(ok__) => {
                        pasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterMetadataCount<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcwriters: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterMetadataCount(this) {
                    Ok(ok__) => {
                        pcwriters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterMetadata<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            iwriter: u32,
            pidinstance: *mut VSS_ID,
            ppmetadata: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterMetadata(
                    this,
                    core::mem::transmute_copy(&iwriter),
                    core::mem::transmute_copy(&pidinstance),
                    core::mem::transmute_copy(&ppmetadata),
                )
                .into()
            }
        }
        unsafe extern "system" fn FreeWriterMetadata<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::FreeWriterMetadata(this).into()
            }
        }
        unsafe extern "system" fn AddComponent<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instanceid: VSS_ID,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddComponent(
                    this,
                    core::mem::transmute(&instanceid),
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                )
                .into()
            }
        }
        unsafe extern "system" fn PrepareForBackup<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PrepareForBackup(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbortBackup<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AbortBackup(this).into()
            }
        }
        unsafe extern "system" fn GatherWriterStatus<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GatherWriterStatus(this) {
                    Ok(ok__) => {
                        pasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterStatusCount<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcwriters: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterStatusCount(this) {
                    Ok(ok__) => {
                        pcwriters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeWriterStatus<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::FreeWriterStatus(this).into()
            }
        }
        unsafe extern "system" fn GetWriterStatus<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            iwriter: u32,
            pidinstance: *mut VSS_ID,
            pidwriter: *mut VSS_ID,
            pbstrwriter: *mut *mut core::ffi::c_void,
            pnstatus: *mut VSS_WRITER_STATE,
            phresultfailure: *mut windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterStatus(
                    this,
                    core::mem::transmute_copy(&iwriter),
                    core::mem::transmute_copy(&pidinstance),
                    core::mem::transmute_copy(&pidwriter),
                    core::mem::transmute_copy(&pbstrwriter),
                    core::mem::transmute_copy(&pnstatus),
                    core::mem::transmute_copy(&phresultfailure),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetBackupSucceeded<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            instanceid: VSS_ID,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            bsucceded: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupSucceeded(
                    this,
                    core::mem::transmute(&instanceid),
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute_copy(&bsucceded),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetBackupOptions<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszbackupoptions: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupOptions(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszbackupoptions),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetSelectedForRestore<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            bselectedforrestore: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetSelectedForRestore(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute_copy(&bselectedforrestore),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetRestoreOptions<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszrestoreoptions: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreOptions(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszrestoreoptions),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetAdditionalRestores<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            badditionalrestores: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetAdditionalRestores(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute_copy(&badditionalrestores),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPreviousBackupStamp<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszpreviousbackupstamp: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetPreviousBackupStamp(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszpreviousbackupstamp),
                )
                .into()
            }
        }
        unsafe extern "system" fn SaveAsXML<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbstrxml: *const *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SaveAsXML(this, core::mem::transmute_copy(&pbstrxml))
                    .into()
            }
        }
        unsafe extern "system" fn BackupComplete<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::BackupComplete(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAlternativeLocationMapping<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            componenttype: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszpath: windows_core::PCWSTR,
            wszfilespec: windows_core::PCWSTR,
            brecursive: bool,
            wszdestination: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddAlternativeLocationMapping(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&componenttype),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszpath),
                    core::mem::transmute(&wszfilespec),
                    core::mem::transmute_copy(&brecursive),
                    core::mem::transmute(&wszdestination),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddRestoreSubcomponent<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            componenttype: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszsubcomponentlogicalpath: windows_core::PCWSTR,
            wszsubcomponentname: windows_core::PCWSTR,
            brepair: bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddRestoreSubcomponent(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&componenttype),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszsubcomponentlogicalpath),
                    core::mem::transmute(&wszsubcomponentname),
                    core::mem::transmute_copy(&brepair),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFileRestoreStatus<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            status: VSS_FILE_RESTORE_STATUS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetFileRestoreStatus(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute_copy(&status),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddNewTarget<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            wszpath: windows_core::PCWSTR,
            wszfilename: windows_core::PCWSTR,
            brecursive: bool,
            wszalternatepath: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddNewTarget(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute(&wszpath),
                    core::mem::transmute(&wszfilename),
                    core::mem::transmute_copy(&brecursive),
                    core::mem::transmute(&wszalternatepath),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetRangesFilePath<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            writerid: VSS_ID,
            ct: VSS_COMPONENT_TYPE,
            wszlogicalpath: windows_core::PCWSTR,
            wszcomponentname: windows_core::PCWSTR,
            ipartialfile: u32,
            wszrangesfile: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRangesFilePath(
                    this,
                    core::mem::transmute(&writerid),
                    core::mem::transmute_copy(&ct),
                    core::mem::transmute(&wszlogicalpath),
                    core::mem::transmute(&wszcomponentname),
                    core::mem::transmute_copy(&ipartialfile),
                    core::mem::transmute(&wszrangesfile),
                )
                .into()
            }
        }
        unsafe extern "system" fn PreRestore<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PreRestore(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PostRestore<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PostRestore(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContext<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            lcontext: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetContext(this, core::mem::transmute_copy(&lcontext))
                    .into()
            }
        }
        unsafe extern "system" fn StartSnapshotSet<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psnapshotsetid: *mut VSS_ID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::StartSnapshotSet(this) {
                    Ok(ok__) => {
                        psnapshotsetid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddToSnapshotSet<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pwszvolumename: windows_core::PCWSTR,
            providerid: VSS_ID,
            pidsnapshot: *mut VSS_ID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::AddToSnapshotSet(
                    this,
                    core::mem::transmute(&pwszvolumename),
                    core::mem::transmute(&providerid),
                ) {
                    Ok(ok__) => {
                        pidsnapshot.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoSnapshotSet<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::DoSnapshotSet(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSnapshots<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sourceobjectid: VSS_ID,
            esourceobjecttype: VSS_OBJECT_TYPE,
            bforcedelete: windows_core::BOOL,
            pldeletedsnapshots: *const i32,
            pnondeletedsnapshotid: *const VSS_ID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DeleteSnapshots(
                    this,
                    core::mem::transmute(&sourceobjectid),
                    core::mem::transmute_copy(&esourceobjecttype),
                    core::mem::transmute_copy(&bforcedelete),
                    core::mem::transmute_copy(&pldeletedsnapshots),
                    core::mem::transmute_copy(&pnondeletedsnapshotid),
                )
                .into()
            }
        }
        unsafe extern "system" fn ImportSnapshots<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::ImportSnapshots(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BreakSnapshotSet<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            snapshotsetid: VSS_ID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::BreakSnapshotSet(
                    this,
                    core::mem::transmute(&snapshotsetid),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            snapshotid: VSS_ID,
            pprop: *mut VSS_SNAPSHOT_PROP,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetSnapshotProperties(
                    this,
                    core::mem::transmute(&snapshotid),
                    core::mem::transmute_copy(&pprop),
                )
                .into()
            }
        }
        unsafe extern "system" fn Query<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            queriedobjectid: VSS_ID,
            equeriedobjecttype: VSS_OBJECT_TYPE,
            ereturnedobjectstype: VSS_OBJECT_TYPE,
            ppenum: *const *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::Query(
                    this,
                    core::mem::transmute(&queriedobjectid),
                    core::mem::transmute_copy(&equeriedobjecttype),
                    core::mem::transmute_copy(&ereturnedobjectstype),
                    core::mem::transmute_copy(&ppenum),
                )
                .into()
            }
        }
        unsafe extern "system" fn IsVolumeSupported<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            providerid: VSS_ID,
            pwszvolumename: windows_core::PCWSTR,
            pbsupportedbythisprovider: *const windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::IsVolumeSupported(
                    this,
                    core::mem::transmute(&providerid),
                    core::mem::transmute(&pwszvolumename),
                    core::mem::transmute_copy(&pbsupportedbythisprovider),
                )
                .into()
            }
        }
        unsafe extern "system" fn DisableWriterClasses<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rgwriterclassid: *const VSS_ID,
            cclassid: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterClasses(
                    this,
                    core::mem::transmute_copy(&rgwriterclassid),
                    core::mem::transmute_copy(&cclassid),
                )
                .into()
            }
        }
        unsafe extern "system" fn EnableWriterClasses<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rgwriterclassid: *const VSS_ID,
            cclassid: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::EnableWriterClasses(
                    this,
                    core::mem::transmute_copy(&rgwriterclassid),
                    core::mem::transmute_copy(&cclassid),
                )
                .into()
            }
        }
        unsafe extern "system" fn DisableWriterInstances<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rgwriterinstanceid: *const VSS_ID,
            cinstanceid: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterInstances(
                    this,
                    core::mem::transmute_copy(&rgwriterinstanceid),
                    core::mem::transmute_copy(&cinstanceid),
                )
                .into()
            }
        }
        unsafe extern "system" fn ExposeSnapshot<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            snapshotid: VSS_ID,
            wszpathfromroot: windows_core::PCWSTR,
            lattributes: i32,
            wszexpose: windows_core::PCWSTR,
            pwszexposed: *mut VSS_PWSZ,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::ExposeSnapshot(
                    this,
                    core::mem::transmute(&snapshotid),
                    core::mem::transmute(&wszpathfromroot),
                    core::mem::transmute_copy(&lattributes),
                    core::mem::transmute(&wszexpose),
                ) {
                    Ok(ok__) => {
                        pwszexposed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevertToSnapshot<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            snapshotid: VSS_ID,
            bforcedismount: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::RevertToSnapshot(
                    this,
                    core::mem::transmute(&snapshotid),
                    core::mem::transmute_copy(&bforcedismount),
                )
                .into()
            }
        }
        unsafe extern "system" fn QueryRevertStatus<
            Identity: IVssBackupComponents_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pwszvolume: windows_core::PCWSTR,
            ppasync: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::QueryRevertStatus(
                    this,
                    core::mem::transmute(&pwszvolume),
                ) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterComponentsCount: GetWriterComponentsCount::<Identity, OFFSET>,
            GetWriterComponents: GetWriterComponents::<Identity, OFFSET>,
            InitializeForBackup: InitializeForBackup::<Identity, OFFSET>,
            SetBackupState: SetBackupState::<Identity, OFFSET>,
            InitializeForRestore: InitializeForRestore::<Identity, OFFSET>,
            SetRestoreState: SetRestoreState::<Identity, OFFSET>,
            GatherWriterMetadata: GatherWriterMetadata::<Identity, OFFSET>,
            GetWriterMetadataCount: GetWriterMetadataCount::<Identity, OFFSET>,
            GetWriterMetadata: GetWriterMetadata::<Identity, OFFSET>,
            FreeWriterMetadata: FreeWriterMetadata::<Identity, OFFSET>,
            AddComponent: AddComponent::<Identity, OFFSET>,
            PrepareForBackup: PrepareForBackup::<Identity, OFFSET>,
            AbortBackup: AbortBackup::<Identity, OFFSET>,
            GatherWriterStatus: GatherWriterStatus::<Identity, OFFSET>,
            GetWriterStatusCount: GetWriterStatusCount::<Identity, OFFSET>,
            FreeWriterStatus: FreeWriterStatus::<Identity, OFFSET>,
            GetWriterStatus: GetWriterStatus::<Identity, OFFSET>,
            SetBackupSucceeded: SetBackupSucceeded::<Identity, OFFSET>,
            SetBackupOptions: SetBackupOptions::<Identity, OFFSET>,
            SetSelectedForRestore: SetSelectedForRestore::<Identity, OFFSET>,
            SetRestoreOptions: SetRestoreOptions::<Identity, OFFSET>,
            SetAdditionalRestores: SetAdditionalRestores::<Identity, OFFSET>,
            SetPreviousBackupStamp: SetPreviousBackupStamp::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
            BackupComplete: BackupComplete::<Identity, OFFSET>,
            AddAlternativeLocationMapping: AddAlternativeLocationMapping::<Identity, OFFSET>,
            AddRestoreSubcomponent: AddRestoreSubcomponent::<Identity, OFFSET>,
            SetFileRestoreStatus: SetFileRestoreStatus::<Identity, OFFSET>,
            AddNewTarget: AddNewTarget::<Identity, OFFSET>,
            SetRangesFilePath: SetRangesFilePath::<Identity, OFFSET>,
            PreRestore: PreRestore::<Identity, OFFSET>,
            PostRestore: PostRestore::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            StartSnapshotSet: StartSnapshotSet::<Identity, OFFSET>,
            AddToSnapshotSet: AddToSnapshotSet::<Identity, OFFSET>,
            DoSnapshotSet: DoSnapshotSet::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            ImportSnapshots: ImportSnapshots::<Identity, OFFSET>,
            BreakSnapshotSet: BreakSnapshotSet::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, OFFSET>,
            DisableWriterClasses: DisableWriterClasses::<Identity, OFFSET>,
            EnableWriterClasses: EnableWriterClasses::<Identity, OFFSET>,
            DisableWriterInstances: DisableWriterInstances::<Identity, OFFSET>,
            ExposeSnapshot: ExposeSnapshot::<Identity, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssBackupComponents {}
windows_core::imp::define_interface!(
    IVssEnumObject,
    IVssEnumObject_Vtbl,
    0xae1c7110_2f60_11d3_8a39_00c04f72d8e3
);
windows_core::imp::interface_hierarchy!(IVssEnumObject, windows_core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut VSS_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Next)(
                windows_core::Interface::as_raw(self),
                celt,
                rgelt as _,
                pceltfetched as _,
            )
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Skip)(
                windows_core::Interface::as_raw(self),
                celt,
            )
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Clone)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppenum),
            )
        }
    }
}
#[repr(C)]
pub struct IVssEnumObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut VSS_OBJECT_PROP,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IVssEnumObject_Impl: windows_core::IUnknownImpl {
    fn Next(
        &self,
        celt: u32,
        rgelt: *mut VSS_OBJECT_PROP,
        pceltfetched: *mut u32,
    ) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IVssEnumObject>) -> windows_core::Result<()>;
}
impl IVssEnumObject_Vtbl {
    pub const fn new<Identity: IVssEnumObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IVssEnumObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
            rgelt: *mut VSS_OBJECT_PROP,
            pceltfetched: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Next(
                    this,
                    core::mem::transmute_copy(&celt),
                    core::mem::transmute_copy(&rgelt),
                    core::mem::transmute_copy(&pceltfetched),
                )
                .into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IVssEnumObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            celt: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IVssEnumObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IVssEnumObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssEnumObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssEnumObject {}
windows_core::imp::define_interface!(
    IVssExamineWriterMetadata,
    IVssExamineWriterMetadata_Vtbl,
    0x902fcf7f_b7fd_42f8_81f1_b2e400b1e5bd
);
windows_core::imp::interface_hierarchy!(IVssExamineWriterMetadata, windows_core::IUnknown);
#[repr(C)]
pub struct IVssExamineWriterMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetIdentity: usize,
    GetFileCounts: usize,
    GetIncludeFile: usize,
    GetExcludeFile: usize,
    GetComponent: usize,
    GetRestoreMethod: usize,
    GetAlternateLocationMapping: usize,
    GetBackupSchema: usize,
    GetDocument: usize,
    SaveAsXML: usize,
    LoadFromXML: usize,
}
impl windows_core::RuntimeName for IVssExamineWriterMetadata {}
windows_core::imp::define_interface!(IVssWriterComponents, IVssWriterComponents_Vtbl);
#[repr(C)]
pub struct IVssWriterComponents_Vtbl {
    GetComponentCount: usize,
    GetWriterInfo: usize,
    GetComponent: usize,
}
windows_core::imp::define_interface!(IVssWriterComponentsExt, IVssWriterComponentsExt_Vtbl);
impl core::ops::Deref for IVssWriterComponentsExt {
    type Target = IVssWriterComponents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssWriterComponentsExt, IVssWriterComponents);
#[repr(C)]
pub struct IVssWriterComponentsExt_Vtbl {
    pub base__: IVssWriterComponents_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OLECHAR(pub u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSECURITY_DESCRIPTOR(pub *mut core::ffi::c_void);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: i32 = 6;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: i32 = 3;
pub const RPC_E_CHANGED_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80010106_u32 as _);
pub const RPC_E_TOO_LATE: windows_core::HRESULT = windows_core::HRESULT(0x80010119_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: *mut OLECHAR,
    pub hr: windows_core::HRESULT,
}
pub type VSS_BACKUP_TYPE = i32;
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = 5;
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = 3;
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = 1;
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = 2;
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = 4;
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = 6;
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = 0;
pub type VSS_COMPONENT_TYPE = i32;
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = -1;
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 9;
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = 0;
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = 29;
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = 13;
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = 16;
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 25;
pub const VSS_E_BAD_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80042301_u32 as _);
pub const VSS_E_FLUSH_WRITES_TIMEOUT: windows_core::HRESULT =
    windows_core::HRESULT(0x80042313_u32 as _);
pub const VSS_E_HOLD_WRITES_TIMEOUT: windows_core::HRESULT =
    windows_core::HRESULT(0x80042314_u32 as _);
pub const VSS_E_INSUFFICIENT_STORAGE: windows_core::HRESULT =
    windows_core::HRESULT(0x8004231F_u32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: windows_core::HRESULT =
    windows_core::HRESULT(0x80042317_u32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: windows_core::HRESULT =
    windows_core::HRESULT(0x80042312_u32 as _);
pub const VSS_E_OBJECT_NOT_FOUND: windows_core::HRESULT =
    windows_core::HRESULT(0x80042308_u32 as _);
pub const VSS_E_PROVIDER_VETO: windows_core::HRESULT = windows_core::HRESULT(0x80042306_u32 as _);
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: windows_core::HRESULT =
    windows_core::HRESULT(0x80042316_u32 as _);
pub const VSS_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80042302_u32 as _);
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: windows_core::HRESULT =
    windows_core::HRESULT(0x8004230F_u32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED: windows_core::HRESULT =
    windows_core::HRESULT(0x8004230C_u32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: windows_core::HRESULT =
    windows_core::HRESULT(0x8004230E_u32 as _);
pub const VSS_E_WRITERERROR_NONRETRYABLE: windows_core::HRESULT =
    windows_core::HRESULT(0x800423F4_u32 as _);
pub const VSS_E_WRITERERROR_RETRYABLE: windows_core::HRESULT =
    windows_core::HRESULT(0x800423F3_u32 as _);
pub const VSS_E_WRITER_INFRASTRUCTURE: windows_core::HRESULT =
    windows_core::HRESULT(0x80042318_u32 as _);
pub type VSS_FILE_RESTORE_STATUS = i32;
pub type VSS_ID = windows_core::GUID;
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = 4;
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = 3;
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = 2;
pub type VSS_OBJECT_TYPE = i32;
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: VSS_ID,
    pub m_pwszProviderName: VSS_PWSZ,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: VSS_PWSZ,
    pub m_ProviderVersionId: VSS_ID,
    pub m_ClassId: windows_core::GUID,
}
pub type VSS_PROVIDER_TYPE = i32;
pub type VSS_PWSZ = *mut u16;
pub type VSS_RESTORE_TYPE = i32;
pub type VSS_SNAPSHOT_CONTEXT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: VSS_ID,
    pub m_SnapshotSetId: VSS_ID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: VSS_PWSZ,
    pub m_pwszOriginalVolumeName: VSS_PWSZ,
    pub m_pwszOriginatingMachine: VSS_PWSZ,
    pub m_pwszServiceMachine: VSS_PWSZ,
    pub m_pwszExposedName: VSS_PWSZ,
    pub m_pwszExposedPath: VSS_PWSZ,
    pub m_ProviderId: VSS_ID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: VSS_TIMESTAMP,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
pub type VSS_SNAPSHOT_STATE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VSS_TIMESTAMP(pub i64);
pub type VSS_WRITER_STATE = i32;
