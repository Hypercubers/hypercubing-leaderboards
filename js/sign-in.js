function set_turnstile_response(response) {
    document.getElementById("input-discord-turnstile").value = response;
    document.getElementById("input-email-turnstile").value = response;
    update_forms();
}

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

function submit_discord() {
    document.getElementById("discord-form").hidden = true;
    document.getElementById("discord-form-waiting").hidden = false;
}

function submit_request_otp() {
    const redirect = document.getElementById("input-redirect").value;
    sessionStorage.setItem("redirect", redirect);
    const email = document.getElementById("input-email-address").value;
    sessionStorage.setItem("email", email);
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
