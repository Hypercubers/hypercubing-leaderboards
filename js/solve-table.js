"use strict";

const isEmpty = (x) => x === undefined || x === null || x === "";

const solvesTableEndpoint = document.currentScript.dataset.solvesTableEndpoint;

var url;
function updateUrl() {
    url = new URL(window.location.href);
}

function updateParam(e) {
    const param_name = e.target.dataset.filter;
    if (param_name !== undefined) {
        const new_value = e.target.dataset.filterValue;
        if (isEmpty(new_value)) {
            url.searchParams.delete(param_name);
        } else {
            url.searchParams.set(param_name, new_value);
        }
    }

    const param_name2 = e.target.dataset.filter2;
    if (param_name2 !== undefined) {
        const new_value2 = e.target.dataset.filterValue2;
        if (isEmpty(new_value2)) {
            url.searchParams.delete(param_name2);
        } else {
            url.searchParams.set(param_name2, new_value2);
        }
    }

    sanitizeQueryParams();
    window.history.pushState(null, "", url.toString());
    handleFilterUpdate();
}

const currentEvent = () => url.searchParams.get("event");

const isFmc = () => ["fmc", "fmcca"].includes(currentEvent());
const isSpeed = () => [null, "avg", "bld", "oh"].includes(currentEvent());

const getSolveTable = () => document.getElementById("solve-table");
const getEventDropdownSummary = () => document.getElementById("filter-event");

function sanitizeQueryParams() {
    if (isFmc()) {
        url.searchParams.delete("filters");
        url.searchParams.delete("macros");
        url.searchParams.delete("variant");
        url.searchParams.delete("program");
    }
}

let xhr;

async function handleFilterUpdate() {
    updateUrl();
    const event = currentEvent();

    // Update buttons disabled state
    for (let element of document.querySelectorAll(".speed-only")) {
        element.disabled = !isSpeed();
    }
    for (let element of document.querySelectorAll(".fmc-only")) {
        element.disabled = !isFmc();
    }

    // Update buttons selected state
    let filterButtons = document.querySelectorAll("button.filter, a.filter");
    for (let btn of filterButtons) {
        btn.classList.remove("selected", "deselected");
        const shouldBeSelected =
            btn.dataset.filterValue ==
                url.searchParams.get(btn.dataset.filter) &&
            (isEmpty(btn.dataset.filter2) ||
                btn.dataset.filterValue2 ==
                    url.searchParams.get(btn.dataset.filter2));
        if (shouldBeSelected) {
            btn.classList.add("selected");
        } else {
            btn.classList.add("deselected");
        }
    }

    // Update dropdown state
    let active_event_button;
    if (event === null) {
        active_event_button = document.querySelector(`[data-filter="event"]`);
    } else {
        active_event_button = document.querySelector(
            `[data-filter="event"][data-filter-value="${event}"]`
        );
    }
    if (active_event_button !== null) {
        getEventDropdownSummary().innerHTML = active_event_button.innerHTML;
    }

    // Load solves
    if (xhr) {
        xhr.abort();
        console.log("XHR request aborted");
    }
    xhr = new XMLHttpRequest();
    const xhrUrl = solvesTableEndpoint + url.searchParams;
    xhr.addEventListener("load", () => {
        console.log("Received response");
        if (xhr.responseXML === null) {
            getSolveTable().innerHTML =
                '<p id="errorMsg">Error loading solves</p>';
            fetch(xhrUrl).then((resp) => {
                resp.text().then((text) => {
                    document.getElementById("errorMsg").innerHTML = text;
                });
            });
        } else {
            getSolveTable().replaceChildren(...xhr.responseXML.children);
            for (let elem of document.getElementsByClassName("solve-row")) {
                elem.addEventListener("click", () => {
                    location.href = elem.dataset.solveUrl;
                });
            }
            handleChart();
            
            
        }
    });
    xhr.addEventListener("error", () => {
        getSolveTable().innerHTML = "<p>Error loading solves</p>";
    });
    console.log(`Querying ${xhrUrl} ...`);
    xhr.open("GET", xhrUrl);
    xhr.responseType = "document";
    xhr.send();
}

document.addEventListener("click", (event) => {
    if (event.target.matches(".filter")) {
        if (event.target.matches("a")) {
            // Close dropdown
            $(event.target).closest(".dropdown")[0].open = undefined;
        }
        updateParam(event);
    }
    // updateChartData();
    updateChartVisibility();
});


// chart stuff


function handleChart() {
    // if the user is on the record history tab
    if (url.searchParams.get('history')) { 
        createChart();
    }
    updateChartVisibility();
}

function createChart() {
    console.log("creating chart!");
    var chartData = [];
    var ctx = document.getElementById('history-chart');

    for (let elem of document.getElementsByClassName("solve-row")) {
        console.log(elem.dataset);
        chartData.push({x: elem.dataset.solveDate, y: elem.dataset.speedCs});
    }
    console.log("chartData has: " + chartData);

    var solveData = {
        datasets: [
                {
                    borderColor: '#d47de4',
                    backgroundColor: '#d47de4',
                    label: 'Time',  
                    data: chartData
                },
            ]
    }

    new Chart(ctx, {
    type: 'line',
    data: solveData,
    options: {
    scales: {
        x: {
        type: 'time',
        time: {
            unit: 'month'
        }
            }
        },
    stepped: `after`
    }
        }
    );
}


function updateChartData() {
    data.length = 0;

    for (let elem of document.getElementsByClassName("solve-row")) {
            console.log(elem.dataset);
            data.push({x: elem.dataset.solveDate, y: elem.dataset.speedCs});
        }
    // theChart.data = data;
    // theChart.update();
    theChart.destroy();
    theChart.update();


}

function updateChartVisibility() {
// if user is showing history, show the chart
        if (url.searchParams.get('history')) {
            document.getElementById('history-chart-div').hidden = false; 
        } else {
            document.getElementById('history-chart-div').hidden = true;
        }
}

window.addEventListener("load", handleFilterUpdate);
// window.addEventListener("load", handleChart);
window.addEventListener("popstate", handleFilterUpdate);
