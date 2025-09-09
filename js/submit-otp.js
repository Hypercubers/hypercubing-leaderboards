"use strict";

function update_forms() {
    document.getElementById("otp-submit").disabled =
        !document.getElementById("input-otp").value;
}

window.addEventListener("load", function () {
    document
        .getElementById("input-otp")
        .addEventListener("input", update_forms);
    update_forms();
});
