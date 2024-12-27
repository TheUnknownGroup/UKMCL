package xyz.unknown.ukmcl;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Scene;
import javafx.scene.image.ImageView;
import javafx.scene.layout.VBox;
import javafx.stage.Stage;
import javafx.scene.image.Image;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;

public class Main extends Application {
    @Override
    public void start(Stage stage) throws IOException {
        FXMLLoader fxmlLoader = new FXMLLoader(Main.class.getResource("main-page.fxml"));
        Scene scene = new Scene(fxmlLoader.load(), 950, 530);
        InputStream url = this.getClass().getResourceAsStream("css/styles.css");
        if (url == null) {
            System.out.println("Resource not found. Aborting");
            System.exit(-1);
        }
        String css = url.toString();
        scene.getStylesheets().add(css);

        stage.setTitle("UKMCL");
        stage.setResizable(false);

//        Image icon = new Image(getClass().getResourceAsStream("128x128.png"));
        stage.getIcons().add(new Image(getClass().getResourceAsStream("assets/32x32.png")));

        VBox root = new VBox();

        stage.setScene(scene);
        stage.show();
    }

    public static void main(String[] args) {
        launch();
    }
}