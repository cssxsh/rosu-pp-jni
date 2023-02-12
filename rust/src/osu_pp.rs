use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::osu::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut OsuPP<'jvm> {
    let pp = OsuPP::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm OsuPP<'jvm>,
) -> &'jvm mut OsuPP<'jvm> {
    let clone = pp.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: *mut OsuPP<'jvm>,
) {
    drop(unsafe { Box::from_raw(pp) })
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm OsuPP<'jvm>, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{pp:#?}")
    } else {
        format!("{pp:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm OsuPP<'jvm>,
) -> &'jvm mut OsuPerformanceAttributes {
    let attributes = pp.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    convertMode_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_convertMode_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm OsuPP<'jvm>, index: jint,
) -> *mut u8 {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let other = match pp.clone().mode(mode) {
        AnyPP::Osu(o) => Box::into_raw(Box::new(o)) as *mut u8,
        AnyPP::Taiko(t) => Box::into_raw(Box::new(t)) as *mut u8,
        AnyPP::Catch(c) => Box::into_raw(Box::new(c)) as *mut u8,
        AnyPP::Mania(m) => Box::into_raw(Box::new(m)) as *mut u8,
    };

    other
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, mods: jlong,
) {
    *pp = pp.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withCombo_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, combo: jlong,
) {
    *pp = pp.clone().combo(combo as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withHitResultPriority_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withHitResultPriority_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, index: jint,
) {
    let priority = parse_hit_result_priority(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    *pp = pp.clone().hitresult_priority(priority as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withN300_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n300(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    setN100_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withN100_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n100(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withN50_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withN50_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n50(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withNMisses_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withNMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n_misses(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().clock_rate(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPP
 * Method:    withAccuracy_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPP_withAccuracy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut OsuPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().accuracy(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPerformanceAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPerformanceAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm OsuPerformanceAttributes,
) -> *mut OsuPerformanceAttributes {
    let clone = attributes.clone();

    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPerformanceAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPerformanceAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, attributes: *mut OsuPerformanceAttributes,
) {
    drop(unsafe { Box::from_raw(attributes) })
}

/*
 * Class:     xyz_cssxsh_rosu_OsuPerformanceAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuPerformanceAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm OsuPerformanceAttributes, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{attributes:#?}")
    } else {
        format!("{attributes:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}