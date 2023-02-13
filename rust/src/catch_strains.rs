use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::catch::*;

/*
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm CatchStrains,
) -> &'jvm mut CatchStrains {
    let clone = strains.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, strains: *mut CatchStrains,
) {
    drop(unsafe { Box::from_raw(strains) })
}

/*
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm CatchStrains, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    len_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_len_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm CatchStrains,
) -> jlong {
    strains.len() as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    getSectionLength_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_getSectionLength_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm CatchStrains,
) -> jdouble {
    strains.section_len as _
}

/*
 * Class:     xyz_cssxsh_rosu_CatchStrains
 * Method:    getMovement_00024rosu_pp_jni
 * Signature: (J)[D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchStrains_getMovement_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, strains: &'jvm CatchStrains,
) -> jdoubleArray {
    let movement = strains.movement.clone();
    let array = _env.new_double_array(movement.len() as _)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    _env.set_double_array_region(array, 0, movement.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    array
}