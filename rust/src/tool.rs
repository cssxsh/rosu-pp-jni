use std::mem::size_of;
use jni::JNIEnv;
use jni::objects::*;
use rosu_pp::*;
use rosu_pp::catch::*;
use rosu_pp::mania::*;
use rosu_pp::osu::*;
use rosu_pp::taiko::*;

#[inline]
pub fn parse_game_mode(index: i32) -> Result<GameMode, i32> {
    match index {
        x if x == GameMode::Osu as i32 => Ok(GameMode::Osu),
        x if x == GameMode::Taiko as i32 => Ok(GameMode::Taiko),
        x if x == GameMode::Catch as i32 => Ok(GameMode::Catch),
        x if x == GameMode::Mania as i32 => Ok(GameMode::Mania),
        _ => Err(index),
    }
}

#[inline]
pub fn parse_hit_result_priority(index: i32) -> Result<HitResultPriority, i32> {
    match index {
        x if x == HitResultPriority::BestCase as i32 => Ok(HitResultPriority::BestCase),
        x if x == HitResultPriority::WorstCase as i32 => Ok(HitResultPriority::WorstCase),
        _ => Err(index),
    }
}

#[inline]
pub fn parse_performance_attributes(ptr: i64, mode: GameMode) -> PerformanceAttributes {
    match mode {
        GameMode::Osu => {
            let attributes = unsafe {
                Box::from_raw(ptr as *mut OsuPerformanceAttributes)
            };
            let clone = attributes.as_ref().clone();
            Box::into_raw(attributes);
            PerformanceAttributes::Osu(clone)
        }
        GameMode::Taiko => {
            let attributes = unsafe {
                Box::from_raw(ptr as *mut TaikoPerformanceAttributes)
            };
            let clone = attributes.as_ref().clone();
            Box::into_raw(attributes);
            PerformanceAttributes::Taiko(clone)
        }
        GameMode::Catch => {
            let attributes = unsafe {
                Box::from_raw(ptr as *mut CatchPerformanceAttributes)
            };
            let clone = attributes.as_ref().clone();
            Box::into_raw(attributes);
            PerformanceAttributes::Catch(clone)
        }
        GameMode::Mania => {
            let attributes = unsafe {
                Box::from_raw(ptr as *mut ManiaPerformanceAttributes)
            };
            let clone = attributes.as_ref().clone();
            Box::into_raw(attributes);
            PerformanceAttributes::Mania(clone)
        }
    }
}

#[inline]
pub fn option_to_byte_buffer<T>(_env: JNIEnv, option: Option<T>) -> JByteBuffer {
    return match option {
        None => {
            JByteBuffer::from(JObject::null())
        }
        Some(point) => {
            let data = Box::into_raw(Box::from(point));

            let size = size_of::<T>();
            unsafe { _env.new_direct_byte_buffer(data as _, size) }
                .unwrap_or_else(|error| _env.fatal_error(error.to_string()))
        }
    };
}

#[inline]
pub fn vec_to_byte_buffer<T>(_env: JNIEnv, vec: Vec<T>) -> JByteBuffer {
    let size = vec.len() * size_of::<T>();
    let buf = vec.leak();
    unsafe { _env.new_direct_byte_buffer(buf.as_mut_ptr() as _, size) }
        .unwrap_or_else(|error| _env.fatal_error(error.to_string()))
}

#[test]
fn ojb_size() {
    use rosu_pp::beatmap::*;
    use rosu_pp::parse::*;

    assert_eq!(16, size_of::<Break>());
    assert_eq!(32, size_of::<DifficultyPoint>());
    assert_eq!(16, size_of::<EffectPoint>());
    assert_eq!(16, size_of::<TimingPoint>());

    assert_eq!(88, size_of::<HitObject>());
    assert_eq!(72, size_of::<HitObjectKind>());

    assert_eq!(16, size_of::<Option<f64>>());
    assert_eq!(8, size_of::<usize>());
    assert_eq!(24, size_of::<Vec<PathControlPoint>>());
    assert_eq!(24, size_of::<Vec<u8>>());
    assert_eq!(8, size_of::<GameMode>());
}