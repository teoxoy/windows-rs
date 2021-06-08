use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let tokens = windows_macros::generate! {
        Windows::{
            Foundation::{IReference, IStringable, PropertyValue},
            Win32::{
                Foundation::{CloseHandle, BSTR, CO_E_NOTINITIALIZED, E_POINTER},
                System::{
                    Com::{
                        CLSIDFromProgID, CoCreateGuid, CoCreateInstance, CoInitializeEx,
                        CoTaskMemAlloc, CoTaskMemFree, IAgileObject,
                    },
                    Diagnostics::Debug::{FormatMessageW, GetLastError},
                    LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA},
                    Memory::{GetProcessHeap, HeapAlloc, HeapFree},
                    OleAutomation::{GetErrorInfo, IErrorInfo, SetErrorInfo},
                    Threading::{CreateEventA, SetEvent, WaitForSingleObject},
                    WinRT::{
                        ILanguageExceptionErrorInfo2, IRestrictedErrorInfo, IWeakReference,
                        IWeakReferenceSource,
                    },
                },
            },
        },
    };

    let mut path = windows_gen::workspace_dir();
    path.push("src");
    path.push("bindings.rs");

    let mut file = std::fs::File::create(&path)?;
    file.write_all(
        "// This file was generated by the `windows` crate - do not edit by hand!\n\n".as_bytes(),
    )?;

    file.write_all(tokens.as_bytes())?;
    drop(file);

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}
