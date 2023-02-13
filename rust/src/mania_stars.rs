use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::mania::*;

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut ManiaStars<'jvm> {
    let pp = ManiaStars::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm ManiaStars<'jvm>,
) -> &'jvm mut ManiaStars<'jvm> {
    let clone = stars.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: *mut ManiaStars<'jvm>,
) {
    drop(unsafe { Box::from_raw(stars) })
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm ManiaStars<'jvm>, pretty: jboolean,
) -> jstring {
    let info = if pretty != 0 {
        format!("{stars:#?}")
    } else {
        format!("{stars:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm ManiaStars<'jvm>,
) -> &'jvm mut ManiaDifficultyAttributes {
    let attributes = stars.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    strains_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_strains_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm ManiaStars<'jvm>,
) -> &'jvm mut ManiaStrains {
    let strains = stars.clone().strains();

    Box::leak(Box::new(strains))
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut ManiaStars<'jvm>, mods: jlong,
) {
    *stars = stars.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut ManiaStars<'jvm>, number: jlong,
) {
    *stars = stars.clone().passed_objects(number as _);
}

/*
 * Class:     xyz_cssxsh_rosu_ManiaStars
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_ManiaStars_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut ManiaStars<'jvm>, value: jdouble,
) {
    *stars = stars.clone().clock_rate(value as _);
}