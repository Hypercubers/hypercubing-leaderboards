
// https://stackoverflow.com/a/64029534
// converts the form request into a form parseable by axum_typed_multipart
window.addEventListener('load', function() {
    let checkboxes = document.querySelectorAll('input[type=checkbox].expand-subcategories');
    for (let checkbox of checkboxes) {
        checkbox.checked = false;
        checkbox.addEventListener('change', function(event) {
            this.closest('tbody').classList.toggle('hide-subcategories', !this.checked);
        });
    }
});