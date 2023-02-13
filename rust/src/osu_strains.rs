use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::osu::*;

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> &'jvm mut OsuStrains {
    let clone = strains.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, strains: *mut OsuStrains,
) {
    drop(unsafe { Box::from_raw(strains) })
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{strains:#?}")
    } else {
        format!("{strains:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    len_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_len_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> jlong {
    strains.len() as _
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    getAim_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_getAim_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> jdoubleArray {
    let aim = strains.aim.clone();
    let array = _env.new_double_array(aim.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, aim.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    getAimNoSliders_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_getAimNoSliders_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> jdoubleArray {
    let aim_no_sliders = strains.aim_no_sliders.clone();
    let array = _env.new_double_array(aim_no_sliders.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, aim_no_sliders.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    getSpeed_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_getSpeed_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> jdoubleArray {
    let speed = strains.speed.clone();
    let array = _env.new_double_array(speed.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, speed.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStrains
 * Method:    getFlashlight_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStrains_getFlashlight_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm OsuStrains,
) -> jdoubleArray {
    let flashlight = strains.flashlight.clone();
    let array = _env.new_double_array(flashlight.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, flashlight.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}