use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::mania::*;

/*
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm ManiaStrains,
) -> &'jvm mut ManiaStrains {
    let clone = strains.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, strains: *mut ManiaStrains,
) {
    drop(unsafe { Box::from_raw(strains) })
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm ManiaStrains, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    len_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_len_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm ManiaStrains,
) -> jlong {
    strains.len() as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    getSectionLength_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_getSectionLength_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm ManiaStrains,
) -> jdouble {
    strains.section_len as _
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStrains
 * Method:    getMovement_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStrains_getStrains_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm ManiaStrains,
) -> jdoubleArray {
    let strains = strains.strains.clone();
    let array = _env.new_double_array(strains.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, strains.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}