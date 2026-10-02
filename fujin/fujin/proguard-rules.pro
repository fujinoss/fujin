-keepattributes *Annotation*
-keepattributes SourceFile,LineNumberTable
-keepattributes Signature
-keepattributes Exceptions
-keepattributes InnerClasses
-keepattributes EnclosingMethod

-keep class oss.fujin.bridge.NativeBridge { *; }
-keep class oss.fujin.bridge.ZmuxClient { *; }
-keep class oss.fujin.bridge.JniCallbacks { *; }
-keep class oss.fujin.session.SessionManager { *; }
-keep class oss.fujin.config.** { *; }
-keep class oss.fujin.theme.** { *; }

-keepclasseswithmembernames class * {
    native <methods>;
}

-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}

-keep class kotlin.Metadata { *; }
-keep class kotlin.coroutines.Continuation { *; }
-dontwarn kotlin.**

-keep class androidx.work.** { *; }
-keep class androidx.lifecycle.** { *; }

-dontwarn org.jetbrains.annotations.**

-optimizationpasses 5
-allowaccessmodification
-repackageclasses ''