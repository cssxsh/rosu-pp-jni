//! In addtion to not considering the positional offset caused by stack leniency, slider paths are also ignored.
//! This means stacked positions aswell as the travel distance of notes is completely omitted which
//! will cause notable inaccuracies.
//! The advantage is that it's considerably faster than `osu_precise`.

#![cfg(feature = "osu_fast")]

use std::mem;

use self::osu_object::ObjectParameters;

use super::DifficultyAttributes;

mod difficulty_object;
mod osu_object;
mod skill;
mod skill_kind;
mod slider_state;

use difficulty_object::DifficultyObject;
use osu_object::OsuObject;
use skill::Skill;
use skill_kind::SkillKind;
use slider_state::SliderState;

use crate::{Beatmap, Mods, Strains};

const OBJECT_RADIUS: f32 = 64.0;
const SECTION_LEN: f32 = 400.0;
const DIFFICULTY_MULTIPLIER: f32 = 0.0675;
const NORMALIZED_RADIUS: f32 = 52.0;

/// Star calculation for osu!standard maps.
///
/// Sliders are considered as regular hitcircles and stack leniency is ignored.
/// Still decently accurate results but definitely less precise than `osu_precise`.
/// However, this version is considerably faster.
///
/// In case of a partial play, e.g. a fail, one can specify the amount of passed objects.
pub fn stars(
    map: &Beatmap,
    mods: impl Mods,
    passed_objects: Option<usize>,
) -> DifficultyAttributes {
    let take = passed_objects.unwrap_or_else(|| map.hit_objects.len());

    let attributes = map.attributes().mods(mods);
    let hit_window = super::difficulty_range_od(attributes.od) / attributes.clock_rate;
    let od = (80.0 - hit_window) / 6.0;

    if take < 2 {
        return DifficultyAttributes {
            ar: attributes.ar,
            hp: attributes.hp,
            od,
            ..Default::default()
        };
    }

    let radius = OBJECT_RADIUS * (1.0 - 0.7 * (attributes.cs - 5.0) / 5.0) / 2.0;
    let mut scaling_factor = NORMALIZED_RADIUS / radius;

    if radius < 30.0 {
        let small_circle_bonus = (30.0 - radius).min(5.0) / 50.0;
        scaling_factor *= 1.0 + small_circle_bonus;
    }

    let mut params = ObjectParameters {
        map,
        radius,
        clock_rate: attributes.clock_rate,
        max_combo: 0,
        slider_state: SliderState::new(map),
    };

    let mut hit_objects = map
        .hit_objects
        .iter()
        .take(take)
        .filter_map(|h| OsuObject::new(h, &mut params));

    let fl = mods.fl();
    let mut skills = Vec::with_capacity(2 + fl as usize);

    skills.push(Skill::new(SkillKind::Aim));
    skills.push(Skill::new(SkillKind::speed(hit_window)));

    if fl {
        skills.push(Skill::new(SkillKind::flashlight(scaling_factor)));
    }

    let mut prev_prev = None;
    let mut prev = hit_objects.next().unwrap();
    let mut prev_vals = None;

    // First object has no predecessor and thus no strain, handle distinctly
    let mut current_section_end = (prev.time / SECTION_LEN).ceil() * SECTION_LEN;

    // Handle second object separately to remove later if-branching
    let curr = hit_objects.next().unwrap();
    let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

    while h.base.time > current_section_end {
        for skill in skills.iter_mut() {
            skill.start_new_section_from(current_section_end);
        }

        current_section_end += SECTION_LEN;
    }

    for skill in skills.iter_mut() {
        skill.process(&h);
    }

    prev_prev = Some(prev);
    prev_vals = Some((h.jump_dist, h.strain_time));
    prev = curr;

    // Handle all other objects
    for curr in hit_objects {
        let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

        while h.base.time > current_section_end {
            for skill in skills.iter_mut() {
                skill.save_current_peak();
                skill.start_new_section_from(current_section_end);
            }

            current_section_end += SECTION_LEN;
        }

        for skill in skills.iter_mut() {
            skill.process(&h);
        }

        prev_prev = Some(prev);
        prev_vals = Some((h.jump_dist, h.strain_time));
        prev = curr;
    }

    for skill in skills.iter_mut() {
        skill.save_current_peak();
    }

    let aim_rating = skills[0].difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER;

    let speed_rating = if mods.rx() {
        0.0
    } else {
        skills[1].difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER
    };

    let flashlight_rating = skills.get_mut(2).map_or(0.0, |skill| {
        skill.difficulty_value().sqrt() * DIFFICULTY_MULTIPLIER
    });

    let base_aim_performance = {
        let base = 5.0 * (aim_rating / 0.0675).max(1.0) - 4.0;

        base * base * base / 100_000.0
    };

    let base_speed_performance = {
        let base = 5.0 * (speed_rating / 0.0675).max(1.0) - 4.0;

        base * base * base / 100_000.0
    };

    let base_flashlight_performance = if fl {
        flashlight_rating * flashlight_rating * 25.0
    } else {
        0.0
    };

    let base_performance = (base_aim_performance.powf(1.1)
        + base_speed_performance.powf(1.1)
        + base_flashlight_performance.powf(1.1))
    .powf(1.0 / 1.1);

    let star_rating = if base_performance > 0.00001 {
        1.12_f32.cbrt()
            * 0.027
            * ((100_000.0 / (1.0_f32 / 1.1).exp2() * base_performance).cbrt() + 4.0)
    } else {
        0.0
    };

    DifficultyAttributes {
        stars: star_rating,
        ar: attributes.ar,
        hp: attributes.hp,
        od,
        speed_strain: speed_rating,
        aim_strain: aim_rating,
        flashlight_rating,
        max_combo: params.max_combo,
        n_circles: map.n_circles as usize,
        n_spinners: map.n_spinners as usize,
        n_sliders: map.n_sliders as usize,
    }
}

/// Essentially the same as the `stars` function but instead of
/// evaluating the final strains, it just returns them as is.
///
/// Suitable to plot the difficulty of a map over time.
pub fn strains(map: &Beatmap, mods: impl Mods) -> Strains {
    let attributes = map.attributes().mods(mods);
    let hit_window = super::difficulty_range_od(attributes.od) / attributes.clock_rate;

    if map.hit_objects.len() < 2 {
        return Strains::default();
    }

    let radius = OBJECT_RADIUS * (1.0 - 0.7 * (attributes.cs - 5.0) / 5.0) / 2.0;
    let mut scaling_factor = NORMALIZED_RADIUS / radius;

    if radius < 30.0 {
        let small_circle_bonus = (30.0 - radius).min(5.0) / 50.0;
        scaling_factor *= 1.0 + small_circle_bonus;
    }

    let mut params = ObjectParameters {
        map,
        radius,
        clock_rate: attributes.clock_rate,
        max_combo: 0,
        slider_state: SliderState::new(map),
    };

    let mut hit_objects = map
        .hit_objects
        .iter()
        .filter_map(|h| OsuObject::new(h, &mut params));

    let fl = mods.fl();
    let mut skills = Vec::with_capacity(2 + fl as usize);

    skills.push(Skill::new(SkillKind::Aim));
    skills.push(Skill::new(SkillKind::speed(hit_window)));

    if fl {
        skills.push(Skill::new(SkillKind::flashlight(scaling_factor)));
    }

    let mut prev_prev = None;
    let mut prev = hit_objects.next().unwrap();
    let mut prev_vals = None;

    // First object has no predecessor and thus no strain, handle distinctly
    let mut current_section_end = (prev.time / SECTION_LEN).ceil() * SECTION_LEN;

    // Handle second object separately to remove later if-branching
    let curr = hit_objects.next().unwrap();
    let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

    while h.base.time > current_section_end {
        for skill in skills.iter_mut() {
            skill.start_new_section_from(current_section_end);
        }

        current_section_end += SECTION_LEN;
    }

    for skill in skills.iter_mut() {
        skill.process(&h);
    }

    prev_prev = Some(prev);
    prev_vals = Some((h.jump_dist, h.strain_time));
    prev = curr;

    // Handle all other objects
    for curr in hit_objects {
        let h = DifficultyObject::new(&curr, &prev, prev_vals, prev_prev, scaling_factor);

        while h.base.time > current_section_end {
            for skill in skills.iter_mut() {
                skill.save_current_peak();
                skill.start_new_section_from(current_section_end);
            }

            current_section_end += SECTION_LEN;
        }

        for skill in skills.iter_mut() {
            skill.process(&h);
        }

        prev_prev = Some(prev);
        prev_vals = Some((h.jump_dist, h.strain_time));
        prev = curr;
    }

    for skill in skills.iter_mut() {
        skill.save_current_peak();
    }

    let mut speed_strains = skills.pop().unwrap().strain_peaks;
    let mut aim_strains = skills.pop().unwrap().strain_peaks;

    let strains = if let Some(mut flashlight_strains) = skills.pop().map(|s| s.strain_peaks) {
        mem::swap(&mut speed_strains, &mut aim_strains);
        mem::swap(&mut aim_strains, &mut flashlight_strains);

        aim_strains
            .into_iter()
            .zip(speed_strains)
            .zip(flashlight_strains)
            .map(|((aim, speed), flashlight)| aim + speed + flashlight)
            .collect()
    } else {
        aim_strains
            .into_iter()
            .zip(speed_strains)
            .map(|(aim, speed)| aim + speed)
            .collect()
    };

    Strains {
        section_length: SECTION_LEN,
        strains,
    }
}
