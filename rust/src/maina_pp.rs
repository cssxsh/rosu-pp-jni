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
 * Method:    withPerformanceAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withPerformanceAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_performance_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withDifficultyAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withDifficultyAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_difficulty_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
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
 * Signature: (JI)V
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
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n300(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withN100_00024rosu_pp_jni
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
 * Signature: (JJ)V
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
 * Signature: (JJ)V
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
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPP_withN320_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut ManiaPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPP
 * Method:    withNMisses_00024rosu_pp_jni
 * Signature: (JJ)V
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
 * Signature: (JJ)V
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
 * Signature: (JD)V
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
 * Signature: (JD)V
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
 * Signature: (JZ)V
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

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    pp_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_pp_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes,
) -> jdouble {
    attributes.pp() as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    stars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_stars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes,
) -> jdouble {
    attributes.stars() as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    maxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_maxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes,
) -> jlong {
    attributes.max_combo() as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaPerformanceAttributes
 * Method:    getDifficulty_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaPerformanceAttributes_getDifficulty_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm ManiaPerformanceAttributes,
) -> *mut ManiaDifficultyAttributes {
    let difficulty = attributes.difficulty.clone();
    Box::into_raw(Box::new(difficulty))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm ManiaDifficultyAttributes,
) -> *mut ManiaDifficultyAttributes {
    let clone = difficulty.clone();
    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, difficulty: *mut ManiaDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(difficulty) })
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm ManiaDifficultyAttributes, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{difficulty:#?}")
    } else {
        format!("{difficulty:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    getStars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_getStars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm ManiaDifficultyAttributes,
) -> jdouble {
    difficulty.stars as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    getHitWindow_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_getHitWindow_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm ManiaDifficultyAttributes,
) -> jlong {
    difficulty.hit_window as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaDifficultyAttributes
 * Method:    getMaxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaDifficultyAttributes_getMaxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm ManiaDifficultyAttributes,
) -> jlong {
    difficulty.max_combo as _
}