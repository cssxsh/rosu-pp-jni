@file:JvmName("BeatmapExtension")

package xyz.cssxsh.rosu.beatmap

import xyz.cssxsh.rosu.*
import java.nio.ByteBuffer

public fun Beatmap.stars(): AnyStars {
    return AnyStars(map = this)
}

public fun Beatmap.maxPP(mods: Long): PerformanceAttributes {
    return AnyPP(map = this).mods(flag = mods).calculate()
}

public fun Beatmap.pp(): AnyPP {
    return AnyPP(map = this)
}

public fun Beatmap.strains(mods: Long): Strains {
    return AnyStars(map = this).mods(flag = mods).strains()
}

public fun Beatmap.gradualDifficulty(mods: Long): GradualDifficultyAttributes {
    return GradualDifficultyAttributes(map = this, mods = mods)
}

public fun Beatmap.gradualPerformance(mods: Long): GradualPerformanceAttributes {
    return GradualPerformanceAttributes(map = this, mods = mods)
}

public fun osuHitObjects(mods: Long): List<Any> = TODO("Serialization is required, otherwise it is too complex")

public fun taikoHitObjects(): List<Any> = TODO("Serialization is required, otherwise it is too complex")

public fun catchHitObjects(mods: Long): List<Any> = TODO("Serialization is required, otherwise it is too complex")

public fun maniaHitObjects(): List<Any> = TODO("Serialization is required, otherwise it is too complex")

@Suppress("NOTHING_TO_INLINE")
internal inline fun ByteBuffer.toByteArray(): ByteArray {
    val dest = ByteArray(remaining())
    get(dest)
    return dest
}