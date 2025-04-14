package xyz.theunknowngroup.app.scenes

import javafx.geometry.Insets
import javafx.geometry.Pos
import javafx.scene.Scene
import javafx.scene.control.Button
import javafx.scene.layout.GridPane
import javafx.stage.Stage
import xyz.theunknowngroup.app.App.Companion.w
import xyz.theunknowngroup.app.App.Companion.h
import xyz.theunknowngroup.images.Images.S.back

fun startPage(stage: Stage): Scene {
    // Components
    val switchBtn = Button("Switch").apply {
        id = "switch"
        setOnAction { logIn(stage) }
    }

    // Layout
    val layout = GridPane().apply {
        // Children
        add(switchBtn, 0, 0)
        // Config
        alignment = Pos.CENTER
        hgap = 10.0
        vgap = 2.0
        padding = Insets(20.0)

        style = """
            -fx-background-image: url('$back');
            -fx-background-repeat: no-repeat;
        """.trimIndent()
    }
    // Setting the scene
    val scene = Scene(layout, w, h).apply {
        stylesheets.add(javaClass.getResource("/main.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene;
}