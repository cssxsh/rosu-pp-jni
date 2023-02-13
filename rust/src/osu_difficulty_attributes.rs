use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::osu::*;

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> *mut OsuDifficultyAttributes {
    let clone = difficulty.clone();
    Box::into_raw(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, difficulty: *mut OsuDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(difficulty) })
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getAim_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getAim_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.aim as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getSpeed_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getSpeed_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.speed as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getFlashlight_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getFlashlight_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.flashlight as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getSpeedNoteCount_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getSliderFactor_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.slider_factor as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getSpeedNoteCount_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getSpeedNoteCount_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.speed_note_count as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getAR_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getAR_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.ar as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getOD_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getOD_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.od as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getHP_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getHP_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jdouble {
    difficulty.hp as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getNCircles_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getNCircles_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jlong {
    difficulty.n_circles as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getNSliders_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getNSliders_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jlong {
    difficulty.n_sliders as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getNSpinners_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getNSpinners_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jlong {
    difficulty.n_spinners as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getStars_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getStars_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jlong {
    difficulty.stars as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuDifficultyAttributes
 * Method:    getMaxCombo_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuDifficultyAttributes_getMaxCombo_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, difficulty: &'jvm OsuDifficultyAttributes,
) -> jlong {
    difficulty.max_combo as _
}