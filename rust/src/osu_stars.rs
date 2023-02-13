use jni::JNIEnv;
use jni::sys::*;
use rosu_pp::*;
use rosu_pp::osu::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    create_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_create_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, map: &'jvm Beatmap,
) -> &'jvm mut OsuStars<'jvm> {
    let pp = OsuStars::new(map);

    Box::leak(Box::new(pp))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_clone_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm OsuStars<'jvm>,
) -> &'jvm mut OsuStars<'jvm> {
    let clone = stars.clone();

    Box::leak(Box::new(clone))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_destroy_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: *mut OsuStars<'jvm>,
) {
    drop(unsafe { Box::from_raw(stars) })
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_debug_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm OsuStars<'jvm>, pretty: jboolean,
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
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    calculate_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_calculate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm OsuStars<'jvm>,
) -> &'jvm mut OsuDifficultyAttributes {
    let attributes = stars.clone().calculate();

    Box::leak(Box::new(attributes))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    strains_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_strains_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm OsuStars<'jvm>,
) -> &'jvm mut OsuStrains {
    let strains = stars.clone().strains();

    Box::leak(Box::new(strains))
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    convertMode_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_convertMode_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm OsuStars<'jvm>, index: jint,
) -> *mut u8 {
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let other = match stars.clone().mode(mode) {
        AnyStars::Osu(o) => Box::into_raw(Box::new(o)) as *mut u8,
        AnyStars::Taiko(t) => Box::into_raw(Box::new(t)) as *mut u8,
        AnyStars::Catch(c) => Box::into_raw(Box::new(c)) as *mut u8,
        AnyStars::Mania(m) => Box::into_raw(Box::new(m)) as *mut u8,
    };

    other
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    withMods_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_withMods_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut OsuStars<'jvm>, mods: jlong,
) {
    *stars = stars.clone().mods(mods as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    withPasseObjects_00024rosu_pp_jni
 * Signature: (JJ)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_withPasseObjects_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut OsuStars<'jvm>, number: jlong,
) {
    *stars = stars.clone().passed_objects(number as _);
}

/*
 * Class:     xyz_cssxsh_rosu_OsuStars
 * Method:    withClockRate_00024rosu_pp_jni
 * Signature: (JD)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_OsuStars_withClockRate_00024rosu_1pp_1jni<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, stars: &'jvm mut OsuStars<'jvm>, value: jdouble,
) {
    *stars = stars.clone().clock_rate(value as _);
}