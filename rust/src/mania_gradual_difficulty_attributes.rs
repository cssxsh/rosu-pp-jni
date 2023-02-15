use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::mania::*;

/*
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    create_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap, mods: jlong
) -> &'jvm mut ManiaGradualDifficultyAttributes<'jvm> {
    let gradual = ManiaGradualDifficultyAttributes::new(map, mods as _);

    Box::leak(Box::new(gradual))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm ManiaGradualDifficultyAttributes,
) -> &'jvm mut ManiaGradualDifficultyAttributes<'jvm> {
    let clone = gradual.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, gradual: *mut ManiaGradualDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(gradual) })
}
/*
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm ManiaGradualDifficultyAttributes, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    next_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_next_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm mut ManiaGradualDifficultyAttributes,
) -> *mut ManiaDifficultyAttributes {
    let difficulty = gradual.next();
    match difficulty {
        None => 0 as _,
        Some(a) => {
            Box::into_raw(Box::new(a))
        }
    }
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes
 * Method:    hasNext_00024rosu_pp_jni
 * Signature: (J)Z
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaGradualDifficultyAttributes_hasNext_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm ManiaGradualDifficultyAttributes,
) -> jboolean {
    gradual.len() as _
}