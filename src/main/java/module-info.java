module xyz.unknown.ukmcl {
    requires javafx.controls;
    requires javafx.fxml;

    requires org.controlsfx.controls;
    requires java.desktop;

    opens xyz.unknown.ukmcl to javafx.fxml;
    exports xyz.unknown.ukmcl;
}