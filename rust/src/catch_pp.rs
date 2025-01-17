use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::catch::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut CatchPP<'jvm> {
    let pp = CatchPP::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm CatchPP<'jvm>,
) -> &'jvm mut CatchPP<'jvm> {
    let clone = pp.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: *mut CatchPP<'jvm>,
) {
    drop(unsafe { Box::from_raw(pp) })
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm CatchPP<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm CatchPP<'jvm>,
) -> &'jvm mut CatchPerformanceAttributes {
    let attributes = pp.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withPerformanceAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withPerformanceAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_performance_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withDifficultyAttributes_00024rosu_pp_jni
 * Signature: (JJI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withDifficultyAttributes_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, ptr: jlong, index: jint,
) {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let attributes = parse_difficulty_attributes(ptr, mode);

    *pp = pp.clone().attributes(attributes);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, mods: jlong,
) {
    *pp = pp.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withCombo_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, combo: jlong,
) {
    *pp = pp.clone().combo(combo as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withFruits_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withFruits_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().fruits(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withDroplets_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withDroplets_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().droplets(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withTinyDropletMisses_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withTinyDropletMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().droplets(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withTinyDroplets_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withTinyDroplets_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().droplets(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withMisses_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().misses(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jlong,
) {
    *pp = pp.clone().passed_objects(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().clock_rate(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withAccuracy_00024rosu_pp_jni
 * Signature: (JD)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withAccuracy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, value: jdouble,
) {
    *pp = pp.clone().accuracy(value as _);
}

/*
 * Class:     xyz_cssxsh_rosu_CatchPP
 * Method:    withScoreState_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchPP_withScoreState_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, pp: &'jvm mut CatchPP<'jvm>, state: &'jvm ScoreState,
) {
    let clone = state.clone();
    *pp = pp.clone().state(clone.into());
}