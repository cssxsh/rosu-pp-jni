use std::collections::hash_map::Entry;
use std::collections::HashMap;
use core::mem::size_of;
use std::sync::Mutex;
use jni::objects::JClass;
use jni::JNIEnv;
use jni::sys::{jbyteArray, jdouble, jint, jlong, jstring};
use once_cell::sync::Lazy;
use bytes::BufMut;
use crate::{Beatmap, OsuPP};
use crate::osu::{OsuAttributeProvider, OsuPerformanceAttributes};

static ALLOCATED_MAP: Lazy<Mutex<HashMap<String, &'static Beatmap>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCreate(
    env: JNIEnv<'_>, _class: JClass<'_>, path: jstring
) -> jlong {
    let path_jstr = env.get_string(path.into()).unwrap();
    let path = path_jstr.to_str().unwrap();

    let mut beatmap_pool = ALLOCATED_MAP.lock().unwrap();

    let beatmap_entry = beatmap_pool.entry(String::from(path));
    let beatmap = match beatmap_entry {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(Box::leak(Box::new(Beatmap::from_path(path).unwrap()))),
    };

    let calc = OsuPP::new(beatmap);
    let leaked = Box::leak(Box::new(calc));

    leaked as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nMods(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, mods: jint
) -> jlong {
    let calc = calc_ptr.mods_borrow(mods as u32);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nAttributes(
    _env: JNIEnv<'_>, _class: JClass<'_>,
    calc_ptr: &'static mut OsuPP<'static>, attr_ptr: &'static mut OsuPerformanceAttributes
) -> jlong {
    let attr = unsafe { Box::from_raw(attr_ptr) };
    let calc = calc_ptr.attributes_borrow(attr.attributes().unwrap());
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCombo(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, combo: jint
) -> jlong {
    let calc = calc_ptr.combo_borrow(combo as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN300(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, n: jint
) -> jlong {
    let calc = calc_ptr.n300_borrow(n as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN100(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, n: jint
) -> jlong {
    let calc = calc_ptr.n100_borrow(n as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nN50(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, n: jint
) -> jlong {
    let calc = calc_ptr.n50_borrow(n as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nMisses(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, n: jint
) -> jlong {
    let calc = calc_ptr.misses_borrow(n as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nPassedObjects(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, n: jint
) -> jlong {
    let calc = calc_ptr.passed_objects_borrow(n as usize);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nAccuracy(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>, acc: jdouble
) -> jlong {
    let calc = calc_ptr.accuracy_borrow(acc);
    calc as *const _ as i64
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nCalculate(
    env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>
) -> jbyteArray {

    let calc = unsafe { Box::from_raw(calc_ptr) };
    let result = calc.calculate();
    let cloned_attributes = result.clone().attributes().unwrap();
    let leaked = Box::leak(Box::new(result));

    let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<jlong>() + size_of::<jdouble>() * 12 + size_of::<jint>() * 3);

    buffer.put_i64(leaked as *const _ as i64);
    buffer.put_f64(leaked.pp);
    buffer.put_f64(leaked.pp_aim);
    buffer.put_f64(leaked.pp_speed);
    buffer.put_f64(leaked.pp_acc);

    buffer.put_f64(cloned_attributes.stars);
    buffer.put_f64(cloned_attributes.ar);
    buffer.put_f64(cloned_attributes.od);
    buffer.put_f64(cloned_attributes.hp);
    buffer.put_f64(0.0);
    buffer.put_f64(cloned_attributes.aim_strain);
    buffer.put_f64(cloned_attributes.slider_factor);
    buffer.put_f64(cloned_attributes.speed_strain);
    buffer.put_i32(cloned_attributes.max_combo as i32);
    buffer.put_i32(cloned_attributes.n_circles as i32);
    buffer.put_i32(cloned_attributes.n_sliders as i32);
    buffer.put_i32(cloned_attributes.n_spinners as i32);

    env.byte_array_from_slice(buffer.as_slice()).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nReleaseAttribute(
    _env: JNIEnv<'_>, _class: JClass<'_>, attr_ptr: &'static mut OsuPerformanceAttributes
) {
    unsafe { Box::from_raw(attr_ptr) };
}

#[no_mangle]
pub extern "system" fn Java_me_stageguard_obms_osu_algorithm_ppnative_PPCalculatorNativeKt_nReleaseCalculator(
    _env: JNIEnv<'_>, _class: JClass<'_>, calc_ptr: &'static mut OsuPP<'static>
) {
    unsafe { Box::from_raw(calc_ptr) };
}