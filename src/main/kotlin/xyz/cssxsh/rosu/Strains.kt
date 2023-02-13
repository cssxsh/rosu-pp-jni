package xyz.cssxsh.rosu

public sealed class Strains(public val mode: GameMode) : Cloneable {
    internal abstract val ptr: NativePointer
}