use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::catch::*;

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualPerformanceAttributes
 * Method:    create_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualPerformanceAttributes_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap, mods: jlong
) -> &'jvm mut CatchGradualPerformanceAttributes<'jvm> {
    let gradual = CatchGradualPerformanceAttributes::new(map, mods as _);

    Box::leak(Box::new(gradual))
}

/*
 * Class:     xyz_cssxsh_rosu_CatchGradualPerformanceAttributes
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualPerformanceAttributes_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: *mut CatchGradualPerformanceAttributes<'jvm>,
) {
    drop(unsafe { Box::from_raw(gradual) })
}
/*
 * Class:     xyz_cssxsh_rosu_CatchGradualPerformanceAttributes
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualPerformanceAttributes_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm CatchGradualPerformanceAttributes<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_CatchGradualPerformanceAttributes
 * Method:    next_00024rosu_pp_jni
 * Signature: (JJI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_CatchGradualPerformanceAttributes_next_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, gradual: &'jvm mut CatchGradualPerformanceAttributes<'jvm>, state: &'jvm ScoreState, number: jint
) -> *mut CatchPerformanceAttributes {
    let clone = state.clone();
    let attributes = gradual.process_next_n_objects(clone.into(), number as _);
    match attributes {
        None => 0 as _,
        Some(a) => {
            Box::into_raw(Box::new(a))
        }
    }
}