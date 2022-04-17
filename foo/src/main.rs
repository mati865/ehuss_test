#![allow(unused)]
use std::ffi::{CStr, OsStr};
use std::mem::*;
use std::os::windows::ffi::OsStrExt;
use std::path::*;
use std::process::Command;
use std::ptr;
use tempfile::TempDir;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::*;
use winapi::shared::sddl::*;
use winapi::um::accctrl::*;
use winapi::um::aclapi::*;
use winapi::um::handleapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::securitybaseapi::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

fn output(cmd: &mut Command) {
    println!("{cmd:?}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !stdout.is_empty() {
        println!("{stdout}");
    }
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !stderr.is_empty() {
        println!("{stderr}");
    }
}

fn sid_dup(sid: PSID) -> PSID {
    unsafe {
        let len = GetLengthSid(sid);
        let dup: Vec<u8> = Vec::with_capacity(len as usize);
        if CopySid(len, dup.as_ptr() as PSID, sid) == 0 {
            panic!("can't copy");
        }
        let p = dup.as_ptr() as PSID;
        std::mem::forget(dup);
        p
    }
}

fn print_user(sid: PSID) {
    unsafe {
        let mut cc_name = 0;
        let mut cc_domainname = 0;
        let mut pe_use = 0;
        let _ = LookupAccountSidW(
            ptr::null::<u16>() as *mut u16,
            sid,
            ptr::null::<u16>() as *mut u16,
            &mut cc_name,
            ptr::null::<u16>() as *mut u16,
            &mut cc_domainname,
            &mut pe_use,
        );

        let mut name: Vec<u16> = Vec::with_capacity(cc_name as usize);
        let mut domainname: Vec<u16> = Vec::with_capacity(cc_domainname as usize);
        name.set_len(cc_name as usize);
        domainname.set_len(cc_domainname as usize);
        let ret = LookupAccountSidW(
            ptr::null::<u16>() as *mut u16,
            sid,
            name.as_mut_ptr() as *mut u16,
            &mut cc_name,
            domainname.as_mut_ptr() as *mut u16,
            &mut cc_domainname,
            &mut pe_use,
        );

        if ret == 0 {
            panic!("lookup");
        }

        let name = from_wide_ptr(name.as_ptr());
        let domainname = from_wide_ptr(domainname.as_ptr());

        println!("name={name}");
    }
}

fn check(path: &Path) {
    println!("check {path:?}");
    unsafe {
        let mut token = std::mem::zeroed();
        let mut len: DWORD = 0;
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token as *mut HANDLE) != 1 {
            panic!("process token error");
        }

        if GetTokenInformation(token, TokenUser, ptr::null_mut(), 0, &mut len as *mut DWORD) == 1 {
            panic!("gettokeninformation");
        }

        let mut buf = Vec::with_capacity(len as usize);
        if GetTokenInformation(
            token,
            TokenUser,
            buf.as_mut_ptr() as *mut c_void,
            len,
            &mut len as *mut DWORD,
        ) != 1
        {
            panic!("gettokeninformation2 {:?}", std::io::Error::last_os_error());
        }
        let info = buf.as_ptr() as *const TOKEN_USER;
        let user_sid_dup = sid_dup((*info).User.Sid);

        CloseHandle(token);

        let mut owner_sid: PSID = ptr::null_mut();
        let mut descriptor = ptr::null_mut();

        let path_w32 = wstr(path.to_str().unwrap());
        let ret = GetNamedSecurityInfoW(
            path_w32.as_ptr(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner_sid,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut descriptor,
        );
        if ret != 0 {
            panic!("ret={}", ret);
        }
        let owner_sid_dup = sid_dup(owner_sid);
        LocalFree(descriptor);

        let mut owner_sid_str: LPSTR = zeroed();
        if ConvertSidToStringSidA(owner_sid_dup, &mut owner_sid_str) == 0 {
            panic!("SidToString");
        }
        let mut user_sid_str: LPSTR = zeroed();
        if ConvertSidToStringSidA(user_sid_dup, &mut user_sid_str) == 0 {
            panic!("SidToString");
        }
        let owner_ss = CStr::from_ptr(owner_sid_str);
        let user_ss = CStr::from_ptr(user_sid_str);
        println!(
            "owner={} user={}",
            owner_ss.to_str().unwrap(),
            user_ss.to_str().unwrap()
        );

        print_user(owner_sid_dup);
        print_user(user_sid_dup);
        if EqualSid(owner_sid_dup, user_sid_dup) == 1 {
            println!("Equal");
        } else {
            println!("not equal");
        }
    }
}

fn doit() {
    // let tmp_path = std::env::temp_dir().join(".tmp1234");
    // std::fs::create_dir(&tmp_path).unwrap();
    let td = TempDir::new().unwrap();
    let tmp_path = td.path();
    println!("{:?}", tmp_path);
    output(Command::new("ls").arg("-al").arg(&tmp_path));
    output(Command::new("who").arg("am").arg("i"));
    output(
        Command::new("powershell")
            .arg("-Command")
            .arg(format!("get-acl {} | format-list", tmp_path.display())),
    );
    check(&tmp_path);
    // let mut slashed = PathBuf::from(&tmp_path);
    // slashed.push("");
    // check(&slashed);
    // let slashed = tmp_path.to_str().unwrap();
    // let slashed = slashed.replace("\\", "/");
    // let slashed_p = PathBuf::from(&slashed);
    // check(&slashed_p);
    // let slashed_p = PathBuf::from(slashed + "/");
    // check(&slashed_p);
}

fn main() {
    doit();
}

fn wstr(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
    if wide.iter().any(|b| *b == 0) {
        panic!("nul byte in wide string");
    }
    wide.push(0);
    wide
}

fn from_wide_ptr(ptr: *const u16) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    unsafe {
        assert!(!ptr.is_null());
        let len = (0..std::isize::MAX)
            .position(|i| *ptr.offset(i) == 0)
            .unwrap();
        let slice = std::slice::from_raw_parts(ptr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

#[test]
fn from_test() {
    doit();
}
