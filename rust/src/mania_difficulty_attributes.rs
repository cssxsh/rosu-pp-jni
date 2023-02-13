use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::mania::*;

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