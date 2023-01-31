use std::path::Path;
use jni::JNIEnv;
use jni::objects::*;
use jni::sys::*;
use rosu_pp::*;

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    create_00024rosu_pp_jni
 * Signature: ([B)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_create_00024rosu_1pp_1jni___3B(
    _env: JNIEnv, _this: jclass, byte_array: jbyteArray,
) -> jlong {
    let bytes = _env.convert_byte_array(byte_array)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    let beatmap = Beatmap::from_bytes(bytes.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    Box::into_raw(Box::new(beatmap)) as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    create_00024rosu_pp_jni
 * Signature: (Ljava/lang/String;)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_create_00024rosu_1pp_1jni__Ljava_lang_String_2(
    _env: JNIEnv, _this: jclass, path_str: JString,
) -> jlong {
    let binding = _env.get_string(path_str)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    let str = String::from(binding);
    let path = Path::new(&str);

    let beatmap = Beatmap::from_path(path)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    Box::into_raw(Box::new(beatmap)) as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    default_00024rosu_pp_jni
 * Signature: ()J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_default_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass,
) -> jlong {
    let beatmap = Beatmap::default();
    Box::into_raw(Box::new(beatmap)) as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_clone_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jlong {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let other = beatmap.clone();

    Box::into_raw(beatmap);
    Box::into_raw(other) as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) {
    unsafe { Box::<Beatmap>::from_raw(ptr as _) };
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    debug_00024rosu_pp_jni
 * Signature: ()Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_debug_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, pretty: jboolean
) -> jstring {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    let info = if pretty != 0 {
        format!("{beatmap:#?}")
    } else {
        format!("{beatmap:?}")
    };

    let binding = _env.new_string(info)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    Box::into_raw(beatmap);

    binding.into_raw()
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getMode_00024rosu_pp_jni
 * Signature: (J)I
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getMode_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jint {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let index = beatmap.mode as jint;

    Box::into_raw(beatmap);
    index
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setMode_00024rosu_pp_jni
 * Signature: (JI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setMode_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, index: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.mode = match index {
        x if x == GameMode::Osu as i32 => GameMode::Osu,
        x if x == GameMode::Taiko as i32 => GameMode::Taiko,
        x if x == GameMode::Catch as i32 => GameMode::Catch,
        x if x == GameMode::Mania as i32 => GameMode::Mania,
        _ => _env.fatal_error(format!("{} not is mode value", index)),
    };

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getVersion_00024rosu_pp_jni
 * Signature: (J)I
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getVersion_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jint {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let version = beatmap.version as jint;

    Box::into_raw(beatmap);
    version
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setVersion_00024rosu_pp_jni
 * Signature: (JI)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setVersion_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, version: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.version = version as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getNCircles_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getNCircles_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jlong {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let n_circles = beatmap.n_circles;

    Box::into_raw(beatmap);
    n_circles as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setNCircles_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setNCircles_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, number: jlong,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.n_circles = number as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getNSliders_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getNSliders_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jlong {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let n_sliders = beatmap.n_sliders;

    Box::into_raw(beatmap);
    n_sliders as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setNSliders_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setNSliders_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, number: jlong,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.n_sliders = number as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getNSpinners_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getNSpinners_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jlong {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let n_spinners = beatmap.n_spinners;

    Box::into_raw(beatmap);
    n_spinners as _
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setNSpinners_00024rosu_pp_jni
 * Signature: (JJ)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setNSpinners_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, number: jlong,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.n_spinners = number as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getAR_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getAR_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jfloat {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let ar = beatmap.ar as jfloat;

    Box::into_raw(beatmap);
    ar
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setAR_00024rosu_pp_jni
 * Signature: (JF)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setAR_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, ar: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.ar = ar as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getOD_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getOD_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jfloat {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let od = beatmap.od as jfloat;

    Box::into_raw(beatmap);
    od
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setOD_00024rosu_pp_jni
 * Signature: (JF)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setOD_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, od: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.od = od as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getCS_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getCS_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jfloat {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let cs = beatmap.cs as jfloat;

    Box::into_raw(beatmap);
    cs
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setCS_00024rosu_pp_jni
 * Signature: (JF)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setCS_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, cs: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.cs = cs as _;

    Box::into_raw(beatmap);
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    getHP_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_getHP_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong,
) -> jfloat {
    let beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };
    let hp = beatmap.hp as jfloat;

    Box::into_raw(beatmap);
    hp
}

/*
 * Class:     xyz_cssxsh_osu_beatmap_Beatmap__Companion
 * Method:    setHP_00024rosu_pp_jni
 * Signature: (JF)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_osu_beatmap_Beatmap_00024Companion_setHP_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: jlong, hp: jint,
) {
    let mut beatmap = unsafe { Box::<Beatmap>::from_raw(ptr as _) };

    beatmap.hp = hp as _;

    Box::into_raw(beatmap);
}