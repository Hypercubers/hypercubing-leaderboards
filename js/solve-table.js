'use strict';

const url = new URL(window.location.href);
function updateParam(e) {
    const param_name = e.target.dataset.filter;
    let new_value = e.target.dataset.filterValue;
    if (new_value === undefined) {
        url.searchParams.delete(param_name);
    } else {
        url.searchParams.set(param_name, new_value);
    }
    sanitizeQueryParams()
    window.history.pushState(null, '', url.toString());
    handleFilterUpdate()
}

const currentEvent = () => url.searchParams.get('event')

const isFmc = () => ['fmc', 'fmcca'].includes(currentEvent())
const isSpeed = () => !isFmc()

const getSolveTable = () => document.getElementById('solve-table');
const getEventDropdownSummary = () => document.getElementById('filter-event');

function sanitizeQueryParams() {
    if (isFmc()) {
        url.searchParams.delete('filters');
        url.searchParams.delete('macros');
    }
}

let xhr;

async function handleFilterUpdate() {
    const event = url.searchParams.get('event')

    // Update buttons disabled state
    for (let element of document.querySelectorAll('.speed-only')) {
        element.disabled = !isSpeed();
    }
    for (let element of document.querySelectorAll('.fmc-only')) {
        element.disabled = !isFmc();
    }

    // Update buttons selected state
    let filterButtons = document.querySelectorAll('button.filter, a.filter');
    for (let btn of filterButtons) {
        btn.classList.remove('selected', 'deselected');
        if (btn.dataset.filterValue == url.searchParams.get(btn.dataset.filter)) {
            btn.classList.add('selected');
        } else {
            btn.classList.add('deselected');
        }
    }

    // Update dropdown state
    let active_event_button
    if (event === null) {
        active_event_button = document.querySelector(`[data-filter="event"]`)
    } else {
        active_event_button = document.querySelector(`[data-filter="event"][data-filter-value="${event}"]`)
    }
    if (active_event_button !== null) {
        getEventDropdownSummary().innerHTML = active_event_button.innerHTML;
    }

    // Load solves
    if (xhr) {
        xhr.abort();
    }
    xhr = new XMLHttpRequest();
    xhr.addEventListener('loadstart', () => {
        getSolveTable().innerHTML = "<span aria-busy=true>Loading solves…</span>";
    });
    xhr.addEventListener('load', () => {
        if (xhr.responseXML === null) {
            getSolveTable().innerHTML = "<p>Error loading solves</p>";
        } else {
            getSolveTable().replaceChildren(...xhr.responseXML.children);
        }
    });
    xhr.addEventListener('error', () => {
        getSolveTable().innerHTML = "<p>Error loading solves</p>";
    });
    xhr.open('GET', 'solves-table/all?' + url.searchParams);
    xhr.responseType = 'document';
    xhr.send();
}

document.addEventListener('click', (event) => {
    if (event.target.matches('.filter')) {
        updateParam(event);
    }
});

window.addEventListener('load', handleFilterUpdate)
