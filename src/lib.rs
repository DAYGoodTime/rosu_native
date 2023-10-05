use std::ffi::{CStr, CString};
use std::future::ready;
use std::ptr;
use libc::c_char;
use rosu_pp::{Beatmap, BeatmapExt, PerformanceAttributes};
use rosu_pp::osu::OsuPerformanceAttributes;

#[no_mangle]
pub extern "C" fn hello_rust(n: *const c_char) -> *mut c_char {
    let cs_str = CString::new(mkstr(n)).expect("Failed to create CString!");
    return cs_str.into_raw();
}

#[repr(C)]
pub struct OsuMap {
    path: *const c_char,
    mods: u32,
    acc: f64,
    miss: usize,
    combo: usize,
    max_combo: usize,
}

#[repr(C)]
pub struct PPResult {
    pub pp: f64,
    /// The accuracy portion of the final pp.
    pub pp_acc: f64,
    /// The aim portion of the final pp.
    pub pp_aim: f64,
    /// The speed portion of the final pp.
    pub pp_speed: f64,
    /// pp if fc
    pub pp_fc: f64,
    /// Max pp
    pub max_pp: f64,
    /// map star
    pub map_star: f64,
    /// debug text
    pub debug_text: *const c_char,
}

#[no_mangle]
pub extern "C" fn cal_pp(map_args: *const OsuMap) -> *const PPResult {
    unsafe {
        let map = ptr::read(map_args);
        let path_to_osu = mkstr(map.path);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let osumap_result = rt.block_on(async {
            // Parse the map asynchronously
            let res = ready(Beatmap::from_path(path_to_osu)).await;
            res
        });

        let osumap = match osumap_result {
            Ok(map) => map,
            Err(err) => {
                // 处理解析错误
                panic!("Error while parsing map: {}", err);
            }
        };
        let debug = format!("mods {} combo {} miss {} acc {} max_combo {}",map.mods,map.combo,map.miss,map.acc,map.max_combo);
        // The rest stays the same
        let result = osumap
            .pp()
            .mods(map.mods) // HDHR
            .combo(map.combo)
            .n_misses(map.miss)
            .accuracy(map.acc)
            .calculate();
        let stdres = match result.clone() {
            PerformanceAttributes::Osu(osu) => osu,
            _ => OsuPerformanceAttributes {
                difficulty: Default::default(),
                pp: 0.0,
                pp_acc: 0.0,
                pp_aim: 0.0,
                pp_flashlight: 0.0,
                pp_speed: 0.0,
                effective_miss_count: 0.0,
            }
        };
        let max_pp_result = osumap.max_pp(map.mods).pp();
        let if_fc = osumap.pp()
            .mods(map.mods)
            .attributes(result.clone())
            .accuracy(map.acc)
            .combo(map.max_combo)
            .n_misses(map.miss)
            .calculate();
        let pp_result = PPResult {
            pp: result.pp(),
            pp_aim: stdres.pp_aim,
            pp_speed: stdres.pp_speed,
            pp_acc: stdres.pp_acc,
            pp_fc: if_fc.pp(),
            max_pp: max_pp_result,
            map_star: if_fc.stars(),
            debug_text: mk_ptr_string(debug),
        };
        Box::into_raw(Box::new(pp_result))
    }
}

#[no_mangle]
pub extern "C" fn return_obj() -> *const PPResult {
    let result = PPResult {
        pp: 0.0,
        pp_acc: 0.0,
        pp_aim: 0.0,
        pp_speed: 0.0,
        max_pp: 0.0,
        pp_fc: 0.0,
        map_star: 0.0,
        debug_text: mk_ptr_str("OK"),
    };
    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn debug_return(n: u32) -> usize {
    return n as usize;
}

#[no_mangle]
pub extern fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        let _ = CString::from_raw(s);
    };
}


pub fn mkstr(s: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().expect("Could not successfully convert string form foreign code!");
    String::from(r_str)
}

pub fn mk_ptr_string(s: String) -> *const c_char {
    CString::new(s).expect("Failed to create CString!").into_raw()
}

pub fn mk_ptr_str(s: &str) -> *const c_char {
    CString::new(s).expect("Failed to create CString!").into_raw()
}