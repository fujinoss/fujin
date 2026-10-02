package oss.fujin.bridge

object NativeBridge {
    private var loaded = false

    fun ensureLoaded(): Boolean {
        if (loaded) return true
        return try {
            System.loadLibrary("fujin")
            loaded = true
            true
        } catch (e: UnsatisfiedLinkError) {
            false
        }
    }
}
