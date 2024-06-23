
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
    }
});