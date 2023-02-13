use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::catch::*;

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> *mut CatchDifficultyAttributes {
    let clone = difficulty.clone();
    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, difficulty: *mut CatchDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(difficulty) })
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_debug_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, difficulty: *mut CatchDifficultyAttributes, pretty: jboolean
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
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    getStars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_getStars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jdouble {
    difficulty.stars as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    getAR_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_getAR_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jdouble {
    difficulty.ar as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    getNFruits_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_getNFruits_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jlong {
    difficulty.n_fruits as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    getNDroplets_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_getNDroplets_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jlong {
    difficulty.n_droplets as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    getNTinyDroplets_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_getNTinyDroplets_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jlong {
    difficulty.n_tiny_droplets as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchDifficultyAttributes
 * Method:    maxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchDifficultyAttributes_maxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm CatchDifficultyAttributes,
) -> jlong {
    difficulty.max_combo() as _
}