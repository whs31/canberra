import QtQuick
import QtQuick.Controls
import QtQuick.Controls.FluentWinUI3
import QtQuick.Layouts

Dialog {
    id: dialog

    title: qsTr("О программе")
    modal: true
    closePolicy: Popup.CloseOnPressOutside
    standardButtons: Dialog.Ok
    onAccepted: this.close()
}