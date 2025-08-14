"use strict";

window.addEventListener("load", function () {
    // https://stackoverflow.com/a/64029534
    // converts the form request into a form parseable by axum_typed_multipart
    let forms = document.getElementsByClassName("normalize-multipart");
    for (let form of forms) {
        form.addEventListener("formdata", function (event) {
            // this is necessary to have TryFromMultipart interpret Option types as None instead of Some("")

            let formData = event.formData;
            for (let [name, value] of Array.from(formData.entries())) {
                if (value === "") {
                    formData.delete(name);
                } else if (value.size === 0) {
                    // handle files
                    formData.delete(name);
                }
            }

            for (let checkbox of form.querySelectorAll(
                "input[type=checkbox]"
            )) {
                formData.delete(checkbox.name);
                if (checkbox.checked) {
                    formData.append(checkbox.name, "true");
                } else {
                    formData.append(checkbox.name, "false");
                }
            }
        });
    }

    for (let item of document.getElementsByClassName("editable-data")) {
        item.classList.remove("editing");
    }

    let editButtons = document.getElementsByClassName("edit-button");
    for (let editButton of editButtons) {
        editButton.addEventListener("click", function (event) {
            this.closest(".editable-data").classList.add("editing");
        });
    }

    let cancelButtons = document.getElementsByClassName("cancel-edit");
    for (let cancelButton of cancelButtons) {
        cancelButton.addEventListener("click", function (event) {
            this.closest(".editable-data").classList.remove("editing");
        });
    }

    const searchParams = new URLSearchParams(window.location.search);
    for (let paramInput of document.querySelectorAll("input.get-parameter")) {
        if (searchParams.has(paramInput.name)) {
            if (paramInput.name === "redirect") {
                let relativeUrl =
                    new URL(document.baseURI).origin ===
                    new URL(searchParams[paramInput.name], document.baseURI)
                        .origin;
                if (!relativeUrl) {
                    continue;
                }
            }
            paramInput.value = searchParams.get(paramInput.name);
        }
    }

    for (let paramInput of document.querySelectorAll("input.get-session")) {
        paramInput.value = this.window.sessionStorage.getItem(paramInput.name);
        if (paramInput.name === "email" && !paramInput.value) {
            location.href = "/sign-in";
        }
    }
});
