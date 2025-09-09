"use strict";

window.addEventListener("load", function () {
    const wholeForm = this.document.getElementById("submit_form");
    for (const tagName of ["input", "select", "textarea"]) {
        for (const e of wholeForm.getElementsByTagName(tagName)) {
            e.addEventListener("input", updateForm);
        }
    }

    const solveDate = document.getElementById("solve_date");
    solveDate.valueAsDate = new Date();

    updateForm();
});

function validateDurationInput(durationContainer) {
    const [h, m, s, cs] = getDurationInputElements(durationContainer);

    var leadingZeros = false;
    function validateUnit(input, maxDigits) {
        // Remove non-digits and then parse int
        const i = parseIntSafe(input.value);
        var newValue = "";
        if (leadingZeros) {
            newValue = i.toString().slice(-maxDigits).padStart(maxDigits, "0");
        } else if (i > 0) {
            newValue = i.toString().slice(-maxDigits);
            leadingZeros = true;
        }
        input.value = newValue;
    }

    validateUnit(h, 3);
    validateUnit(m, 2);
    validateUnit(s, 2);
    validateUnit(cs, 2);
}

function getDurationInputElements(container) {
    return Array.from(container.children).filter((e) => e.tagName == "INPUT");
}

function validatePositiveNumericInput(input) {
    const moveCountNum = parseIntSafe(input.value);
    if (moveCountNum == 0) {
        input.value = "";
    } else {
        input.value = moveCountNum;
    }
}

function parseIntSafe(s) {
    const i = parseInt(s.replace(/\D/g, ""));
    if (isNaN(i)) {
        return 0;
    }
    return i;
}

function removeFile(fileInput) {
    fileInput.value = "";
    updateForm();
}

function updateForm() {
    const puzzleId = document.getElementById("puzzle_id");

    const solveDurationLabel = document.getElementById("solve_duration_label");
    const solveDuration =
        document.getElementById("solve_duration").parentElement;
    const blind = document.getElementById("blind");
    const memoDurationFieldset = document.getElementById(
        "fieldset_memo_duration"
    );
    const memoDuration = document.getElementById("memo_duration").parentElement;
    const videoUrl = document.getElementById("video_url");

    const moveCount = document.getElementById("move_count");
    const logFile = document.getElementById("log_file");

    const submitButton = document.getElementById("submit");

    const hasPuzzle = puzzleId.value != "";

    function getTotalCs(container) {
        const [h, m, s, cs] = getDurationInputElements(container);
        var totalCs = parseIntSafe(h.value);
        totalCs *= 60;
        totalCs += parseIntSafe(m.value);
        totalCs *= 60;
        totalCs += parseIntSafe(s.value);
        totalCs *= 100;
        totalCs += parseIntSafe(cs.value);
        return totalCs;
    }

    const solveCs = getTotalCs(solveDuration);
    const memoCs = getTotalCs(memoDuration);

    const validSpeed = hasPuzzle && solveCs > 0 && URL.canParse(videoUrl.value);
    const validFmc =
        hasPuzzle && parseIntSafe(moveCount.value) > 0 && logFile.value != "";

    memoDurationFieldset.disabled = !blind.checked;
    if (blind.checked) {
        solveDurationLabel.innerHTML = "Total solve duration";
    } else {
        solveDurationLabel.innerHTML = "Solve duration";

        // Clear memo time duration
        for (const e of getDurationInputElements(memoDuration)) {
            e.value = "";
        }
    }

    submitButton.disabled = !validSpeed && !validFmc;
    if (validSpeed && validFmc) {
        submitButton.value = "Submit speedsolve + fewest moves";
    } else if (validSpeed) {
        submitButton.value = "Submit speedsolve";
    } else if (validFmc) {
        submitButton.value = "Submit fewest moves";
    } else {
        submitButton.value = "Submit solve";
    }
}
