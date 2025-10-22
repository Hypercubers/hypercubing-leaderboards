"use strict";

window.addEventListener("load", function () {
    for (let elem of this.document.getElementsByClassName("edit-user")) {
        elem.addEventListener("click", (event) => {
            let dataset = event.target.closest("a").dataset;
            this.document.getElementById("user_details").open = true;
            this.document.getElementById("user_id").value = dataset.id;
            this.document.getElementById("user_name").value = dataset.name;
            this.document.getElementById("user_email").value = dataset.email;
            this.document.getElementById("user_discord").value =
                dataset.discord;
            this.document.getElementById("user_moderator_notes").value =
                dataset.moderatorNotes;
            this.document.getElementById("user_moderator").checked =
                dataset.moderator == "true";
            this.document.getElementById("user_dummy").checked =
                dataset.dummy == "true";
        });
    }
});
