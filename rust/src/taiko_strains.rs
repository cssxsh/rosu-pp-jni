use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::taiko::*;

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> &'jvm mut TaikoStrains {
    let clone = strains.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, strains: *mut TaikoStrains,
) {
    drop(unsafe { Box::from_raw(strains) })
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    len_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_len_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> jlong {
    strains.len() as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    getSectionLength_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_getSectionLength_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> jdouble {
    strains.section_len as _
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    getColor_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_getColor_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> jdoubleArray {
    let color = strains.color.clone();
    let array = _env.new_double_array(color.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, color.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    getRhythm_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_getRhythm_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> jdoubleArray {
    let rhythm = strains.rhythm.clone();
    let array = _env.new_double_array(rhythm.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, rhythm.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoStrains
 * Method:    getStamina_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoStrains_getStamina_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm TaikoStrains,
) -> jdoubleArray {
    let stamina = strains.stamina.clone();
    let array = _env.new_double_array(stamina.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, stamina.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}