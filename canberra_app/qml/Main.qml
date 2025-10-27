import QtQuick
import QtQuick.Controls
import QtQuick.Controls.FluentWinUI3
import QtQuick.Window

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Canberra"
    Text {
        anchors.centerIn: parent
        text: "hi from rust"
    }

    Button {
        text: "Fuck"
    }
}