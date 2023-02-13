package xyz.cssxsh.rosu

public sealed class PerformanceAttributes(public val mode: GameMode) : Cloneable {
    internal abstract val ptr: Long

    public abstract fun pp(): Double

    public abstract fun stars(): Double

    public abstract fun maxCombo(): Long
}