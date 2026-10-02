package oss.fujin.view

import android.os.Bundle
import android.view.ViewGroup
import android.widget.FrameLayout
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import oss.fujin.term.FujinTerminal

class FujinActivity : AppCompatActivity() {

    private lateinit var terminal: FujinTerminal

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        terminal = FujinTerminal(this)

        val placeholder = TextView(this).apply {
            text = "Fujin"
            textSize = 32f
            setTextColor(0xffd8d8d8.toInt())
            gravity = android.view.Gravity.CENTER
        }

        val root = FrameLayout(this).apply {
            setBackgroundColor(0xff0e0e12.toInt())
            addView(terminal, ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT,
            ))
            addView(placeholder, ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT,
            ))
        }

        setContentView(root)
    }
}
