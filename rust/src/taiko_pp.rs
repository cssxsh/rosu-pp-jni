use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::taiko::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut TaikoPP<'jvm> {
    let pp = TaikoPP::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm TaikoPP<'jvm>,
) -> &'jvm mut TaikoPP<'jvm> {
    let clone = pp.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: *mut TaikoPP<'jvm>,
) {
    drop(unsafe { Box::from_raw(pp) })
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm TaikoPP<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm TaikoPP<'jvm>,
) -> &'jvm mut TaikoPerformanceAttributes {

    let attributes = pp.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withPerformanceAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withPerformanceAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_performance_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withDifficultyAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withDifficultyAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_difficulty_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
}
/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, mods: jlong,
) {
    *pp = pp.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withCombo_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, combo: jlong,
) {
    *pp = pp.clone().combo(combo as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withHitResultPriority_00024rosu_pp_jni
 * Signature: (JI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withHitResultPriority_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, index: jint,
) {
    let priority = parse_hit_result_priority(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    *pp = pp.clone().hitresult_priority(priority as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withN300_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n300(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withN100_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withN100_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n100(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withNMisses_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withNMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().n_misses(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().clock_rate(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withAccuracy_00024rosu_pp_jni
 * Signature: (JD)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withAccuracy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().accuracy(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPP
 * Method:    withIsConvert_00024rosu_pp_jni
 * Signature: (JZ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPP_withIsConvert_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut TaikoPP<'jvm>, value: jboolean,
) {
    *pp = pp.clone().is_convert(value != 0);
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes,
) -> *mut TaikoPerformanceAttributes {
    let clone = attributes.clone();

    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, attributes: *mut TaikoPerformanceAttributes,
) {
    drop(unsafe { Box::from_raw(attributes) })
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    pp_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_pp_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes,
) -> jdouble {
    attributes.pp() as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    stars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_stars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes,
) -> jdouble {
    attributes.stars() as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    maxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_maxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes,
) -> jlong {
    attributes.max_combo() as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoPerformanceAttributes
 * Method:    getDifficulty_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoPerformanceAttributes_getDifficulty_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, attributes: &'jvm TaikoPerformanceAttributes,
) -> *mut TaikoDifficultyAttributes {
    let difficulty = attributes.difficulty.clone();
    Box::into_raw(Box::new(difficulty))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> *mut TaikoDifficultyAttributes {
    let clone = difficulty.clone();
    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, difficulty: *mut TaikoDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(difficulty) })
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getStamina_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getStamina_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.stamina as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getRhythm_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getRhythm_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.rhythm as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getColour_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getColour_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.colour as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getPeak_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getPeak_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.peak as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getHitWindow_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getHitWindow_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.hit_window as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getStars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getStars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jdouble {
    difficulty.stars as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoDifficultyAttributes
 * Method:    getMaxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoDifficultyAttributes_getMaxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm TaikoDifficultyAttributes,
) -> jlong {
    difficulty.max_combo as _
}