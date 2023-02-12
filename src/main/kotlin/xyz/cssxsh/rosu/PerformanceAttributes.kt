package xyz.cssxsh.rosu

public sealed class PerformanceAttributes(public val mode: GameMode) : Cloneable {
    internal abstract val ptr: Long
}