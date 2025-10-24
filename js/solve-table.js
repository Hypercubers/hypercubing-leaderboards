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
    updateChartVisibility();
});


// chart stuff

function updateChartVisibility() {
// if user is showing history, show the chart
        if (url.searchParams.get('history')) {
            document.getElementById('history-chart-div').hidden = false; 
        } else {
            document.getElementById('history-chart-div').hidden = true;
        }
}

function handleChart() {
    updateChartVisibility();
    myChart.destroy();
    myChart = createChart();
}

function createChart() {
    var chartData = [];
    var ctx = document.getElementById('history-chart');

    for (let elem of document.getElementsByClassName("solve-row")) {
        console.log(elem.dataset);
        
        var formattedSolveDate = dateFns.format(elem.dataset.solveDate, 'yyyy-MM-dd');
        chartData.push({x: (formattedSolveDate), y: (elem.dataset.speedCs), solver: (elem.dataset.solverName)});
        console.log(`x: ${formattedSolveDate}, y: ${elem.dataset.speedCs}, solver: ${elem.dataset.solverName}`);
    }

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

    return new Chart(ctx, {
        type: 'line',
        data: solveData,
        
        options: {
            stepped: 'after',
            plugins: {
                tooltip: {
                    callbacks: {
                        title: function(context) {
                            return dateFns.format(context[0].parsed.x, 'yyyy-MM-dd');
                        },
                        label: function(context) {
                            let label = context.dataset.label || '';

                            if (context.parsed.y !== null) {
                                label = csToString(context.parsed.y);
                            }
                            return label;   
                        },
                        footer: function(context) {
                            const dataIndex = context[0].dataIndex;
                            const originalDataPoint = context[0].dataset.data[dataIndex];
                            const solverName = originalDataPoint.solver;
                            return [`by ${solverName}`];
                        }
                    }
                }
            },
            scales: {
                y: {
                    ticks: {
                        callback: function(value) {
                            return csToStringAxis(value);
                        }
                    }
                },
                x: {
                    type: 'time',
                    time: {
                        unit: 'month',
                        displayFormats: {
                            day: 'YYYY MM DD' // Format for displaying only month and day
                        }
                        
                    }
                }
            },
        },
    }
    );
}

// takes in a number of centiseconds, and returns a formatted string
function csToString(cs) {
    var d = new Date(0,0,0,0,0,0,cs*10);
    var cs = d.getMilliseconds()/10;
    var s = d.getSeconds();
    var m = d.getMinutes();
    var h = d.getHours();
    var label = `${h}h ${m}m ${s}.${cs}s`;
    if (h == 0) {
        label = `${m}m ${s}.${cs}s`;
    }
    if (h == 0 && m == 0) {
        label = `${s}.${cs}s`;
    } 
    return label;   
}

function csToStringAxis(cs) {
    var d = new Date(0,0,0,0,0,0,cs*10);
    var cs = d.getMilliseconds()/10;
    var s = d.getSeconds();
    var m = d.getMinutes();
    var h = d.getHours();
    var label = `${h}h ${m}m ${s}s`;
    if (h == 0) {
        label = `${m}m ${s}s`;
    }
    if (h == 0 && m == 0) {
        label = `${s}s`;
    } 
    return label; 

}

var myChart = createChart();

window.addEventListener("load", handleFilterUpdate);
window.addEventListener("popstate", handleFilterUpdate);
window.addEventListener("popstate", handleChart);
