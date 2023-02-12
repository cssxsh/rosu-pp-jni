use std::path::Path;
use jni::JNIEnv;
use jni::objects::*;
use jni::sys::*;
use rosu_pp::*;
use crate::tool::*;

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    create_00024rosu_pp_jni
 * Signature: ([B)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_create_00024rosu_1pp_1jni___3B(
    _env: JNIEnv, _this: jclass, byte_array: jbyteArray,
) -> *mut Beatmap {
    let bytes = _env.convert_byte_array(byte_array)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    let beatmap = Beatmap::from_bytes(bytes.as_slice())
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    Box::into_raw(Box::new(beatmap))
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    create_00024rosu_pp_jni
 * Signature: (Ljava/lang/String;)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_create_00024rosu_1pp_1jni__Ljava_lang_String_2<'jvm>(
    _env: JNIEnv<'jvm>, _this: jclass, path_str: JString<'jvm>,
) -> *mut Beatmap {
    let binding = _env.get_string(path_str)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    let str = String::from(binding);
    let path = Path::new(&str);

    let beatmap = Beatmap::from_path(path)
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()));

    Box::into_raw(Box::new(beatmap))
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    default_00024rosu_pp_jni
 * Signature: ()J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_default_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass,
) -> *mut Beatmap {
    let beatmap = Beatmap::default();

    Box::into_raw(Box::new(beatmap))
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    clone_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_clone_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> *mut Beatmap {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let other = beatmap.clone();

    Box::into_raw(beatmap);
    Box::into_raw(other)
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    destroy_00024rosu_pp_jni
 * Signature: (J)V
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_destroy_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) {
    drop(unsafe { Box::from_raw(ptr) })
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    debug_00024rosu_pp_jni
 * Signature: (JZ)Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_debug_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap, pretty: jboolean,
) -> jstring {
    let beatmap = unsafe { Box::from_raw(ptr) };

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
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getMode_00024rosu_pp_jni
 * Signature: (J)I
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getMode_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jint {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let index = beatmap.mode as jint;

    Box::into_raw(beatmap);
    index
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getVersion_00024rosu_pp_jni
 * Signature: (J)I
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getVersion_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jint {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let version = beatmap.version as jint;

    Box::into_raw(beatmap);
    version
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getNCircles_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getNCircles_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jlong {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let n_circles = beatmap.n_circles as jlong;

    Box::into_raw(beatmap);
    n_circles
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getNSliders_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getNSliders_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jlong {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let n_sliders = beatmap.n_sliders as jlong;

    Box::into_raw(beatmap);
    n_sliders
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getNSpinners_00024rosu_pp_jni
 * Signature: (J)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getNSpinners_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jlong {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let n_spinners = beatmap.n_spinners as jlong;

    Box::into_raw(beatmap);
    n_spinners
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getAR_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getAR_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jfloat {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let ar = beatmap.ar as jfloat;

    Box::into_raw(beatmap);
    ar
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getOD_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getOD_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jfloat {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let od = beatmap.od as jfloat;

    Box::into_raw(beatmap);
    od
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getCS_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getCS_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jfloat {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let cs = beatmap.cs as jfloat;

    Box::into_raw(beatmap);
    cs
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getHP_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getHP_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jfloat {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let hp = beatmap.hp as jfloat;

    Box::into_raw(beatmap);
    hp
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getSM_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getSM_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jdouble {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let sm = beatmap.slider_mult as jdouble;

    Box::into_raw(beatmap);
    sm
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getTR_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getTR_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jdouble {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let tr = beatmap.tick_rate as jdouble;

    Box::into_raw(beatmap);
    tr
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getSL_00024rosu_pp_jni
 * Signature: (J)F
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getSL_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jfloat {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let sl = beatmap.stack_leniency as jfloat;

    Box::into_raw(beatmap);
    sl
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getSounds_00024rosu_pp_jni
 * Signature: (J)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getSounds_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let points = beatmap.sounds.clone();
    let buffer = vec_to_byte_buffer(_env, points);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getBreaks_00024rosu_pp_jni
 * Signature: (J)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getBreaks_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let breaks = beatmap.breaks.clone();
    let buffer = vec_to_byte_buffer(_env, breaks);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    bpm_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_bpm_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jdouble {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let bpm = beatmap.bpm();

    Box::into_raw(beatmap);
    bpm
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getTotalBreakTime_00024rosu_pp_jni
 * Signature: (J)D
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getTotalBreakTime_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jdouble {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let time = beatmap.total_break_time();

    Box::into_raw(beatmap);
    time
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getTimingPoints_00024rosu_pp_jni
 * Signature: (J)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getTimingPoints_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let points = beatmap.timing_points.to_vec();
    let buffer = vec_to_byte_buffer(_env, points);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getTimingPointAt_00024rosu_pp_jni
 * Signature: (JD)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getTimingPointAt_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap, time: jdouble,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let point = beatmap.timing_point_at(time as _);
    let buffer = option_to_byte_buffer(_env, Some(point));

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getDifficultyPoints_00024rosu_pp_jni
 * Signature: (J)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getDifficultyPoints_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let points = beatmap.difficulty_points.to_vec();
    let buffer = vec_to_byte_buffer(_env, points);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getDifficultyPointAt_00024rosu_pp_jni
 * Signature: (JD)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getDifficultyPointAt_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap, time: jdouble,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let point = beatmap.difficulty_point_at(time as _);
    let buffer = option_to_byte_buffer(_env, point);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getEffectPoints_00024rosu_pp_jni
 * Signature: (J)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getEffectPoints_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let points = beatmap.effect_points.to_vec();
    let buffer = vec_to_byte_buffer(_env, points);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    getEffectPointAt_00024rosu_pp_jni
 * Signature: (JD)Ljava/nio/ByteBuffer;
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_getEffectPointAt_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap, time: jdouble,
) -> jobject {
    let beatmap = unsafe { Box::from_raw(ptr) };

    let point = beatmap.effect_point_at(time as _);
    let buffer = option_to_byte_buffer(_env, point);

    Box::into_raw(beatmap);
    buffer.into_raw()
}

/*
 * Class:     xyz_cssxsh_rosu_beatmap_Beatmap
 * Method:    convertMode_00024rosu_pp_jni
 * Signature: (JI)J
 */
#[no_mangle]
pub extern "system" fn Java_xyz_cssxsh_rosu_beatmap_Beatmap_convertMode_00024rosu_1pp_1jni(
    _env: JNIEnv, _this: jclass, ptr: *mut Beatmap, index: jint,
) -> *mut Beatmap {
    let beatmap = unsafe { Box::from_raw(ptr) };
    let mode = parse_game_mode(index)
        .unwrap_or_else(|index| _env.fatal_error(format!("error index: {index}")));

    let cow = beatmap.convert_mode(mode);
    let new = cow.into_owned();

    Box::into_raw(beatmap);
    Box::into_raw(Box::from(new))
}