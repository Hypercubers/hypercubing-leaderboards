"use strict";

function update_forms() {
    document.getElementById("discord-submit").disabled = !(
        document.getElementById("input-discord-turnstile").value &&
        document.getElementById("input-discord-username").value
    );

    document.getElementById("email-submit").disabled = !(
        document.getElementById("input-email-turnstile").value &&
        document.getElementById("input-email-address").value
    );
}

function set_turnstile_response(response) {
    document.getElementById("input-discord-turnstile").value = response;
    document.getElementById("input-email-turnstile").value = response;
    update_forms();
}

window.addEventListener("load", function () {
    document
        .getElementById("input-discord-username")
        .addEventListener("input", update_forms);
    document
        .getElementById("input-email-address")
        .addEventListener("input", update_forms);
    update_forms();
});
