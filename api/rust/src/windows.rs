use super::*;

use std::mem;

extern crate winapi;

use self::winapi::shared::ntdef::{HANDLE};
use self::winapi::um::processthreadsapi::{PROCESS_INFORMATION};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenLevel {
    Lockdown = 0,
    Restricted,
    Limited,
    Interactive,
    NonAdmin,
    RestrictedSameAccess,
    Unprotected,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobLevel {
    Lockdown = 0,
    Restricted,
    LimitedUser,
    Interactive,
    Unprotected,
    None,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegrityLevel {
    System,
    High,
    Medium,
    MediumLow,
    Low,
    BelowLow,
    Untrusted,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSubSystem {
    Files,             
    NamedPipes,       
    Process,           
    Registry,          
    Synchronization,              
    Lockdown,    
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSemantics {
    FilesAllowAny,       
    FilesAllowReadOnly,  
    FilesAllowQuery,     
    FilesAllowDirAny,   
    NamedPipesAllowAny,  
    ProcessMinExec,      
    ProcessAllExec,      
    EventsAllowAny,      
    EventsAllowReadOnly, 
    RegistryAllowReadOnly,    
    RegistryAllowAny,         
    FakeUserGdiImpl,    
    ImplementOpmApis,     
}

impl Policy {
    pub fn set_token_level(&mut self, initial: TokenLevel, lockdown: TokenLevel) -> Result<()> {
        unsafe {
            try_sb!(sys::sandbox_policy_set_token_level(self.0, mem::transmute(initial), mem::transmute(lockdown)));

            Ok(())
        }
    }

    pub fn set_job_level(&mut self, level: JobLevel) -> Result<()> {
        unsafe {
            try_sb!(sys::sandbox_policy_set_job_level(self.0, mem::transmute(level)));

            Ok(())
        }
    }

    pub fn set_integrity_level(&mut self, level: IntegrityLevel) -> Result<()> {
        unsafe {
            try_sb!(sys::sandbox_policy_set_integrity_level(self.0, mem::transmute(level)));

            Ok(())
        }
    }

    pub fn set_delayed_integrity_level(&mut self, level: IntegrityLevel) -> Result<()> {
        unsafe {
            try_sb!(sys::sandbox_policy_set_delayed_integrity_level(self.0, mem::transmute(level)));

            Ok(())
        }
    }

    pub fn set_alternate_desktop(&mut self, alternate_winstation: bool) -> Result<()> {
        unsafe {
            try_sb!(sys::sandbox_policy_set_alternate_desktop(self.0, alternate_winstation));

            Ok(())
        }
    }

    pub fn set_low_box<S>(&mut self, sid: S) -> Result<()> where S: AsRef<OsStr> {
        let mut string_sid: Vec<u16> = sid.as_ref()
                .encode_wide()
                .collect();
        string_sid.push(0);

        unsafe {
            try_sb!(sys::sandbox_policy_set_low_box(self.0, string_sid.as_ptr()));

            Ok(())
        }
    }

    pub unsafe fn set_stdout_handle(&mut self, handle: HANDLE) -> Result<()> {
        try_sb!(sys::sandbox_policy_set_stdout_handle(self.0, mem::transmute(handle)));

        Ok(())
    }

    pub unsafe fn set_stderr_handle(&mut self, handle: HANDLE) -> Result<()> {
        try_sb!(sys::sandbox_policy_set_stderr_handle(self.0, mem::transmute(handle)));

        Ok(())
    }

    pub unsafe fn add_handle_to_share(&mut self, handle: HANDLE) -> Result<()> {
        try_sb!(sys::sandbox_policy_add_handle_to_share(self.0, mem::transmute(handle)));

        Ok(())
    }

    pub fn add_rule<P>(&mut self, subsystem: RuleSubSystem, semantics: RuleSemantics, pattern: P) -> Result<()> where
        P: AsRef<OsStr>,
    {
        let mut string_pattern: Vec<u16> = pattern.as_ref()
                .encode_wide()
                .collect();
        string_pattern.push(0);

        unsafe {
            try_sb!(sys::sandbox_policy_add_rule(self.0, mem::transmute(subsystem), mem::transmute(semantics), string_pattern.as_ptr()));
            Ok(())
        }
    }
}

impl TargetProcess {
    pub unsafe fn get_process_handle(&self) -> HANDLE {
        let process_info = sys::sandbox_target_process_get_process_information(self.0) as *const PROCESS_INFORMATION;
        (*process_info).hProcess
    }

    pub unsafe fn get_thread_handle(&self) -> HANDLE {
        let process_info = sys::sandbox_target_process_get_process_information(self.0) as *const PROCESS_INFORMATION;
        (*process_info).hThread
    }
}