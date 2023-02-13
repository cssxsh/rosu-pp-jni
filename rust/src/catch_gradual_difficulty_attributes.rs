use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::catch::*;

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    create_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap, mods: jlong
) -> &'jvm mut CatchGradualDifficultyAttributes<'jvm> {
    let pp = CatchGradualDifficultyAttributes::new(map, mods as _);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm CatchGradualDifficultyAttributes<'jvm>,
) -> &'jvm mut CatchGradualDifficultyAttributes<'jvm> {
    let clone = gradual.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, gradual: *mut CatchGradualDifficultyAttributes,
) {
    drop(unsafe { Box::from_raw(gradual) })
}
/*
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm CatchGradualDifficultyAttributes<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    next_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_next_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm mut CatchGradualDifficultyAttributes<'jvm>,
) -> *mut CatchDifficultyAttributes {
    let difficulty = gradual.next();
    match difficulty {
        None => 0 as _,
        Some(a) => {
            Box::into_raw(Box::new(a))
        }
    }
}

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualDifficultyAttributes
 * Method:    hasNext_00024rosu_pp_jni
 * Signature: (J)Z
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualDifficultyAttributes_hasNext_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm CatchGradualDifficultyAttributes<'jvm>,
) -> jboolean {
    // TODO: check next
    let mut clone = gradual.clone();
    let next = clone.next();
    match next {
        None => 0,
        Some(_) => 1
    }
}