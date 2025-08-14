function update_forms() {
    console.log("yes");
    document.getElementById("otp-submit").disabled =
        !document.getElementById("input-otp").value;
}

window.addEventListener("load", function () {
    console.log("hm");
    document
        .getElementById("input-otp")
        .addEventListener("input", update_forms);
    update_forms();
});
