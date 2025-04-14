package xyz.theunknowngroup.app.scenes

import javafx.geometry.Pos
import javafx.scene.Scene
import javafx.scene.control.Label
import javafx.scene.control.TextField
import javafx.scene.input.KeyCode
import javafx.scene.layout.GridPane
import javafx.scene.layout.Priority
import javafx.scene.layout.VBox
import javafx.stage.Stage
import xyz.theunknowngroup.app.App.Companion.h
import xyz.theunknowngroup.app.App.Companion.w
import xyz.theunknowngroup.images.Images.S.back

var key = KeyCode.ENTER

fun logIn(stage: Stage): Scene {
    // Components
    val user = Label("Username:").apply {
        style = "-fx-padding: 5 5 5 5;"
    }
    val usern = TextField()
    val pass = Label("Password:").apply {
        style = "-fx-padding: 5 5 5 5;"
    }
    val passw = TextField()
    passw.setOnKeyPressed {
        event -> if (event.code == key) {
            println(usern.text)
            println(passw.text)
            reset(usern, passw)
        }
    }

    // Layout
    val lay1 = GridPane().apply {
        add(user, 0, 0)
        add(usern, 0, 1)

        add(pass, 0, 2)
        add(passw, 0, 3)

        // config
        style = """
            -fx-padding: 20 20 20 20;
            -fx-background-color: rgba(96, 105, 51, 0.8);
            -fx-border-color: transparent;
            -fx-border-style: solid;
            -fx-background-radius: 10px;
        """.trimIndent()

        maxWidth = 200.0
        maxHeight = 200.0

        alignment = Pos.CENTER
    }

    GridPane.setHgrow(lay1, Priority.SOMETIMES)

    val layout = VBox(10.0).apply {
        children.addAll(lay1)
        alignment = Pos.CENTER

        style = """
            -fx-background-image: url('$back');
            -fx-background-repeat: no-repeat;
        """.trimIndent()
        VBox.setVgrow(lay1, Priority.SOMETIMES)
    }

    val scene = Scene(layout, w, h).apply {
        stylesheets.add(javaClass.getResource("/login.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene
}
fun reset(text: TextField, text2: TextField) {
    text.text = ""
    text2.text = ""
}