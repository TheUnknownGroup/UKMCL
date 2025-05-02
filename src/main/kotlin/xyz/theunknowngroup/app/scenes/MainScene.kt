package xyz.theunknowngroup.app.scenes

import javafx.geometry.Insets
import javafx.geometry.Pos
import javafx.scene.Scene
import javafx.scene.control.Button
import javafx.scene.layout.GridPane
import javafx.stage.Stage
import xyz.theunknowngroup.app.h
import xyz.theunknowngroup.app.w
import xyz.theunknowngroup.images.Images.S.back

fun startPage(stage: Stage): Scene {
    // Components
    val signInBtn = Button("Sign In").apply {
        id = "signIn"
        setOnAction { logIn(stage) }
    }
    val signUpBtn = Button("Sign Up").apply {
        id = "signUp"
        setOnAction { signUp(stage) }
    }

    // Layout
    val layout = GridPane().apply {
        // Children
        add(signInBtn, 0, 0)
        add(signUpBtn, 1, 0)
        // Config
        alignment = Pos.CENTER
        hgap = 10.0
        vgap = 2.0
        padding = Insets(20.0)
        id = "mainLay"
        style = "-fx-background-image: url('$back');"
    }
    // Setting the scene
    val scene = Scene(layout, w, h).apply {
        stylesheets.add(javaClass.getResource("/main.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene;
}