use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    default_00024rosu_pp_jni
 * Signature: ()J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_default_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass,
) -> *mut ScoreState {
    let default = ScoreState::default();

    Box::into_raw(Box::new(default))
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> *mut ScoreState {
    let clone = state.clone();

    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, state: *mut ScoreState,
) {
    drop(unsafe { Box::from_raw(state) })
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{state:#?}")
    } else {
        format!("{state:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    totalHits_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_totalHits_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState, index: jint,
) -> jlong {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    state.total_hits(mode) as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    accuracy_00024rosu_pp_jni
 * Signature: (JI)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_accuracy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState, index: jint,
) -> jdouble {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    match mode {
        GameMode::Osu => osu::OsuScoreState::from(state.clone()).accuracy(),
        GameMode::Taiko => taiko::TaikoScoreState::from(state.clone()).accuracy(),
        GameMode::Catch => 0.0f64,
        GameMode::Mania => mania::ManiaScoreState::from(state.clone()).accuracy(),
    }
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getMaxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getMaxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setMaxCombo_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setMaxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.max_combo = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getNGeki_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getNGeki_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n_geki as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setNGeki_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setNGeki_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n_geki = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getNKatu_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getNKatu_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n_katu as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setNKatu_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setNKatu_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n_katu = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getN300_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n300 as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setN300_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setN300_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n300 = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getN100_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getN100_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n100 as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setN100_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setN100_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n100 = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getN50_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getN50_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n50 as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setN50_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setN50_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n50 = max_combo as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    getNMisses_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_getNMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm ScoreState,
) -> jlong {
    state.n_misses as _
}

/*
 * Class:     xyz_cssxsh_rosu_ScoreState
 * Method:    setNMisses_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ScoreState_setNMisses_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, state: &'jvm mut ScoreState, max_combo: jlong
) {
    state.n_misses = max_combo as _
}