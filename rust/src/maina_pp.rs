use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::mania::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut ManiaPP<'jvm> {
    let pp = ManiaPP::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm ManiaPP<'jvm>,
) -> &'jvm mut ManiaPP<'jvm> {
    let clone = pp.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: *mut ManiaPP<'jvm>,
) {
    drop(unsafe { Box::from_raw(pp) })
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm ManiaPP<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm ManiaPP<'jvm>,
) -> &'jvm mut ManiaPerformanceAttributes {
    let attributes = pp.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, mods: jlong,
) {
    *pp = pp.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withHitResultPriority_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withHitResultPriority_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, index: jint,
) {
    let priority = parse_hit_result_priority(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    *pp = pp.clone().hitresult_priority(priority as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withN300_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n300(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    setN100_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN100_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n100(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withN50_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN50_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n50(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withN200_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN200_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n_misses(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withN320_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN320_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withNMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().clock_rate(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withAccuracy_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withAccuracy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().accuracy(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withIsConvert_00024rosu_pp_jni
 * Signature: (JZ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withIsConvert_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jboolean,
) {
    *pp = pp.clone().is_convert(value != 0);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes,
) -> *mut ManiaPerformanceAttributes {
    let clone = attributes.clone();

    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, attributes: *mut ManiaPerformanceAttributes,
) {
    drop(unsafe { Box::from_raw(attributes) })
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes, pretty: jboolean,
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