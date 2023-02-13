use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::taiko::*;

/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    create_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap, mods: jlong
) -> &'jvm mut TaikoGradualDifficultyAttributes {
    let pp = TaikoGradualDifficultyAttributes::new(map, mods as _);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm TaikoGradualDifficultyAttributes,
) -> &'jvm mut TaikoGradualDifficultyAttributes {
    let clone = gradual.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, gradual: *mut TaikoGradualDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(gradual) })
}
/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm TaikoGradualDifficultyAttributes, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{gradual:#?}")
    } else {
        format!("{gradual:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    next_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_next_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm mut TaikoGradualDifficultyAttributes,
) -> *mut TaikoDifficultyAttributes {
    let difficulty = gradual.next();
    match difficulty {
        None => 0 as _,
        Some(a) => {
            Box::into_raw(Box::new(a))
        }
    }
}

/*
 * Class:     xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes
 * Method:    hasNext_00024rosu_pp_jni
 * Signature: (J)Z
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_TaikoGradualDifficultyAttributes_hasNext_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm TaikoGradualDifficultyAttributes,
) -> jboolean {
    gradual.len() as _
}