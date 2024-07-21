
// https://stackoverflow.com/a/64029534
// converts the form request into a form parseable by axum_typed_multipart
window.addEventListener('load', function() {
    let forms = document.getElementsByClassName('normalize-multipart');
    for (let form of forms) {
        form.addEventListener('formdata', function(event) {
            let formData = event.formData;
            for (let [name, value] of Array.from(formData.entries())) {
                console.log(name, value, form.querySelector(`[name=${name}]`).type)
                if (value === ''){
                    formData.delete(name);
                }
            }

            for (let checkbox of form.querySelectorAll('input[type=checkbox]')){
                console.log("AAAAAAAAAAAAA",checkbox)
                formData.delete(checkbox.name);
                if (checkbox.checked){
                    formData.append(checkbox.name, 'true');
                } else {
                    formData.append(checkbox.name, 'false');
                }
            }
        });

        if (form.classList.contains("editable-data")){
            form.classList.remove("edit-data")
        }
    }

    let editButtons = document.getElementsByClassName('edit-button');
    for (let editButton of editButtons) {
        editButton.addEventListener('click', function(event) {
            this.closest("td").classList.add("edit-td");
        });
    }

    let cancelButtons = document.getElementsByClassName('cancel-edit');
    for (let cancelButton of cancelButtons) {
        cancelButton.addEventListener('click', function(event) {
            this.closest("td").classList.remove("edit-td");
        });
    }
});