"use strict";

window.addEventListener("load", function () {
    for (let elem of this.document.getElementsByClassName("edit-variant")) {
        elem.addEventListener("click", (event) => {
            let dataset = event.target.closest("a").dataset;
            this.document.getElementById("variant_details").open = true;
            this.document.getElementById("variant_id").value = dataset.id;
            this.document.getElementById("variant_name").value = dataset.name;
            this.document.getElementById("variant_prefix").value =
                dataset.prefix;
            this.document.getElementById("variant_suffix").value =
                dataset.suffix;
            this.document.getElementById("variant_abbr").value = dataset.abbr;
            this.document.getElementById("variant_material").checked =
                dataset.materialByDefault == "true";
            this.document.getElementById("variant_filters").checked =
                dataset.primaryFilters == "true";
            this.document.getElementById("variant_macros").checked =
                dataset.primaryMacros == "true";
        });
    }

    for (let elem of this.document.getElementsByClassName("edit-program")) {
        elem.addEventListener("click", (event) => {
            let dataset = event.target.closest("a").dataset;
            this.document.getElementById("program_details").open = true;
            this.document.getElementById("program_id").value = dataset.id;
            this.document.getElementById("program_name").value = dataset.name;
            this.document.getElementById("program_abbr").value = dataset.abbr;
            this.document.getElementById("program_material").checked =
                dataset.material == "true";
        });
    }

    for (let elem of this.document.getElementsByClassName("edit-puzzle")) {
        elem.addEventListener("click", (event) => {
            let dataset = event.target.closest("a").dataset;
            this.document.getElementById("puzzle_details").open = true;
            this.document.getElementById("puzzle_id").value = dataset.id;
            this.document.getElementById("puzzle_name").value = dataset.name;
            this.document.getElementById("puzzle_filters").checked =
                dataset.primaryFilters == "true";
            this.document.getElementById("puzzle_macros").checked =
                dataset.primaryMacros == "true";
        });
    }
});
